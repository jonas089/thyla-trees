extern crate sha256;
use sha256::{digest, try_digest};
// insert, get
use std::{collections::HashMap};
use uint::construct_uint;
fn main(){
    panic!("Run cargo test -- --nocapture instead!");
}


/* 

construct_uint! {
    pub struct U256(4); // 4 * 64 = 256 bits
}

fn hash_string(input: String) -> String{
    digest(input)
}

fn hashLeftRight(left: String, right: String) -> String{
    hash_string(left + &right)
}

#[derive(Debug, Clone)]
struct MerkleNode{
    data: String,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>
}

fn build_merkle_tree(tx: Vec<String>) -> Option<MerkleNode>{
    if tx.is_empty(){
        return None;
    };
    // turn every transaction into a node
    let mut nodes = tx.into_iter()
        .map(|t| MerkleNode {
            data: t,
            left: None,
            right: None,
        })
        .collect::<Vec<_>>();
    // Build tree from the bottom up
    while nodes.len() > 1 {
        // New vector to hold the parents of the current level.
        let mut new_level = Vec::new();

        // Process nodes in pairs. If there's an odd one out, it will be included in the next level as-is.
        for pair in nodes.chunks_exact(2) {
            let left = Box::new(pair[0].clone());
            let right = Box::new(pair[1].clone());
            new_level.push(MerkleNode {
                data: hash_string(format!("{}{}", left.data, right.data)),
                left: Some(left),
                right: Some(right),
            });
        }

        // Check if there's one unpaired node left and carry it over to the next level.
        if nodes.len() % 2 != 0 {
            new_level.push(nodes.last().unwrap().clone());
        }

        // Move up to the next level of the tree.
        nodes = new_level;
    }
    // There's exactly one node left, the root of the Merkle tree
    nodes.pop()

}
// Tornado tree for Strings(hex) in Rust
#[derive(Default)]
struct TornadoTree{
    nextIndex: u32,
    currentRootIndex: u32,
    levels: u32,
    zero: Vec<String>,
    filledSubtrees: HashMap<u32, String>,
    roots: HashMap<u32, String>,
    ROOT_HISTORY_SIZE: u32,
    leafs: Vec<String>
}
impl TornadoTree{
    fn insert(&mut self, leaf: String) -> u32{
        self.leafs.push(leaf.clone());
        let mut currentIndex: u32 = self.nextIndex;
        let mut currentLevelHash: String = leaf;
        let mut left: String = String::new();
        let mut right: String = String::new();
        for i in 0..self.levels{
            if (currentIndex % 2 == 0){
                left = currentLevelHash.clone();
                right = self.zero[i as usize].clone();
                self.filledSubtrees.insert(i, currentLevelHash.clone());
            }
            else{
                left = self.filledSubtrees.get(&i).unwrap().clone();
                right = currentLevelHash;
            }
            currentLevelHash = hashLeftRight(left, right);
            currentIndex /= 2;
        };
        let newRootIndex: u32 = (self.currentRootIndex + 1) % self.ROOT_HISTORY_SIZE;
        self.currentRootIndex = newRootIndex;
        self.roots.insert(newRootIndex, currentLevelHash);
        self.nextIndex = self.nextIndex + 1;
        self.nextIndex
    }
    
    fn calculateLevels(&mut self){
        let mut zero: Vec<String> = Vec::new();
        let zero_value: String = String::from("snark");
        // first level hash
        let mut current_hash: String = hash_string(zero_value.clone() + &zero_value);
        zero.push(current_hash.clone());
        // next level hashs
        for i in 0..self.levels - 1{
            current_hash = hash_string(current_hash.clone() + &current_hash);
            zero.push(current_hash.clone());
        };
        self.zero = zero.clone();
        self.roots.insert(0, zero[zero.len() - 1].clone());
    }

    fn getLastRoot(&self) -> String{
        return self.roots[&self.currentRootIndex].clone();
    }
}

#[test]
fn tornado(){
    let levels: u32 = 4;
    const ROOT_HISTORY_SIZE: u32 = 30;
    // construct empty tree from params
    let mut tree = TornadoTree::default();
    tree.levels = levels;
    tree.ROOT_HISTORY_SIZE = ROOT_HISTORY_SIZE;
    tree.calculateLevels();
    println!("Root before insert: {:?}", tree.getLastRoot());
    tree.insert(String::from("some_transaction_id"));
    println!("Root after first insert: {:?}", tree.getLastRoot());
    tree.insert(String::from("some_other_transaction_id"));
    println!("Root after second insert: {:?}", tree.getLastRoot());

    tree.insert(String::from("some_3rd_transaction"));
    tree.insert(String::from("some_5th_transaction"));
    println!("test: {:?}", tree.filledSubtrees);
    println!("Leafs: {:?}", tree.leafs);
    let tx = vec![String::from("some_tx"); 11];
    let full_tree = build_merkle_tree(tx.clone());
    println!("Input tx: {:?}", tx);
    println!("Full tree: {:?}", full_tree);
}

/* What's to be proven
    * valid merkle path computation (assert | == root)
*/

/* How Tornadocash works
    * deposit => add leaf to preimage of tree
    * withdraw => construct full merkle tree, prove path for a known leaf
    * the transaction itself is not stored on-chain, only a commitment (hash)
    * knowing the preimage of the hash and being able to prove the path makes you elible to withdraw
*/

/* Construct full merkle tree
    * Take leafs from tornado tree
    * Build merkle tree bottom-up
    * obtain proof path
    * generate zk-proof for merkle path
*/


*/



#[derive(Debug, Clone)]
struct MerkleNode{
    data: String,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>
}

fn hash_string(input: String) -> String{
    digest(input)
}

fn hashLeftRight(left: String, right: String) -> String{
    hash_string(left + &right)
}



/* Tree parameters

    * Depth: 3
    * Size: 2^2 = 4 transactions

*/

fn constructor(depth: u32) -> Option<Vec<MerkleNode>>{
    // zero val for merkle roots
    let size = 2_u32.pow(depth-1_u32);
    let zero_val = String::from("casper");
    let mut zero_levels: Vec<String> = Vec::new();
    let mut current_level = zero_val.clone();
    zero_levels.push(zero_val.clone());
    for level in 0..depth - 2{
        let _hash = hashLeftRight(current_level.clone(), current_level.clone());
        zero_levels.push(_hash.clone());
        current_level = _hash;
    };
    println!("Levels: {:?}", zero_levels);
    
    let transactions = vec![String::from("tx01"), String::from("tx02")];
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
    println!("Bottom level: {:?}", &bottom_level);
    // start at first hash (one level above tx data)
    let mut current_level_height = 1;
    while current_level.len() > 1{
        println!("Current Level: {:?}", &current_level);


        while current_level.len() % 2 != 0{
            current_level.push(MerkleNode { data: zero_levels[current_level_height].clone(), left: None, right: None });
        }
        let mut new_level: Vec<MerkleNode> = Vec::new();



        println!("Len of current level: {:?}", &current_level.len());
        for i in (0..current_level.len()).step_by(2){
            let left: MerkleNode = current_level[i].clone();
            let right: MerkleNode = current_level[i+1].clone();
            new_level.push(MerkleNode { 
                data: hashLeftRight(left.clone().data, right.clone().data), 
                left: Some(Box::new(left.clone())), 
                right: Some(Box::new(right.clone()))}
            );
            levels.push(new_level.clone());
        };
        current_level = new_level.clone();
        current_level_height += 1;
    };
    return levels.pop();

    // return here
}

#[test]
fn test_constructor(){
    let depth: u32 = 3;
    let result = constructor(depth).unwrap();
    println!("Merkle Tree: {:?}", result);
}