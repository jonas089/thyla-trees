struct Message{
    index: u8,
    recipient: [u8;32],
    amount: [u8;2],
    message: [u8],
    message_hash: [u8;32]
}