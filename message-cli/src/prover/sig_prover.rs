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

    
}