// Patricia Merkle Tree for use in Kairos V0

use std::{collections::HashSet, vec, slice::Chunks, process::Child};
extern crate alloc;
use alloc::boxed::Box;
use digest::typenum::Le;

struct Trie{
	levels: Vec<Option<NodeEnum>>
}

#[derive(Clone)]
struct Level{
	hash: Option<Vec<u8>>,
	children: Vec<Node>,
}

#[derive(Clone, PartialEq)]
struct Node{
    key: &'static [u8],
	hash: Vec<u8>,
    children: Option<Vec<Node>>,
	serialized_data: Option<Vec<u8>>
}

impl Node{
    fn has_key(&self, key: &[u8]) -> bool {
        self.key == key
    }
}

#[derive(Clone)]
enum NodeEnum{
	Level(Box<Level>),
	Node(Box<Node>)
}
impl NodeEnum{
    fn unwrap_as_level(self) -> Level{
        let level = match self{
            NodeEnum::Level(level) => {
                return *level;
            },
            NodeEnum::Node(_) => unreachable!("This should never happen!")
        };
    }
    
}


#[test]
fn tests(){
    // max depth of trie will be 32/2 = 16
    let trie = Trie{
        levels: vec![None;16]
    };
    use crate::helpers::hash_bytes;
    // create a set of 10 transactions
    let mut transactions: Vec<Vec<u8>> = Vec::new();
    for i in 0..10{
        transactions.push(hash_bytes(vec![0,0,i]));
    };
    for transaction in transactions{
        let mut depth: usize = 0;
        for chunk in transaction.chunks(2){
            let level = match trie.levels[depth].clone(){
                Some(level) => level,
                None => {
                    // return a new level
                    NodeEnum::Level(Box::new(Level{
                        hash: None,
                        children: Vec::new()
                    }))
                }
            }.unwrap_as_level();
            // insert to level if key does not exist
            // move on to the next level if key does exist
            // update all hashs for all affected levels

            let mut is_new: bool = true;
            for child in level.children{
                if child.has_key(&chunk){
                    is_new = false;
                    break;
                }
            }
            if is_new{
                // insert the new chunk (as a Node) into the child
                // don't yet update the hashs, that happens in the end
                // to find a given node, use the key (Node.has_key())
            }
            depth += 1;
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