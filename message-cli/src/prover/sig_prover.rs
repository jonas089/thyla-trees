use k256::{
    ecdsa::{SigningKey, Signature, signature::Signer},
    SecretKey,
};
use rand_core::OsRng;
use sha2::{Digest, Sha256};


fn generate_signature_inputs(){
    /* What data is signed?
    recipient + amount + nonce + timestamp

    for now reduce to:
    recipient + amount,
    where amount is u64

    nonce will also be a u64 and ideally the timestamp will be a u64 aswell

    => problemspace
    convert an a u64 of any size to a u8[] and back.
    */
    // Signing
    let signing_key = SigningKey::random(&mut OsRng); // Serialize with `::to_bytes()`

    let message = "jonas10";
    let mut sha = Sha256::new();
    sha.update(message);
    let hashed_message = sha.finalize();
    let signature: Signature = signing_key.sign(&hashed_message);
    // Verification
    use k256::{EncodedPoint, ecdsa::{VerifyingKey, signature::Verifier}};
    let verifying_key = VerifyingKey::from(&signing_key); // Serialize with `::to_encoded_point()`
    //assert!(verifying_key.verify(message, &signature).is_ok());

    let verifying_key_point = verifying_key.to_encoded_point(false);
    let verifying_key_x = verifying_key_point.x().unwrap().as_slice();
    let verifying_key_y = verifying_key_point.y().unwrap().as_slice();

    println!("X: {:?}", verifying_key_x);
    println!("Y: {:?}", verifying_key_y);
    println!("Sig: {:?}", signature.to_bytes());

    println!("Message bytes: {:?}", &hashed_message);
    println!("Message length: {:?}", &hashed_message.len());
}


#[test]
fn test(){
    generate_signature_inputs();
}