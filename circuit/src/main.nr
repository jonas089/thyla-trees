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

fn ascii_to_u64(arr: [u8]) -> u64 {
    let mut result: u64 = 0;
    let mut multiplier: u64 = 1;
    let len = arr.len();

    for i in 0..len {
        // Convert ASCII to its integer equivalent
        let int_val = (arr[len - i - 1] - 48) as u64;

        // Multiply by the position's power of 10
        result += int_val * multiplier;
        
        // Increase the multiplier for the next iteration
        multiplier *= 10;
    }

    result
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
    let x: [u8] = [106, 111, 110, 97, 115, 49, 48];
    state_machine.process_message(message);
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

        // sender x and y coordinates -> use x coordinate as pubkey
        sender_x: [[u8; 32]],
        sender_y: [[u8; 32]],

        // recipient hex string -> x coordinate of public key
        recipient: [[u8]],
        // amount u64
        amount: [[u8]],
        /* message for sig (oversimplified / missing timestamp):
            recipient + amount (+ timestamp) -> must in production ensure uniqueness

            * to ensure uniqueness, include nonces in the input balance data.
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
        // revert if a signature is invalid

        assert(
            verify_signature(
                message.message_hash, 
                self.sender_x[message.index], 
                self.sender_y[message.index],
                self.signature[message.index]
            )
        );

        assert(ascii_to_u64(message.amount) == 10);
        // apply state transition to balance and merkle
        // self.balances[recipient] -= 10 
        // compute new merkle
        // ...

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
        [10, 0],
        // this would be hashs fo fixed length e.g. account-hashs / addresses / pubkey o.e.
        [[76, 195, 113, 207, 31, 191, 143, 106, 36, 227, 255, 61, 28, 142, 167, 76, 31, 82, 16, 250, 167, 114, 101, 243, 133, 87, 115, 12, 12, 19, 26, 177], [76, 195, 113, 207, 31, 191, 143, 106, 36, 227, 255, 61, 28, 142, 167, 76, 31, 82, 16, 250, 167, 114, 101, 243, 133, 87, 115, 12, 12, 19, 26, 178]],
        // merkle hash (anything)
        [123, 123],
        // x-coordinate
        [[76, 195, 113, 207, 31, 191, 143, 106, 36, 227, 255, 61, 28, 142, 167, 76, 31, 82, 16, 250, 167, 114, 101, 243, 133, 87, 115, 12, 12, 19, 26, 177]],
        // y-coordinate
        [[100, 235, 51, 68, 137, 178, 110, 103, 129, 0, 95, 88, 149, 40, 157, 116, 102, 11, 63, 87, 202, 234, 39, 133, 127, 99, 197, 208, 167, 89, 29, 199]],
        // recipient bytes
        [[57, 141, 121, 60, 115, 189, 115, 103, 174, 6, 108, 20, 114, 134, 156, 80, 7, 222, 7, 107, 196, 2, 216, 251, 119, 174, 151, 31, 19, 46, 39, 92]],
        // amount bytes
        [[49, 48]],
        // signature bytes
        [[36, 242, 254, 140, 255, 254, 247, 61, 0, 116, 57, 148, 101, 115, 255, 56, 4, 30, 31, 139, 33, 45, 127, 73, 244, 136, 245, 205, 152, 214, 141, 20, 58, 40, 206, 47, 15, 120, 43, 70, 209, 148, 98, 207, 145, 15, 2, 175, 218, 253, 169, 230, 96, 91, 187, 191, 202, 63, 10, 144, 189, 85, 109, 169]]
    );
    /*
    * message contains:
        * recipient[u8].append(amount[u8])
    */
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
    
