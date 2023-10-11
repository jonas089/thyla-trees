# noir-rollup - a conceptual noir circuit for transaction rollups 

# This project is not done / not ready for use! 

The purpose of this repo is to explore potential zk rollup implementations in noir.

Circuit [here](https://github.com/jonas089/noir-rollup/blob/master/sig_verifier/src/main.nr)

# 1. Typeology

## 1.1. Public inputs

```Rust
    balances: [u64;n], 
    accounts: [[u8;32];n], 
    merkle_in: [u8;32], 
    sender_x: [[u8;32];n],
    sender_y: [[u8;32];n],
    recipient: [[u8;32];n],
    amount: [[u8]],
```

