use dep::std;
use dep::std::collections::vec::Vec;

pub fn verify_signature(message: [u8;32], pub_x: [u8;32], pub_y: [u8;32], signature: [u8;64]) -> bool{
    let message_hash = std::hash::sha256(message);
    let valid_signature = std::ecdsa_secp256k1::verify_signature(pub_x, pub_y, signature, message_hash);
    valid_signature
}

pub fn combine(left: [u8; 32], right: [u8; 32]) -> [u8]{
    let mut combined_vec: Vec<u8> = Vec::new();
    for byte in left{
        combined_vec.push(byte);
    };
    for byte in right{
        combined_vec.push(byte);
    };
    combined_vec.slice
}