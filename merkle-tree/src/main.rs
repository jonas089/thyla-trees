extern crate sha256;
use sha256::{digest, try_digest};
// insert, get
use std::collections::HashMap;
use uint::construct_uint;
fn main(){
    panic!("Run cargo test -- --nocapture instead!");
}

construct_uint! {
    pub struct U256(4); // 4 * 64 = 256 bits
}

fn hash_string(input: String) -> String{
    digest(input)
}

fn hashLeftRight(left: String, right: String) -> String{
    hash_string(left + &right)
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
    ROOT_HISTORY_SIZE: u32
}
impl TornadoTree{
    fn insert(&mut self, leaf: String) -> u32{
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
}