// Patricia Merkle Tree for use in Kairos V0

extern crate alloc;
use std::borrow::BorrowMut;

use crate::helpers::hash_bytes;
//use alloc::boxed::Box;

#[derive(Clone, PartialEq, Debug)]
struct Root{
    hash: Option<Vec<u8>>,
    children: Vec<NodeEnum>,
}

#[derive(Clone, PartialEq, Debug)]
struct Leaf{
    key: Vec<u8>,
    hash: Vec<u8>,
    serialized_data: Option<Vec<u8>>,
}

#[derive(Clone, PartialEq, Debug)]
struct Node{
    key: Vec<u8>,
	hash: Option<Vec<u8>>,
    children: Vec<NodeEnum>,
}

#[derive(Clone, PartialEq, Debug)]
enum NodeEnum{
    Node(Node),
    Leaf(Leaf),
    Root(Root),
}
impl NodeEnum{
    fn unwrap_as_node(self) -> Node{
        match self{
            NodeEnum::Node(node) =>
                node,
            NodeEnum::Leaf(_) => panic!("Tried to unwrap a leaf as node!"),
            NodeEnum::Root(_) => panic!("Tried to unwrap a root as node!")
        }
    }
    fn unwrap_as_leaf(self) -> Leaf{
        match self{
            NodeEnum::Leaf(leaf) => 
                leaf,
            NodeEnum::Node(_) => panic!("Tried to unwrap a node as leaf!"),
            NodeEnum::Root(_) => panic!("Tried to unwrap a root as leaf!")
        }
    }
    fn unwrap_as_root(self) -> Root{
        match self{
            NodeEnum::Root(root) => 
                root,
            NodeEnum::Node(_) => panic!("Tried to unwrap a node as root!"),
            NodeEnum::Leaf(_) => panic!("Tried to unwrap a leaf as root!")
        }
    }
    fn has_key(&self, key: &[u8]) -> bool {
        match self{
            NodeEnum::Root(root) => {
                unreachable!("This should never happen!");
            }
            NodeEnum::Node(node) => {
                node.key == key
            }
            NodeEnum::Leaf(leaf) => {
                leaf.key == key
            }
        }
    }
    fn hash(&self) -> Vec<u8>{
        match self{
            NodeEnum::Root(root) => {
                let mut preimage: Vec<u8> = Vec::new();
                for child in &root.children{
                    match child{
                        NodeEnum::Node(node) => {
                            preimage.append(&mut node.hash.as_ref().unwrap().to_owned());
                        },
                        NodeEnum::Leaf(leaf) => {
                            preimage.append(&mut leaf.hash.to_owned());
                        },
                        NodeEnum::Root(_) => unreachable!("This should never happen!")
                    }
                };
                return hash_bytes(preimage);
            },
            NodeEnum::Node(node) => {
                let mut preimage: Vec<u8> = Vec::new();
                for child in &node.children{
                    match child{
                        NodeEnum::Node(node) => {
                            preimage.append(&mut node.hash.as_ref().unwrap().to_owned());
                        },
                        NodeEnum::Leaf(leaf) => {
                            preimage.append(&mut leaf.hash.to_owned());
                        },
                        NodeEnum::Root(_) => unreachable!("This should never happen!")
                    }
                };
                return hash_bytes(preimage);
            },
            NodeEnum::Leaf(leaf) => {
                return hash_bytes(leaf.key.to_owned());
            }
        }
    }
}

trait Updatable{
    fn update(&mut self);
    // recursive update function?
}

impl Updatable for Root{
    fn update(&mut self) {
        if self.children.is_empty(){
            return;
        }
        let mut preimage: Vec<u8> = Vec::new();
        for child in &self.children{
            preimage.append(&mut child.hash());
        }
        self.hash = Some(hash_bytes(preimage));
    }
}

impl Updatable for Node{
    fn update(&mut self) {
        if self.children.is_empty(){
            return;
        }
        let mut preimage: Vec<u8> = Vec::new();
        for child in &self.children{
            preimage.append(&mut child.hash());
        }
        self.hash = Some(hash_bytes(preimage));
    }
}

trait Appendable{
    fn append(&mut self, node: NodeEnum);
}

impl Appendable for Root{
    fn append(&mut self, node: NodeEnum) {
        self.children.push(node);
    }
}

impl Appendable for Node{
    fn append(&mut self, node: NodeEnum) {
        self.children.push(node);
    }
}

fn insert(current_node: &mut NodeEnum, transaction: &[u8], index: usize){
    if index >= transaction.len() - 1 {
        return;
    }
    let chunk = &transaction[index..index + 2];
    let is_last_chunk = index == transaction.len() - 2;
    match current_node{
        NodeEnum::Root(root) => {
            /*
                error when trying to add a leaf as a child of the root
            */
            if is_last_chunk{
                unreachable!("This should never happen!");
            }
            /*
                add a new node as a child of the root
            */
            else{
                /*
                    if the node already exists, re-use it instead of creating a new one
                    => every node will be unique
                */
                let mut has_key = false;
                for child in &mut root.children{
                    match child{
                        NodeEnum::Root(root) => unreachable!("Root can't be a child!"),
                        NodeEnum::Node(node) => {
                            if chunk == node.key{
                                // proceed with this node
                                insert(child, transaction, index + 2);
                                //root.update();
                                has_key = true;
                            }
                        },
                        NodeEnum::Leaf(leaf) => {}
                    };
                }
                if !has_key{
                    let mut new_node = NodeEnum::Node(Node{
                        key: chunk.to_vec(),
                        hash: None,
                        children: Vec::new()
                    });
                    insert(&mut new_node, transaction, index + 2);
                    root.append(new_node.clone());
                    root.update();
                }
            }
        },
        NodeEnum::Node(node) => {
            /*
                add a leaf as a final child of the node
            */
            if is_last_chunk{
                let new_leaf = NodeEnum::Leaf(Leaf{ 
                    key: chunk.to_vec(), 
                    hash: hash_bytes(chunk.to_vec()), 
                    serialized_data: None
                });
                node.append(new_leaf);
                node.update();
            }
            /* 
                add a node as a child of the node
            */
            else{
                let mut has_key = false;
                for child in &mut node.children{
                    match child{
                        NodeEnum::Root(root) => unreachable!("Root can't be a child!"),
                        NodeEnum::Node(node) => {
                            if chunk == node.key{
                                // proceed with this node
                                insert(child, transaction, index + 2);
                                //root.update();
                                has_key = true;
                            }
                        },
                        NodeEnum::Leaf(leaf) => {}
                    };
                }
                if !has_key{
                    let mut new_node = NodeEnum::Node(Node{
                        key: chunk.to_vec(),
                        hash: None,
                        children: Vec::new()
                    });
                    insert(&mut new_node, transaction, index + 2);
                    node.append(new_node.clone());
                    node.update();
                }
            }
        },
        NodeEnum::Leaf(leaf) => {
            unreachable!("This should never happen!")
        }
    }
}

fn insert_recursively(trie_root: &mut NodeEnum, transactions: Vec<Vec<u8>>){
    // insert the set of transactions and update the trie
    for transaction in transactions{
        insert(trie_root, transaction.as_ref(), 0);
    }
}

/*
fn check_for_target(trie_root: &mut NodeEnum, target_hash: &[u8], index: usize){
    if index >= target_hash.len() - 1 {
        return;
    }
    let chunk = &target_hash[index..index + 2];
    let is_last_chunk = index == target_hash.len() - 2;
    /*
        check if the current chunk is the key of a child of the current node,
        the current node can be a Root or Node, but never a Leaf
    */
}*/

fn check_for_target(trie_node: &NodeEnum, target_hash: &[u8], index: usize) -> bool {
    if index >= target_hash.len() - 1 {
        return false;
    }
    let chunk = &target_hash[index..index + 2];
    let is_last_chunk = index == target_hash.len() - 2;

    match trie_node {
        NodeEnum::Root(root) => {
            // Repeat the logic for Root, similar to Node
            for child in &root.children {
                match child {
                    NodeEnum::Node(node) => {
                        if node.key == chunk {
                            return check_for_target(child, target_hash, index + 2);
                        }
                    },
                    NodeEnum::Leaf(leaf) => {
                        if is_last_chunk && leaf.key == chunk {
                            return true;
                        }
                    },
                    _ => unreachable!(),
                }
            }
            false
        },
        NodeEnum::Node(node) => {
            // Existing logic for Node
            for child in &node.children {
                match child {
                    NodeEnum::Node(node) => {
                        if node.key == chunk {
                            return check_for_target(child, target_hash, index + 2);
                        }
                    },
                    NodeEnum::Leaf(leaf) => {
                        if is_last_chunk && leaf.key == chunk {
                            return true;
                        }
                    },
                    _ => unreachable!(),
                }
            }
            false
        },
        NodeEnum::Leaf(leaf) => {
            leaf.key == &target_hash[index..]
        },
    }
}


#[test]
fn tests(){
    use crate::helpers::hash_bytes;
    // create a set of 10 transactions
    
    let mut transactions: Vec<Vec<u8>> = Vec::new();
    for i in 0..255{
        transactions.push(hash_bytes(vec![0,0,i]));
    };


    //println!("Transactions: {:?}", &transactions);
    // create a trie instance
    let mut trie_root = NodeEnum::Root(Root { 
        hash: None, 
        children: Vec::new()
    });
    // insert recursively
    insert_recursively(&mut trie_root, transactions.clone());
    println!("Root: {:?}", &trie_root);
    for transaction in transactions{
        assert!(check_for_target(&trie_root, &transaction, 0));
    }
}
