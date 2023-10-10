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

        // apply state transition to balance and merkle
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
        [0, 0],
        // this would be hashs fo fixed length e.g. account-hashs / addresses / pubkey o.e.
        [[0], [1]],
        // merkle hash (anything)
        [123, 123],
        [[78, 8, 95, 216, 33, 147, 128, 182, 64, 31, 224, 242, 58, 103, 252, 133, 230, 166, 156, 217, 100, 16, 65, 154, 159, 255, 218, 188, 0, 241, 180, 124]],
        [[79, 249, 47, 68, 25, 183, 132, 96, 80, 42, 47, 144, 144, 175, 162, 20, 129, 131, 49, 23, 190, 31, 66, 133, 235, 18, 7, 195, 120, 93, 27, 224]],
        [[51, 57, 56, 100, 55, 57, 51, 99, 55, 51, 98, 100, 55, 51, 54, 55, 97, 101, 48, 54, 54, 99, 49, 52, 55, 50, 56, 54, 57, 99, 53, 48, 48, 55, 100, 101, 48, 55, 54, 98, 99, 52, 48, 50, 100, 56, 102, 98, 55, 55, 97, 101, 57, 55, 49, 102, 49, 51, 50, 101, 50, 55, 53, 99]],
        [[49, 48]],
        [[42, 162, 112, 253, 76, 219, 177, 129, 112, 179, 103, 30, 2, 191, 229, 249, 160, 4, 46, 99, 23, 236, 118, 242, 114, 3, 245, 252, 12, 254, 41, 80, 70, 101, 91, 223, 231, 33, 70, 42, 139, 28, 255, 190, 237, 238, 54, 8, 163, 148, 35, 73, 154, 119, 7, 110, 93, 176, 223, 125, 182, 87, 239, 149]]
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
    
