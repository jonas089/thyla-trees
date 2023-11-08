use dep::std;
use dep::std::collections::vec::Vec;

fn hash_sha(message: [u8]) -> [u8; 32]{
    std::hash::sha256(message)
}

// input size depends on tree depth
fn main(transaction: [u8;64], merkle_proof: [[u8; 64];4], position: [[u8;1];4], merkle_root: [u8;64]) {
    let combined_slice = combine(transaction, merkle_proof[0]);
    assert(combined_slice.len() == 128);
    
}

// combine LR
fn combine(left: [u8; 64], right: [u8; 64]) -> [u8]{
    let mut combined_vec: Vec<u8> = Vec::new();
    for byte in left{
        combined_vec.push(byte);
    };
    for byte in right{
        combined_vec.push(byte);
    };
    combined_vec.slice
}

// combine RL
fn combine_reverse(right: [u8; 64], left: [u8; 64]) -> [u8]{
    let mut combined_vec: Vec<u8> = Vec::new();
    for byte in right{
        combined_vec.push(byte);
    };
    for byte in left{
        combined_vec.push(byte);
    };
    combined_vec.slice
}

#[test]
fn test_main() {
    main(
    // transaction hash
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    // merkle proof
    [
        [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        [54, 102, 102, 50, 102, 97, 56, 102, 55, 53, 52, 51, 101, 50, 55, 98, 57, 100, 98, 50, 57, 52, 98, 101, 53, 56, 48, 57, 48, 97, 50, 54, 54, 57, 100, 101, 55, 51, 48, 57, 97, 98, 101, 57, 50, 98, 99, 53, 98, 102, 56, 48, 50, 50, 98, 102, 50, 56, 98, 56, 52, 48, 52, 102],
        [53, 57, 98, 101, 101, 101, 52, 56, 100, 52, 57, 49, 48, 49, 97, 48, 57, 57, 101, 54, 49, 50, 55, 102, 48, 102, 101, 55, 48, 52, 97, 97, 53, 102, 57, 52, 97, 101, 57, 101, 102, 101, 102, 54, 51, 48, 54, 97, 102, 99, 54, 97, 97, 99, 55, 100, 101, 56, 50, 52, 51, 99, 98, 53],
        [102, 51, 98, 50, 54, 48, 50, 55, 51, 54, 99, 97, 49, 97, 52, 50, 48, 54, 52, 50, 102, 99, 52, 97, 98, 100, 48, 102, 57, 51, 52, 100, 48, 56, 54, 49, 102, 97, 57, 51, 101, 56, 57, 102, 54, 100, 55, 54, 57, 52, 54, 55, 52, 55, 50, 54, 52, 49, 52, 50, 54, 98, 50, 53],
    ],
    // positions
    [
        [0],
        [0],
        [0],
        [0]
    ],
    // current merkle root
    [102, 52, 50, 98, 50, 97, 52, 49, 52, 98, 51, 102, 102, 100, 56, 97, 48, 99, 53, 48, 99, 51, 102, 54, 52, 52, 49, 51, 50, 52, 52, 53, 55, 52, 54, 56, 50, 54, 49, 102, 101, 98, 56, 102, 52, 99, 99, 52, 56, 101, 51, 56, 52, 99, 102, 51, 49, 49, 57, 57, 53, 57, 49, 55]
    );
}


/* Specification

    * Merkle tree specification
        * depth 5

    * Input
        * transaction
        * proof path for transaction (including all siblings)

    * Processing
        * Input u8 arrays (len 64) => 1:1 sha256 hash
        * Input u8 merkle hash

    Assert if current hash != merkle hash

*/