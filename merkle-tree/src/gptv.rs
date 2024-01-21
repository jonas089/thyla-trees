// modified version of patricia.rs created w. help of GPT
// insert takes very long?

extern crate alloc;
use std::{collections::HashMap, borrow::BorrowMut};
use crate::helpers::hash_bytes;
use std::time::{SystemTime, UNIX_EPOCH};

const CHUNK_SIZE: usize = 2;

#[derive(Clone, PartialEq, Debug)]
struct Root {
    hash: Option<Vec<u8>>,
    children: HashMap<Vec<u8>, NodeEnum>,
}

#[derive(Clone, PartialEq, Debug)]
struct Leaf {
    hash: Vec<u8>,
    serialized_data: Option<Vec<u8>>,
}

#[derive(Clone, PartialEq, Debug)]
struct Node {
    hash: Option<Vec<u8>>,
    children: HashMap<Vec<u8>, NodeEnum>,
}

#[derive(Clone, PartialEq, Debug)]
enum NodeEnum {
    Node(Node),
    Leaf(Leaf),
    Root(Root),
}

impl NodeEnum {
    fn hash(&self, key: &Vec<u8>) -> Vec<u8> {
        match self {
            NodeEnum::Root(root) => {
                let mut preimage: Vec<u8> = Vec::new();
                for (child_key, child) in root.children.iter() {
                    preimage.append(&mut child.hash(child_key));
                }
                hash_bytes(preimage)
            },
            NodeEnum::Node(node) => {
                let mut preimage: Vec<u8> = Vec::new();
                for (child_key, child) in node.children.iter() {
                    preimage.append(&mut child.hash(child_key));
                }
                hash_bytes(preimage)
            },
            NodeEnum::Leaf(_) => {
                hash_bytes(key.clone())
            },
        }
    }
}

trait Updatable {
    fn update(&mut self);
}

impl Updatable for Root {
    fn update(&mut self) {
        if self.children.is_empty() {
            return;
        }
        let mut preimage: Vec<u8> = Vec::new();
        for (child_key, child) in self.children.iter() {
            preimage.append(&mut child.hash(child_key));
        }
        self.hash = Some(hash_bytes(preimage));
    }
}

impl Updatable for Node {
    fn update(&mut self) {
        if self.children.is_empty() {
            return;
        }
        let mut preimage: Vec<u8> = Vec::new();
        for (child_key, child) in self.children.iter() {
            preimage.append(&mut child.hash(child_key));
        }
        self.hash = Some(hash_bytes(preimage));
    }
}

trait Appendable {
    fn append(&mut self, key: Vec<u8>, node: NodeEnum);
}

impl Appendable for Root {
    fn append(&mut self, key: Vec<u8>, node: NodeEnum) {
        self.children.insert(key, node);
    }
}

impl Appendable for Node {
    fn append(&mut self, key: Vec<u8>, node: NodeEnum) {
        self.children.insert(key, node);
    }
}

fn insert(current_node: &mut NodeEnum, transaction: &[u8], index: usize) {
    if index >= transaction.len() {
        return;
    }
    let chunk = &transaction[index..index + CHUNK_SIZE];
    let is_last_chunk = index >= transaction.len() - CHUNK_SIZE;

    match current_node {
        NodeEnum::Root(root) => {
            if let Some(existing_child) = root.children.get_mut(chunk) {
                insert(existing_child, transaction, index + CHUNK_SIZE);
            } else {
                let new_child = if is_last_chunk {
                    NodeEnum::Leaf(Leaf { 
                        hash: hash_bytes(chunk.to_vec()), 
                        serialized_data: None 
                    })
                } else {
                    let mut new_node = NodeEnum::Node(Node {
                        hash: None,
                        children: HashMap::new(),
                    });
                    insert(&mut new_node, transaction, index + CHUNK_SIZE);
                    new_node
                };
                root.children.insert(chunk.to_vec(), new_child);
            }
            root.update();
        },
        NodeEnum::Node(node) => {
            if let Some(existing_child) = node.children.get_mut(chunk) {
                insert(existing_child, transaction, index + CHUNK_SIZE);
            } else {
                let new_child = if is_last_chunk {
                    NodeEnum::Leaf(Leaf { 
                        hash: hash_bytes(chunk.to_vec()), 
                        serialized_data: None 
                    })
                } else {
                    let mut new_node = NodeEnum::Node(Node {
                        hash: None,
                        children: HashMap::new(),
                    });
                    insert(&mut new_node, transaction, index + CHUNK_SIZE);
                    new_node
                };
                node.children.insert(chunk.to_vec(), new_child);
            }
            node.update();
        },
        NodeEnum::Leaf(_) => unreachable!("Leaf should not have children!"),
    }
}

fn insert_recursively(trie_root: &mut NodeEnum, transactions: Vec<Vec<u8>>) {
    let mut height = 0;
    for transaction in transactions{
        let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        insert(trie_root, transaction.as_ref(), 0);
        println!("Elapsed time: {:?} height: {:?}", SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start_time, &height);
        height += 1;
    }
}

fn check_for_target(trie_node: &NodeEnum, target_hash: &[u8], index: usize) -> bool {
    if index >= target_hash.len() {
        return false;
    }
    let chunk = &target_hash[index..index + CHUNK_SIZE];
    let is_last_chunk = index >= target_hash.len() - CHUNK_SIZE;

    match trie_node {
        NodeEnum::Root(root) => {
            if let Some(child) = root.children.get(chunk) {
                if is_last_chunk {
                    match child {
                        NodeEnum::Leaf(leaf) => leaf.hash == hash_bytes(chunk.to_vec()),
                        _ => false,
                    }
                } else {
                    check_for_target(child, target_hash, index + CHUNK_SIZE)
                }
            } else {
                false
            }
        },
        NodeEnum::Node(node) => {
            if let Some(child) = node.children.get(chunk) {
                if is_last_chunk {
                    match child {
                        NodeEnum::Leaf(leaf) => leaf.hash == hash_bytes(chunk.to_vec()),
                        _ => false,
                    }
                } else {
                    check_for_target(child, target_hash, index + CHUNK_SIZE)
                }
            } else {
                false
            }
        },
        NodeEnum::Leaf(leaf) => {
            is_last_chunk && leaf.hash == hash_bytes(chunk.to_vec())
        },
    }
}

#[test]
fn test_gptv() {
    let start_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

    let mut transactions: Vec<Vec<u8>> = Vec::new();
    for i in 0..100 {
        for j in 0..10 {
            for k in 0..10 {
                transactions.push(hash_bytes(vec![i, j, k]));
            }
        }
    }
    println!("transactions: {:?}", &transactions.len());
    println!("transaction: {:?}", &transactions[0]);
    let mut trie_root = NodeEnum::Root(Root { 
        hash: None, 
        children: HashMap::new()
    });    
    insert_recursively(&mut trie_root, transactions.clone());
    for transaction in transactions {
        assert!(check_for_target(&trie_root, &transaction, 0));
    };

    let elapsed_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap() - start_time;
    println!("Elapsed time: {:?}", elapsed_time);
}