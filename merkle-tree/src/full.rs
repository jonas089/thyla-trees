// a fully constructed binary merkle tree for kairos

use std::thread::current;

use crate::helpers::{hash_bytes, hashLeftRight};
use crate::error::MerkleTreeError;

#[derive(Debug, Clone, PartialEq)]
pub struct MerkleNode{
    pub data: Vec<u8>,
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct MerkleTree{
    pub root: Option<MerkleNode>,
}

impl MerkleTree{
    pub fn build(&mut self, leafs: Vec<Vec<u8>>){
        let mut current_level: Vec<MerkleNode> = leafs
            .iter()
            .map(|leaf| MerkleNode{data: leaf.to_owned(), left: None, right: None})
            .collect();
        current_level.reverse();
        while current_level.len() > 1{
            let mut next_level: Vec<MerkleNode> = Vec::new();
            if current_level.len() % 2 != 0{
                current_level.push(MerkleNode{
                    data: vec![3, 19, 16, 18],
                    left: None,
                    right: None
                });
            }
            while current_level.len() > 1{
                let left: MerkleNode = current_level.pop().expect("Missing left node!");
                let right: MerkleNode = current_level.pop().expect("Missing left node!");
                next_level.push(
                    MerkleNode{
                        data: hashLeftRight(left.clone().data, right.clone().data),
                        left: Some(Box::new(left)),
                        right: Some(Box::new(right))
                    }
                );
            };
            current_level = next_level.clone();
        }
        self.root = current_level.pop();
    }
    pub fn discover_sibling(&self, parent: &MerkleNode, target: &Vec<u8>) -> Option<MerkleNode>{
        if let Some(ref left) = parent.left{
            if parent.clone().left.unwrap().data == target.to_vec(){
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
    pub fn discover_parent(&self, node: &MerkleNode, target: &Vec<u8>) -> Option<MerkleNode> {
        if let Some(ref left) = node.left {
            if left.data == *target {
                return Some(node.clone());
            }
        }
        if let Some(ref right) = node.right {
            if right.data == *target {
                return Some(node.clone());
            }
        }
        if let Some(ref left) = node.left {
            if let Some(found) = self.discover_parent(left, target) {
                return Some(found);
            }
        }
        if let Some(ref right) = node.right {
            if let Some(found) = self.discover_parent(right, target) {
                return Some(found);
            }
        }
        None
    }
}

#[test]
fn binary_merkle_tree(){
    let mut leafs: Vec<Vec<u8>> = Vec::new();
    for i in 0..5{
        leafs.push(vec![0,0,i]);
    };
    let mut tree: MerkleTree = MerkleTree{
        root: None
    };
    tree.build(leafs);
    let root: Vec<u8> = tree.clone().root.unwrap().data;
    let mut path: Vec<Vec<u8>> = Vec::new();
    println!("Tree root: {:?}", &root);
    let mut target: Vec<u8> = vec![0,0,0];
    let mut target_parent: MerkleNode = tree.clone().discover_parent(&tree.clone().root.unwrap(), &target).unwrap();
    while &target_parent.data != &root{
        println!("Parent: {:?}, Root: {:?}", &target_parent.data, &root);
        let target_sibling: MerkleNode = tree.clone().discover_sibling(&target_parent, &target).unwrap();
        path.push(target_sibling.clone().data);
        target = target_parent.clone().data;
        target_parent = tree.clone().discover_parent(&tree.clone().root.unwrap(), &target_parent.clone().data).unwrap();
    }
    println!("Proof path: {:?}", &path);    
}