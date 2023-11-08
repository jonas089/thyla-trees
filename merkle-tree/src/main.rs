use std::{collections::HashMap};
use uint::construct_uint;
mod deprecated;
mod helpers;
use helpers::{hash_bytes, hashLeftRight};
mod config;
use config::DEFAULT_DEPTH;

fn main(){
    panic!("Run cargo test -- --nocapture instead!");
}

#[derive(Debug, Clone)]
struct MerkleNode{
    data: Vec<u8>,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>
}

#[derive(Debug, Clone)]
struct MerkleTree{
    root: Option<MerkleNode>,
    depth: u32
}

impl MerkleTree{
    // not optimized, but sufficient for circuit research with a small set of transactions.
    fn build(&mut self, transactions: Vec<Vec<u8>>){
        // zero val for merkle roots
        let size = 2_u32.pow(self.depth-1_u32);
        let zero_val: Vec<u8> = hash_bytes(vec![0]);
        let mut zero_levels: Vec<Vec<u8>> = Vec::new();
        let mut current_level = zero_val.clone();
        zero_levels.push(zero_val.clone());
        for level in 0..self.depth - 2{
            let _hash = hashLeftRight(current_level.clone(), current_level.clone());
            zero_levels.push(_hash.clone());
            current_level = _hash;
        };
        let mut levels: Vec<Vec<MerkleNode>> = Vec::new();
        let mut bottom_level: Vec<MerkleNode> = Vec::new();
        for tx in transactions{
            bottom_level.push(MerkleNode { 
                data: tx, 
                left: None, 
                right: None })
        };

        while bottom_level.len() < size as usize{
            bottom_level.push(MerkleNode { data: 
                zero_val.clone(), 
                left: None, 
                right: None });
        }

        let mut current_level = bottom_level.clone();
        // start at first hash (one level above tx data)
        let mut current_level_height = 1;
        while current_level.len() > 1{
            while current_level.len() % 2 != 0{
                current_level.push(MerkleNode { data: zero_levels[current_level_height].clone(), left: None, right: None });
            }
            let mut new_level: Vec<MerkleNode> = Vec::new();
            for i in (0..current_level.len()).step_by(2){
                let left: MerkleNode = current_level[i].clone();
                let right: MerkleNode = current_level[i+1].clone();

                /*
                    Don't hash if L === R, instead push zero_level node.
                    * create nodes for each level
                    * push the level node 
                */
                new_level.push(MerkleNode { 
                    data: hashLeftRight(left.clone().data, right.clone().data), 
                    left: Some(Box::new(left.clone())), 
                    right: Some(Box::new(right.clone()))}
                );
            };
            levels.push(new_level.clone());
            current_level = new_level.clone();
            current_level_height += 1;
        };
        self.root = Some(levels.pop().unwrap()[0].clone());

        // return here
    }
    // find the sibling of a leaf -> takes parent as input
    fn find_leaf_sibling(&self, parent: MerkleNode, target: Vec<u8>) -> Option<MerkleNode>{
        if let Some(ref left) = parent.left{
            if parent.clone().left.unwrap().data == target{
                return Some(*parent.clone().right.unwrap());
            }
            else{
                return Some(*parent.clone().left.unwrap());
            }
        }
        else{
            return None;
        }
    }
    // find the parent for a leaf in the tree
    fn find_leaf_parent(&self, root: MerkleNode, target: Vec<u8>) -> Option<MerkleNode>{
        // check if target in children
        if let Some(ref left) = root.left{
            if root.clone().left.unwrap().data == target{
                return Some(root);
            }
        }
        if let Some(ref right) = root.right{
            if root.clone().right.unwrap().data == target{
                return Some(root);
            }
        }
        /*
        if root.clone().left.unwrap().data == target || root.clone().right.unwrap().data == target{
            // return node with child
            return Some(root);
        };
        */
        if let Some(ref left) = root.left{
            let left_node = self.find_leaf_parent(*root.clone().left.unwrap(), target.clone());
            if !left_node.is_none(){
                return  left_node;
            }
        }
        if let Some(ref right) = root.right{
            let right_node = self.find_leaf_parent(*root.clone().right.unwrap(), target.clone());
            if !right_node.is_none(){
                return right_node;
            }
        }
        return None
    }
    // recursive function to find the path to a leaf
    fn find_leaf_path(&self, root: MerkleNode, target: Vec<u8>, mut path: Vec<Vec<u8>>) -> Option<Vec<Vec<u8>>>{
        if root.data == target{
            path.push(target);
            let mut proof_path: Vec<Vec<u8>> = Vec::new();
            for leaf in &path.clone()[1..path.len()]{
                //proof_path.push(leaf.clone());
                let leaf_parent = self.find_leaf_parent(self.clone().root.unwrap(), leaf.clone()).unwrap();
                let leaf_sibling = self.find_leaf_sibling(leaf_parent.clone(), leaf.clone()).unwrap();
                //println!("Leaf: {}, sibling: {:?}", leaf, leaf_sibling.data);
                proof_path.push(leaf_sibling.data);
            };
            return Some(proof_path);
        }
        let mut path_cp_left = path.clone();
        path_cp_left.push(root.data.clone());
        let mut path_cp_right = path.clone();
        path_cp_right.push(root.data.clone());
        if let Some(ref left) = root.left{
            let left_path = self.find_leaf_path(*root.clone().left.unwrap(), target.clone(), path_cp_left);
            if !left_path.is_none(){
                return left_path;
            }
        }
        if let Some(ref right) = root.right{
            let right_path = self.find_leaf_path(*root.clone().right.unwrap(), target, path_cp_right);
            if !right_path.is_none(){
                return right_path;
            }
        }
        return None;
    }
    fn merkle_path_in_order(&self, merkle_root: Vec<u8>, merkle_tree: MerkleNode, mut proof_path: Vec<Vec<u8>>) -> Vec<(Vec<u8>, bool)>{
        let mut in_order: Vec<(Vec<u8>, bool)> = Vec::new();
        let mut sibling = proof_path.pop().unwrap();
        let parent = self.find_leaf_parent(merkle_tree.clone(), sibling.clone()).unwrap();
        in_order.push((sibling.clone(), false));
        while !proof_path.is_empty() {
            sibling = proof_path.pop().unwrap();
            let parent = self.find_leaf_parent(merkle_tree.clone(), sibling.clone()).unwrap();
            if let Some(ref left) = parent.left{
                if left.clone().data == sibling{
                    // is left child
                    in_order.push((sibling.clone(), true));
                }
                else{
                    in_order.push((sibling.clone(), false));
                }
            }
        }
        in_order
    }
}


#[test]
fn build_merkle_tree(){
    let mut tree = MerkleTree{
        root: None,
        depth: DEFAULT_DEPTH
    };

    let transactions = vec![vec![1;32], vec![2;32]];
    tree.build(transactions.clone());
    let parent = tree.find_leaf_parent(tree.root.clone().unwrap(), transactions.clone()[0].clone());
    let sibling = tree.find_leaf_sibling(parent.clone().unwrap(), transactions.clone()[0].clone());
    let mut path = tree.find_leaf_path(tree.root.clone().unwrap(), transactions.clone()[0].clone(), Vec::new()).unwrap();
    // True -> is left, False -> is right
    let result = tree.merkle_path_in_order(tree.clone().root.unwrap().data, tree.clone().root.unwrap(), path);
    // compute merkle hash
    let mut current_hash: Vec<u8> = transactions.clone()[0].clone();
    for (sibling, indicator) in result.clone(){
        if indicator == false{
            current_hash = hashLeftRight(current_hash, sibling);
            let current_hash_hex: Vec<String> = current_hash.clone().iter()
            .map(|byte| format!("0x{:02x}", byte))
            .collect();
        }
        else{
            current_hash = hashLeftRight(sibling, current_hash);
        }
    }
    
    assert_eq!(current_hash, tree.clone().root.unwrap().data);
    // output merkle proof information
    println!("Transaction to prove: {:?}", transactions[0]);
    for (index, (sibling, indicator)) in result.into_iter().enumerate(){
        println!("Sibling #{:?}: {:?} : {:?}", index, sibling, indicator);
    }
    println!("Merkle root: {:?}", tree.clone().root.unwrap().data);

}
