use k256::{
    ecdsa::{SigningKey, Signature, signature::Signer},
    SecretKey,
};
use rand_core::OsRng;


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
    let message = b"ECDSA proves knowledge of a secret number in the context of a single message";
    todo!("Sha256 of message for fixed len!");
    
    let signature: Signature = signing_key.sign(message);
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

    println!("Message bytes: {:?}", message);
}


#[test]
fn test(){
    generate_signature_inputs();

}