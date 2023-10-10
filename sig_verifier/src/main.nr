use dep::std;
use dep::std::collections::vec::Vec;

/*
    To be done:
        + sender (!inf. bal)
        + timestamp

    Increases recipient balances, doesn't decrease sender
    balances.
*/
fn hash_sha(message: [u8]) -> [u8; 32]{
    std::hash::sha256(message)
}

fn verify_signature(message_hash : [u8;32], pub_key_x : [u8;32], pub_key_y : [u8;32], signature : [u8;64]) -> bool {
    // Single signature verification
    let message_rehash = std::hash::sha256(message_hash);
    let valid_signature = std::ecdsa_secp256k1::verify_signature(pub_key_x, pub_key_y, signature, message_rehash);
    //assert(valid_signature);
    valid_signature
}

fn main(
    balances: [u64], 
    accounts: [[u8]], 
    merkle_in: [u8], 
    sender_x: [[u8;32]], 
    sender_y: [[u8;32]],
    recipient: [[u8]],
    amount: [[u8]],
    // each return value as bytes
    signature: [[u8;64]]) -> pub[[u8]]{
    // use StateMachine to generate public outputs
    let state_machine = StateMachine{
        balances,
        accounts,
        merkle_in,
        sender_x,
        sender_y,
        recipient,
        amount,
        signature
    };
    let message: Message = state_machine.build_message(0);
    [[0]]
}

struct Message{
    index: u8,
    recipient: [u8],
    amount: [u8],
    message: [u8],
    message_hash: [u8; 32]
}

// Rollup state transitions
struct StateMachine{

    /* Old State */

        // list of balances (private)
        balances: [u64],
        // corresponding accounts b[i] -> a[i] (private)
        accounts: [[u8]],
        // input merkle hash (oracle) (public)
        merkle_in: [u8],

    /* New Transactions */

        // sender x and y coordinates
        sender_x: [[u8; 32]],
        sender_y: [[u8; 32]],

        // recipient hex string
        recipient: [[u8]],
        // amount u64
        amount: [[u8]],
        /* message for sig (oversimplified / missing timestamp):
            recipient + amount 
        */
        signature: [[u8;64]]

    // tbd: public output? -> main return values? -> figure out when tests pass
}

impl StateMachine{
    fn build_message(self, index: u8) -> Message{
        //let x: [[u8]] = [[]];
        let recipient = self.recipient[index];
        let amount = self.amount[index];
        let message = recipient.append(amount);
        Message{
            index: index,
            recipient: recipient,
            amount: amount,
            message: message,
            message_hash: hash_sha(message)
        }
    }
    fn process_message(self, message: Message){

    }
}


#[test]
fn stateMachine(){
    /*
        balances,
        accounts,
        merkle_in,
        sender_x,
        sender_y,
        recipient,
        amount,
        signature
    */
    main(
        [0, 0],
        // this would be hashs fo fixed length e.g. account-hashs / addresses / pubkey o.e.
        [[0], [1]],
        // merkle hash (anything)
        [123, 123],
        [[57, 141, 121, 60, 115, 189, 115, 103, 174, 6, 108, 20, 114, 134, 156, 80, 7, 222, 7, 107, 196, 2, 216, 251, 119, 174, 151, 31, 19, 46, 39, 92]],
        [[207, 92, 161, 239, 169, 234, 3, 35, 59, 37, 172, 77, 65, 179, 24, 23, 210, 215, 112, 148, 177, 230, 247, 201, 131, 225, 233, 194, 143, 189, 239, 9]],
        [[20]],
        [[10]],
        [[231, 122, 163, 143, 93, 12, 18, 132, 138, 215, 111, 46, 67, 202, 74, 30, 1, 182, 92, 92, 81, 117, 227, 209, 150, 177, 116, 185, 27, 220, 170, 134, 32, 74, 210,
        119, 169, 78, 210, 35, 203, 151, 209, 89, 232, 67, 39, 113, 78, 155, 34, 147, 86, 152, 75, 154, 158, 220, 35, 12, 255, 18, 23, 103]]
    );
}
    
    
    
#[test]
fn signatures() {
    // Test Signature
    let is_valid: bool = verify_signature(
        [223, 253, 96, 33, 187, 43, 213, 176, 175, 103, 98, 144, 128, 158, 195, 165, 49, 145, 221, 129, 199, 247, 10, 75, 40, 104, 138, 54, 33, 130, 152, 111],
        [77, 154, 126, 232, 196, 225, 168, 59, 92, 209, 131, 12, 75, 100, 207, 199, 5, 43, 99, 228, 204, 68, 6, 215, 158, 218, 115, 138, 189, 163, 55, 245],
        [16, 228, 74, 23, 195, 86, 207, 3, 197, 97, 183, 204, 88, 193, 180, 218, 193, 183, 83, 215, 24, 137, 139, 93, 43, 143, 42, 151, 214, 102, 243, 75],
        [230, 120, 141, 193, 123, 240, 107, 68, 229, 28, 43, 118, 121, 95, 209, 71, 158, 89, 0, 244, 242, 87, 223, 154, 39, 236, 163, 8, 122, 66, 225, 21, 38, 48, 182, 148, 244, 248, 85, 165, 224, 231, 211, 166, 42, 170, 118, 153, 140, 184, 20, 128, 195, 182, 233, 102, 181, 197, 208, 199, 240, 171, 150, 87]
    );
    assert(is_valid);
}
    
