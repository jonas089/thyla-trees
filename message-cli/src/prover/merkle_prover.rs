use merkle_tree::tornado::{TornadoTree};
use merkle_tree::helpers;
use helpers::{hash_bytes, hashLeftRight};

fn generate_merkle_proof_inputs(leaf: Vec<u8>){
    // a simple tree for testing with depth 5 (including root node)
    let mut tree = TornadoTree{
        zero_node: hash_bytes(vec![0;32]),
        zero_levels: Vec::new(),
        root_history: Vec::new(),
        filled: vec![vec![], vec![], vec![], vec![], vec![]],
        index: 0,
        depth: 5
    };
    // calculate the zero levels for the tree
    tree.calculate_zero_levels();
    // initialize the proof path container
    let mut proof_path: Vec<(Vec<u8>, bool)> = tree.add_leaf(leaf);
    // pop the merkle root for comparison (test assertion)
    let merkle_root: &Vec<u8> = &tree.filled.pop().unwrap();
    
    // collect the proof path
    let mut current_hash: Vec<u8> = proof_path[0].clone().0;
    for i in 1..proof_path.len(){
        if &proof_path[i].1 == &true{
            current_hash = hashLeftRight(current_hash, proof_path[i].clone().0);
        }
        else{
            current_hash = hashLeftRight(proof_path[i].clone().0, current_hash);
        }
    };
    assert_eq!(&current_hash, merkle_root);
    println!("Proof Path: {:?}", proof_path);
}