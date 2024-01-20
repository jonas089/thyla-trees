// Patricia Merkle Tree for use in Kairos V0

extern crate alloc;
use std::borrow::BorrowMut;

use crate::helpers::hash_bytes;
//use alloc::boxed::Box;

#[derive(Clone, PartialEq)]
struct Root{
    hash: Option<Vec<u8>>,
    children: Vec<NodeEnum>,
}

#[derive(Clone, PartialEq)]
struct Leaf{
    key: Vec<u8>,
    hash: Vec<u8>,
    serialized_data: Option<Vec<u8>>,
}

#[derive(Clone, PartialEq)]
struct Node{
    key: Vec<u8>,
	hash: Option<Vec<u8>>,
    children: Vec<NodeEnum>,
}

#[derive(Clone, PartialEq)]
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

trait Updateable{
    fn update(&mut self);
    // recursive update function?
}

impl Updateable for Root{
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

impl Updateable for Node{
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

#[test]
fn tests(){
    use crate::helpers::hash_bytes;
    // create a set of 10 transactions
    let mut transactions: Vec<Vec<u8>> = Vec::new();
    for i in 0..10{
        transactions.push(hash_bytes(vec![0,0,i]));
    };
    // construct a trie root
    let mut trie_root = NodeEnum::Root(Root { 
        hash: None, 
        children: Vec::new()
    });
    // insert the set of transactions and update the trie
    let mut current_node = trie_root.clone();
    for transaction in transactions{
        for chunk in transaction.chunks(2){
            // should first check if the chunk already exists
            /*
            
            
                tbd.
            
            */
            match &mut current_node{
                NodeEnum::Root(root) => {
                    if chunk == &transaction[transaction.len() - 2..]{
                        unreachable!("This should never happen!");
                    }
                    else{
                        let mut new_node = NodeEnum::Node(Node{
                            key: chunk.to_vec(),
                            hash: None,
                            children: Vec::new()
                        });
                        root.append(new_node.clone());
                        current_node = new_node;
                    }
                },
                NodeEnum::Node(node) => {
                    if chunk == &transaction[transaction.len() - 2..]{
                        let new_leaf = NodeEnum::Leaf(Leaf{ 
                            key: chunk.to_vec(), 
                            hash: hash_bytes(chunk.to_vec()), 
                            serialized_data: None
                        });
                        node.append(new_leaf);
                    }
                    else{
                        let mut new_node = NodeEnum::Node(Node{
                            key: chunk.to_vec(),
                            hash: None,
                            children: Vec::new()
                        });
                        node.append(new_node.clone());
                        current_node = new_node;
                    }
                },
                NodeEnum::Leaf(leaf) => {
                    unreachable!("This should never happen!")
                }
            }
        }
    }
}

/* How hashs are updated
    * Update only the path for the newly inserted node
    * re-hash using all siblings for each level (bottom up)
*/

/* How merkle paths are constructed
    * Find all siblings for all levels
    * L/R does not matter, in-order [0..n]

*/