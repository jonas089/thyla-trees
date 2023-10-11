mod helpers;
use helpers::{hash_sha, verify_signature, ascii_to_u64};
mod message;
use message::Message;
// Rollup state transitions
struct StateMachine{

    /* Old State */
        balances: [u64;2],
        accounts: [[u8;32];2],
        merkle_in: [u8;2],

    /* New Transactions */

        // sender x and y coordinates -> use x coordinate as pubkey
        sender_x: [[u8;32];1],
        sender_y: [[u8;32];1],
        // recipient hex string -> x coordinate of public key
        recipient: [[u8;32];1],
        // amount u64
        amount: [[u8;2];1],
        /* message for sig (oversimplified / missing timestamp):
            recipient + amount (+ timestamp) -> must in production ensure uniqueness
            * to ensure uniqueness, include nonces in the input balance data.
        */
        signature: [[u8;64];1]

    // tbd: public output? -> main return values? -> figure out when tests pass
}

impl StateMachine{
    fn build_message(self, index: u8) -> Message{
        let recipient = self.recipient[index];
        let amount = self.amount[index];

        let recipient_slice: [u8] = recipient.as_slice();
        let amount_slice: [u8] = amount.as_slice();
        let mut message: [u8] = recipient.as_slice();
        for byte in amount{
            message = message.push_back(byte);
        }
        //message.append(amount_slice);
        Message{
            index: index,
            recipient: self.recipient[index],
            amount: amount,
            message: message,
            message_hash: hash_sha(message)
        }
    }
    fn process_message(&mut self, message: Message) -> [u64;2]{
        // revert if a signature is invalid

        assert(
            verify_signature(
                message.message_hash, 
                self.sender_x[message.index], 
                self.sender_y[message.index],
                self.signature[message.index]
            )
        );

        //assert(ascii_to_u64(message.amount) == 10);
        let fixed_bytes: [u8;2] = [49, 48];
        let x = ascii_to_u64(fixed_bytes);
        assert(x == 10);
        let mut i: u8 = 0;
        let mut new_balances: [u64; 2] = [0,0];
        for a in self.accounts{
            let mut i: u8 = 0;
            for account in self.accounts{
                if account == message.recipient{
                    new_balances[i] = self.balances[i] + ascii_to_u64(message.amount);
                }
                else if account == self.sender_x[i]{
                    new_balances[i] = self.balances[i] - ascii_to_u64(message.amount);
                }
                else{
                    new_balances[i] = self.balances[i];
                }
                i += 1;
            };
            assert(new_balances[0] == 0);
            assert(new_balances[1] == 10);
            //let new_state: StateMachine = self.update_balance(new_balances);
        };
        new_balances
    }
    fn update_balance(self, balances: [u64;2]) -> StateMachine{
        // return new state machine with new_balances
        StateMachine{
            balances: balances,
            accounts: self.accounts,
            merkle_in: self.merkle_in,
            sender_x: self.sender_x,
            sender_y: self.sender_y,
            recipient: self.recipient,
            amount: self.amount,
            signature: self.signature
        }
    }
}
