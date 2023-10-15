use dep::std;
mod reactor;
use reactor::{StateMachine, message::Message};

fn main(
    balances: pub [u64;2], 
    accounts: pub [[u8;32];2],
    sender_x: [[u8;32];1],
    sender_y: [[u8;32];1],
    recipient: [[u8;32];1],
    amount: [[u8;2];1],
    // each return value as bytes
    signature: [[u8;64];1]) -> pub[[u8;1];1]{
    // use StateMachine to generate public outputs
    let mut state_machine = StateMachine{
        balances,
        accounts,
        sender_x,
        sender_y,
        recipient,
        amount,
        signature
    };
    let message: Message = state_machine.build_message(0);
    state_machine.process_message(message);
    [[0]]
}


#[test]
fn stateMachine(){
    /*
        balances,
        accounts,
        sender_x,
        sender_y,
        recipient,
        amount,
        signature
    */
    main(
        [10, 0],
        // accounts
        [[76, 195, 113, 207, 31, 191, 143, 106, 36, 227, 255, 61, 28, 142, 167, 76, 31, 82, 16, 250, 167, 114, 101, 243, 133, 87, 115, 12, 12, 19, 26, 177], [57, 141, 121, 60, 115, 189, 115, 103, 174, 6, 108, 20, 114, 134, 156, 80, 7, 222, 7, 107, 196, 2, 216, 251, 119, 174, 151, 31, 19, 46, 39, 92]],
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
}
