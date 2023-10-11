# noir-rollup - a conceptual noir circuit for transaction rollups 

# This project is not done / not ready for use! 

The purpose of this repo is to explore potential zk rollup implementations in noir.

Circuit [here](https://github.com/jonas089/noir-rollup/blob/master/circuit/src/main.nr)

# 1. Typeology

## 1.1. Public inputs

```Rust
    balances: [u64;n], 
    accounts: [[u8;32];n], 
    merkle_in: [u8;32], 
```

## 1.2. Private inputs


⚠️ **WARNING:** For development and testing some datatypes are changed temporarily


```Rust
    sender_x: [[u8;32];n],
    sender_y: [[u8;32];n],
    recipient: [[u8;32];n],
    amount: [[u8;8];n],
    // or amount: [[u64;1];n]
```

## 1.2. Public outputs
```Rust
    /* To be defined
        * public outputs will include
            * new merkle root
            * new balances
        -> return statement
            * [[T;n];n] or [[[T;n];n];n]
    */
```

## 2. Structs

## StateMachine (reactor)

Represents all state (public and private inputs)

Methods:

```Rust
build_message
process_message
update_balance
```

1. build_message

`Uses message data to construct the inputs to the signature verifier (for a given transaction at index i).`

2. process_message

`Applies state transitions according to the message data (includes index) -> transactions are processed one-by-one.`
`Will revert should an invalid signature appear.`

3. update_balance

`returns a new StateMachine with the updated balance infromation.`

# Use with Nargo client

0. Run tests

```bash
cd circuit
nargo test
```

1. Build the circuit

```bash
cd circuit
nargo check
```

2. Provide circuit Inputs -> edit Prover.toml

3. Generate a proof

```bash
cd circuit
nargo prove
```

4. verify a proof

```bash
cd circuit
nargo verify
```


