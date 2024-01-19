use crate::helpers::hashLeftRight;
extern crate alloc;
use alloc::{vec, vec::Vec, boxed::Box};

#[derive(Debug, Clone, PartialEq)]
pub enum MerkleNodeEnum{
    MerkleNode(Box<MerkleNode>),
    DummyLeaf
}

#[derive(Debug, Clone, PartialEq)]
pub struct MerkleNode{
    pub data: Vec<u8>,
    pub left: Option<MerkleNodeEnum>,
    pub right: Option<MerkleNodeEnum>
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MerkleTree{
    pub root: Option<MerkleNode>,
}

impl MerkleTree{
    pub fn build(&mut self, leafs: Vec<Vec<u8>>){
        let mut current_level: Vec<MerkleNode> = leafs
            .iter()
            .map(|leaf| MerkleNode{data: leaf.clone(), left: None, right: None})
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
            while !current_level.is_empty(){
                let left: MerkleNode = current_level.pop().unwrap();
                let right: MerkleNode = current_level.pop().unwrap();
                next_level.push(
                    MerkleNode{
                        data: hashLeftRight(left.clone().data, right.clone().data),
                        left: Some(MerkleNodeEnum::MerkleNode(Box::new(left))),
                        right: Some(MerkleNodeEnum::MerkleNode(Box::new(right)))
                    }
                );
            };
            current_level = next_level.clone();
        }
        self.root = current_level.pop();
        debug_assert!(current_level.is_empty());
    }
    pub fn discover_sibling(&self, parent: &MerkleNode, target: &Vec<u8>) -> Option<(Option<MerkleNodeEnum>, u8)>{
        if let Some(ref left) = parent.left{
            match parent.left.as_ref().unwrap(){
                MerkleNodeEnum::MerkleNode(leaf) => {
                    if &leaf.data == target{
                        return Some((Some(parent.right.clone().unwrap()), 0));
                    }
                    else{
                        return Some((Some(parent.left.clone().unwrap()), 1));
                    }
                },
                MerkleNodeEnum::DummyLeaf => {
                    unreachable!("This should never happen!");                    
                }
            }
        }
        else{
            return None;
        }
    }
    pub fn discover_parent(&self, node: &MerkleNode, target: &Vec<u8>) -> Option<MerkleNode> {
        if let Some(MerkleNodeEnum::MerkleNode(ref left_node)) = node.left {
            if left_node.data == *target {
                return Some(node.clone());
            }
        }
        if let Some(MerkleNodeEnum::MerkleNode(ref right_node)) = node.right {
            if right_node.data == *target {
                return Some(node.clone());
            }
        }
        if let Some(MerkleNodeEnum::MerkleNode(ref left_node)) = node.left {
            if let Some(parent) = self.discover_parent(left_node, target) {
                return Some(parent);
            }
        }
        if let Some(MerkleNodeEnum::MerkleNode(ref right_node)) = node.right {
            if let Some(parent) = self.discover_parent(right_node, target) {
                return Some(parent);
            }
        }
        None
    }
}

#[test]
fn merkle_proof(){
    let mut leafs: Vec<Vec<u8>> = Vec::new();
    for i in 0..255{
        leafs.push(vec![0,0,i]);
    };
    let mut tree: MerkleTree = MerkleTree{
        root: None
    };
    tree.build(leafs);
    let mut target_leaf: Vec<u8> = vec![0,0,0];
    let root: Vec<u8> = tree.clone().root.unwrap().data;
    let mut path: Vec<(Vec<u8>, u8)> = Vec::new();
    let mut target: Vec<u8> = target_leaf.clone();
    let mut target_parent: MerkleNode = MerkleTree::default().discover_parent(&tree.clone().root.unwrap(), &target).unwrap();
    while &target != &root{
        let target_sibling: (Option<MerkleNodeEnum>, u8) = MerkleTree::default().discover_sibling(&target_parent, &target).unwrap();
        let target_sibling_node = target_sibling.0.unwrap();
        let target_sibling_lr: u8 = target_sibling.1;
        let unwrapped_node: Box<MerkleNode> = match target_sibling_node{
            MerkleNodeEnum::MerkleNode(node) => {node},
            MerkleNodeEnum::DummyLeaf => unreachable!("This should never happen!")
        };
        path.push((unwrapped_node.data, target_sibling_lr));
        target = target_parent.clone().data;
        if &target != &root{
            target_parent = MerkleTree::default().discover_parent(&tree.clone().root.unwrap(), &target).unwrap();
        };
    }
    path.reverse();
    let mut current_hash = target_leaf.clone();
    while !path.is_empty(){
        let sibling: (Vec<u8>, u8) = path.pop().unwrap();
        if sibling.1 == 0{
            current_hash = hashLeftRight(current_hash, sibling.0);
        }
        else if sibling.1 == 1{
            current_hash = hashLeftRight(sibling.0, current_hash);
        }
    }
    assert_eq!(&current_hash, &root);
}