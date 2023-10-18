extern crate sha256;
use core::panic;
use std::thread::current;

use sha256::{digest, try_digest};

fn main(){
    panic!("Run cargo test -- --nocapture instead!");
}

fn hash_string(input: String) -> String{
    digest(input)
}

#[derive(Debug, Clone)]
struct MerkleNode{
    data: String,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>
}

impl MerkleNode{
    fn is_empty(self) -> bool{
        if self.data.len() == 0{
            return true;
        }
        false
    }
}


// tree constructor (tested)
fn build_merkle_tree(tx: Vec<String>) -> Option<MerkleNode> {
    // Immediate return if no transactions
    if tx.is_empty() {
        return None;
    }

    // Create leaves for each transaction
    let mut nodes = tx.into_iter()
        .map(|t| MerkleNode {
            data: t,
            left: None,
            right: None,
        })
        .collect::<Vec<_>>();

    // Build tree from the bottom up
    while nodes.len() > 1 {
        // If odd number of nodes, duplicate last
        if nodes.len() % 2 != 0 {
            let last = nodes.last().unwrap().clone();
            nodes.push(last);
        }
        // Combine pairs of nodes
        nodes = nodes.chunks(2).map(|pair| {
            let left = Box::new(pair[0].clone());
            let right = Box::new(pair[1].clone());
            MerkleNode {
                data: hash_string(format!("{}{}", left.data, right.data)),
                left: Some(left),
                right: Some(right),
            }
        }).collect();
    }
    // There's exactly one node left, the root of the Merkle tree
    nodes.pop()
}

// recursive function to find the path to a leaf
fn find_leaf_path(root: MerkleNode, target: String, path: Vec<String>) -> Option<Vec<String>>{
    if root.clone().is_empty(){
        return None;
    }
    if root.data == target{
        return Some(path);
    }
    let mut path_cp_left = path.clone();
    path_cp_left.push(root.data.clone());
    let mut path_cp_right = path.clone();
    path_cp_right.push(root.data.clone());
    if let Some(ref left) = root.left{
        let left_path = find_leaf_path(*root.clone().left.unwrap(), target.clone(), path_cp_left);
        if !left_path.is_none(){
            return left_path;
        }
    }
    if let Some(ref right) = root.right{
        let right_path = find_leaf_path(*root.clone().right.unwrap(), target, path_cp_right);
        if !right_path.is_none(){
            return right_path;
        }
    }
    return None;
}

// find the parent for a leaf in the tree
fn find_leaf_parent(root: MerkleNode, target: String) -> Option<MerkleNode>{
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
        let left_node = find_leaf_parent(*root.clone().left.unwrap(), target.clone());
        if !left_node.is_none(){
            return  left_node;
        }
    }
    if let Some(ref right) = root.right{
        let right_node = find_leaf_parent(*root.clone().right.unwrap(), target.clone());
        if !right_node.is_none(){
            return right_node;
        }
    }
    return None
}

// find the sibling of a leaf -> takes parent as input
fn find_leaf_sibling(root: MerkleNode, target: String) -> Option<MerkleNode>{
    if let Some(ref left) = root.left{
        if root.clone().left.unwrap().data == target{
            return Some(*root.clone().right.unwrap());
        }
        else{
            return Some(*root.clone().left.unwrap());
        }
    }
    else{
        return None;
    }
}

// put the merkle path in order to generate inputs for the circuit
// -> this eliminates the need to re-hash the merkle inclusion in a distributed system
fn merkle_path_in_order(merkle_root: String, merkle_tree: MerkleNode, mut proof_path: Vec<String>) -> Vec<Vec<String>>{
    let mut left_side = Vec::new();
    let mut right_side = Vec::new();

    let mut current_hash = proof_path.pop().unwrap();
    let parent = find_leaf_parent(merkle_tree.clone(), current_hash.clone()).unwrap();
    if let Some(ref left) = parent.left{
        if left.clone().data == current_hash{
            left_side.push(current_hash.clone());
        }
        else{
            right_side.push(current_hash.clone());
        }
    }
    while !proof_path.is_empty() {
        let parent = find_leaf_parent(merkle_tree.clone(), current_hash.clone()).unwrap();
        let sibling = proof_path.pop().unwrap();

        // there is always one node to the left and one to the right
        // first push left node, then push the right node
        if let Some(ref left) = parent.left{
            if left.clone().data == current_hash{
                right_side.push(sibling.clone());
                current_hash = hash_string(current_hash + &sibling);
            }
            else{
                left_side.push(sibling.clone());
                current_hash = hash_string(sibling.clone() + &current_hash);
            }
        }
    }
    vec![left_side, right_side]
}

// verify that a merkle path is valid
fn verify_merkle_proof(merkle_root: String, merkle_tree: MerkleNode, mut proof_path: Vec<String>) -> bool {
    let mut current_hash = proof_path.pop().unwrap();
    while !proof_path.is_empty() {
        println!("Remaining proof path: {:?}", proof_path);
        println!("Current Hash: {:?}", current_hash);
        let parent = find_leaf_parent(merkle_tree.clone(), current_hash.clone()).unwrap();
        let sibling = proof_path.pop().unwrap();//find_leaf_sibling(parent.clone(), current_hash.clone()).unwrap();
        if let Some(ref left) = parent.left{
            if left.clone().data == current_hash{
                println!("Is left.");
                println!("hashing: {:?} and {:?}", current_hash, &sibling);
                current_hash = hash_string(current_hash + &sibling);
            }
            else{
                current_hash = hash_string(sibling.clone() + &current_hash);
            }
        }
    }
    assert_eq!(merkle_root, current_hash);
    merkle_root == current_hash
    /*
    println!("Final hash: {:?}", &current_hash);
    //assert_eq!(merkle_root, current_hash);
    merkle_root == current_hash
    */  
}

#[test]
fn more_tests(){
    let mut transactions: Vec<String> = Vec::new();
    let mut ids: Vec<String> = Vec::new();
    // merkle tree with 32 transactions
    for i in 0..32{
        let _id = format!("0x{}", i.to_string());
        transactions.push(_id.clone());
        ids.push(_id);
    };
    let merkle_tree = build_merkle_tree(transactions.clone());
    let merkle_root = merkle_tree.clone().unwrap().data;
    //println!("Merkle Tree: {:?}", merkle_tree.clone());

    // Tx right from tree root
    let parent = find_leaf_parent(merkle_tree.clone().unwrap(), String::from("0x31"));
    let sibling = find_leaf_sibling(parent.clone().unwrap(), String::from("0x31"));
    //println!("Sibling of 0x5: {:?}", sibling);
    let mut path = find_leaf_path(merkle_tree.clone().unwrap(), String::from("0x31"), Vec::new()).unwrap();
    path.push(String::from("0x31"));
    println!("Path: {:?}", path);
    let mut proof_path: Vec<String> = Vec::new();
    for leaf in &path.clone()[1..path.len()-1]{
        //proof_path.push(leaf.clone());
        let leaf_parent = find_leaf_parent(merkle_tree.clone().unwrap(), leaf.clone()).unwrap();
        let leaf_sibling = find_leaf_sibling(leaf_parent.clone(), leaf.clone()).unwrap();
        //println!("Parent of leaf {:?} is {:?}", &leaf, &leaf_parent);
        //println!("Sibling of leaf {:?} is {:?}", &leaf, &leaf_sibling);
        proof_path.push(leaf_sibling.data);
    };
    // append transaction and sibling
    for leaf in &path.clone()[path.len()-1..]{
        proof_path.push(leaf.clone());
        let leaf_parent = find_leaf_parent(merkle_tree.clone().unwrap(), leaf.clone()).unwrap();
        let leaf_sibling = find_leaf_sibling(leaf_parent.clone(), leaf.clone()).unwrap();
        proof_path.push(find_leaf_sibling(leaf_parent.clone(), leaf.to_string()).unwrap().data);
    }
    println!("Proof path with tx: {:?}", proof_path.clone());
    println!("Merkle root: {:?}", &merkle_root);
    println!("[RESULT] Verifier: {:?}", verify_merkle_proof(merkle_root.clone(),  merkle_tree.clone().unwrap(), proof_path.clone()));
    println!("{:?}", 0%2==0);

    // derive full proof path (circuit input) for a transaction
    println!("{:?}", merkle_path_in_order(merkle_root.clone(), merkle_tree.clone().unwrap(), proof_path.clone()));
    let left_side = merkle_path_in_order(merkle_root.clone(), merkle_tree.clone().unwrap(), proof_path.clone())[0].clone();
    let right_side = merkle_path_in_order(merkle_root.clone(), merkle_tree.clone().unwrap(), proof_path.clone())[1].clone();
    let depth: usize = {
        if left_side.len() > right_side.len(){
            left_side.len()
        }
        else{
            right_side.len()
        }
    };
    let complete_proof_path: Vec<Vec<String>> = vec![left_side, right_side];
    let mut current_hash = String::new();
    for i in 0..depth{
        if i < complete_proof_path[0].len() && i < complete_proof_path[1].len(){
            current_hash = hash_string(complete_proof_path[0][i].clone() + &complete_proof_path[1][i]);
        }
        else if i < complete_proof_path[0].len(){
            current_hash = hash_string(complete_proof_path[0][i].clone() + &current_hash);
        }
        else if i < complete_proof_path[1].len(){
            current_hash = hash_string(current_hash + &complete_proof_path[1][i].clone());
        }
    }
    println!("Output hash: {:?}", current_hash);
    assert_eq!(current_hash, merkle_root);
}

/*
! number of nodes must be even, or else last element will be duplicated / e.g. transactions / 2 == even

Merkle tree: Some(MerkleNode { data: "9893e1b34d1bedb6f26c1b74daa1d94c8312003718674ffa1afecba378b9d735", left: Some(MerkleNode { data: "00ada7f0393fced15bbb1fa02b200e487d1ea2562e63acff56ad8a753de9f981", left: Some(MerkleNode { data: "cc68c6ed4c7ec3ee1340b3227035ad94e33cf9a7a59345af0a5a49ee1723dcad", left: Some(MerkleNode { data: "0x0", left: None, right: None }), right: Some(MerkleNode { data: "0x1", left: None, right: None }) }), right: Some(MerkleNode { data: "7e844584f83a63208c5bb8851057910d2040eb253de53bce3057c33286270f7c", left: Some(MerkleNode { data: "0x2", left: None, right: None }), right: Some(MerkleNode { data: "0x3", left: None, right: None }) }) }), right: Some(MerkleNode { data: "cba59f10224e845a7c090576b53a35bb613d3bf91545b5c12d9e9c23e653946d", left: Some(MerkleNode { data: "16b30164c14d9cdc8c60f961799740352dd32cf7b1010b9c7cadaacc418e0e25", left: Some(MerkleNode { data: "0x4", left: None, right: None }), right: Some(MerkleNode { data: "0x5", left: None, right: None }) }), right: Some(MerkleNode { data: "97628320616cfee422be81b7eb8500ac796aaf34aa6c4a45777edd6546df116b", left: Some(MerkleNode { data: "0x6", left: None, right: None }), right: Some(MerkleNode { data: "0x7", left: None, right: None }) }) }) })

*/

/* Limited circuit design
    * sort hash path before submission according to L / R position
    * simple re-hashing in reverse order
    * [0x1, 0x2, ax33, fb44, cd59]
    -> c = H(H(H(H(0x1, 0x2), ax33), fb44), cd59)
    assert(c == merkle_hash)
    ! fixed size array

*/

/*

Vec<(String, bool)>, where bool indicates whether a node is left or right.
-> Single, fixed size Vec that can be used as circuit input.
-> pass as [(String, bool)] or equivalent [String] & [bool]; n
-> hash the bytes of each String and compare merkle hash.
*/