use crate::helpers::hashLeftRight;
extern crate alloc;
use alloc::{vec, vec::Vec, boxed::Box};
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum MerkleNodeEnum {
    MerkleNode(Box<MerkleNode>),
    DummyLeaf
}

#[derive(Debug, Clone, PartialEq)]
pub struct MerkleNode {
    pub data: Vec<u8>,
    pub left: Option<MerkleNodeEnum>,
    pub right: Option<MerkleNodeEnum>,
    pub parent: RefCell<Option<Rc<MerkleNode>>>
}

#[derive(Debug, Clone, PartialEq, Default)]
pub struct MerkleTree {
    pub root: Option<Rc<MerkleNode>>,
    pub parent_map: HashMap<Vec<u8>, Rc<MerkleNode>>
}

impl MerkleTree {
    pub fn build(&mut self, leafs: Vec<Vec<u8>>) {
        let mut current_level: Vec<MerkleNode> = leafs
            .iter()
            .map(|leaf| MerkleNode {
                data: leaf.clone(), 
                left: None, 
                right: None,
                parent: RefCell::new(None)
            })
            .collect();
        current_level.reverse();

        while current_level.len() > 1 {
            let mut next_level: Vec<MerkleNode> = Vec::new();
            if current_level.len() % 2 != 0 {
                current_level.push(MerkleNode {
                    data: vec![3, 19, 16, 18],
                    left: None,
                    right: None,
                    parent: RefCell::new(None)
                });
            }
            while !current_level.is_empty() {
                let left = Rc::new(current_level.pop().unwrap());
                let right = Rc::new(current_level.pop().unwrap());

                let new_node = MerkleNode {
                    data: hashLeftRight(left.data.clone(), right.data.clone()),
                    left: Some(MerkleNodeEnum::MerkleNode(Box::new(MerkleNode {
                        data: left.data.clone(),
                        left: left.left.clone(),
                        right: left.right.clone(),
                        parent: RefCell::new(Some(Rc::clone(&left)))
                    }))),
                    right: Some(MerkleNodeEnum::MerkleNode(Box::new(MerkleNode {
                        data: right.data.clone(),
                        left: right.left.clone(),
                        right: right.right.clone(),
                        parent: RefCell::new(Some(Rc::clone(&right)))
                    }))),
                    parent: RefCell::new(None)
                };

                *left.parent.borrow_mut() = Some(Rc::new(new_node.clone()));
                *right.parent.borrow_mut() = Some(Rc::new(new_node.clone()));
                self.parent_map.insert(left.data.clone(), Rc::new(new_node.clone()));
                self.parent_map.insert(right.data.clone(), Rc::new(new_node.clone()));
                next_level.push(new_node);
            }
            current_level = next_level;
        }

        self.root = current_level.pop().map(Rc::new);
    }

    pub fn discover_sibling(&self, parent: &MerkleNode, target: &Vec<u8>) -> Option<(Option<MerkleNodeEnum>, u8)> {
        if let Some(ref left) = parent.left {
            match left {
                MerkleNodeEnum::MerkleNode(leaf) => {
                    if &leaf.data == target {
                        return Some((parent.right.clone(), 0));
                    } else {
                        return Some((parent.left.clone(), 1));
                    }
                },
                MerkleNodeEnum::DummyLeaf => {
                    unreachable!("This should never happen!");                    
                }
            }
        }
        None
    }

    pub fn discover_parent(&self, target: &Vec<u8>) -> Option<Rc<MerkleNode>> {
        self.parent_map.get(target).cloned()
    }
}

#[test]
fn tracking_proof(){
    let mut tree: MerkleTree = MerkleTree{
        root: None,
        parent_map: HashMap::new()
    };
    let mut leafs: Vec<Vec<u8>> = Vec::new();
    for i in 0..255{
        leafs.push(vec![0,0,i]);
    };
    tree.build(leafs);
    let target_leaf = vec![0,0,0];
    let root_value = tree.clone().root.unwrap().data.clone();
    let mut path: Vec<(Vec<u8>, u8)> = Vec::new();
    let mut target = target_leaf.clone();
    let mut target_parent = tree.discover_parent(&target).unwrap();
    while &target != &root_value{
        let target_sibling = tree.discover_sibling(&target_parent, &target).unwrap();
        let target_sibling_node = match target_sibling.0.unwrap(){
                MerkleNodeEnum::MerkleNode(node) => node,
                MerkleNodeEnum::DummyLeaf => unreachable!("This should never happen!")
            };
        let target_sibling_lr = target_sibling.1;
        path.push((target_sibling_node.data, target_sibling_lr));
        target = target_parent.clone().data.clone();
        if &target != &root_value{
            target_parent = tree.discover_parent(&target).unwrap();
        }
    }

    // verify
    path.reverse();
    let mut current_hash = target_leaf.clone();
    while !path.is_empty(){
        let sibling = path.pop().unwrap();
        if sibling.1 == 0{
            current_hash = hashLeftRight(current_hash, sibling.0);
        }
        else if sibling.1 == 1{
            current_hash = hashLeftRight(sibling.0, current_hash);
        }
    }
    assert_eq!(&current_hash, &root_value);
}