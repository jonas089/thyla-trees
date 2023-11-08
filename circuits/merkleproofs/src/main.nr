use dep::std;
use dep::std::collections::vec::Vec;

fn hash_sha(message: [u8]) -> [u8; 32]{
    std::hash::sha256(message)
}

// input size depends on tree depth
fn main(transaction: [u8;32], merkle_proof: [[u8; 32];4], position: [u8;4], merkle_root: [u8;32], debug_hashs: [[u8;32];4]) {
    let mut current_hash: [u8;32] = [0;32];
    if position[0] == 0{
        current_hash = hash_sha(combine(transaction, merkle_proof[0]));
    }
    else{
        current_hash = hash_sha(combine(merkle_proof[0], transaction));
    };
    for i in 1..merkle_proof.len(){
        if position[i] == 0{
            current_hash = hash_sha(combine(current_hash, merkle_proof[i]));
        }
        else{
            current_hash = hash_sha(combine(merkle_proof[i], current_hash));
        }
    };

    assert(current_hash == merkle_root);
}

// combine LR
fn combine(left: [u8; 32], right: [u8; 32]) -> [u8]{
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
fn combine_reverse(right: [u8; 32], left: [u8; 32]) -> [u8]{
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
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    // merkle proof
    [
        [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2],
        [178, 137, 222, 169, 44, 165, 171, 165, 242, 225, 137, 26, 26, 241, 27, 226, 121, 20, 196, 136, 84, 219, 15, 229, 180, 187, 149, 193, 55, 224, 242, 214],
        [248, 211, 204, 204, 180, 196, 230, 213, 226, 254, 251, 255, 140, 104, 170, 245, 141, 86, 82, 142, 59, 109, 142, 191, 7, 180, 33, 12, 239, 230, 161, 241],
        [175, 132, 242, 248, 185, 9, 188, 62, 34, 213, 240, 199, 176, 177, 75, 99, 187, 215, 70, 226, 72, 67, 45, 66, 103, 218, 50, 31, 1, 52, 216, 168],
    ],
    // positions
    [
        0,
        0,
        0,
        0
    ],
    // current merkle root
    [122, 111, 152, 168, 16, 14, 202, 82, 72, 133, 213, 28, 57, 178, 64, 160, 4, 58, 202, 252, 110, 5, 87, 19, 48, 234, 78, 220, 229, 87, 141, 223],
    // debug hashs
    [
        [248, 24, 175, 211, 122, 109, 195, 188, 146, 251, 68, 115, 16, 17, 39, 112, 6, 219, 78, 250, 110, 144, 35, 205, 116, 104, 192, 35, 53, 210, 42, 77],
        [74, 138, 225, 194, 91, 196, 227, 136, 237, 20, 174, 251, 117, 212, 91, 34, 240, 3, 131, 152, 72, 22, 208, 29, 101, 221, 151, 226, 134, 255, 211, 44],
        [159, 26, 91, 57, 7, 188, 49, 237, 121, 144, 115, 57, 147, 246, 200, 240, 15, 163, 217, 51, 204, 102, 115, 24, 61, 108, 210, 138, 224, 26, 224, 47],
        [122, 111, 152, 168, 16, 14, 202, 82, 72, 133, 213, 28, 57, 178, 64, 160, 4, 58, 202, 252, 110, 5, 87, 19, 48, 234, 78, 220, 229, 87, 141, 223]
    ]
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