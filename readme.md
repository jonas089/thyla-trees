# noir-rollup - a conceptual noir circuit for transaction rollups 

# This project is not done / not ready for use! 

The purpose of this repo is to explore potential zk rollup implementations in noir.

Circuit [here](https://github.com/jonas089/noir-rollup/tree/master/circuit/src)

# Slick implementation

Merkle path proof design:

```
Merkle path consists of nodes and sibling nodes for each transaction.
Proof is generated for each transaction's inclusion in the merkle tree.
For that every transaction's merkle path and the merkle root are required.
```

Experimental merkle tree [code](https://github.com/jonas089/noir-rollup/blob/master/merkle-tree/src/main.rs)

❗ How is output state proven / how are balances calculated?


# Suboptimal implementation

```
Circuit takes all new transaction data and accounting state
as input. Constraint system does not support dynamic datatypes.
Therefore it's not feasible to implement a circuit for a
growing database of accounts.

Prove inclusion in a merkle tree instead.
```

suboptimal [code](https://github.com/jonas089/noir-rollup/tree/master/circuits/experiments)

Problems when taking all transactions and state as input:
```
❄️ All inputs to the circuit must be of fixed length
    ❄️ pub_x ✅ 
    ❄️ pub_y ✅ 
    ❄️ recipients ✅ 
    ❄️ signatures ✅ 
    ❄️ amounts✅  
    ❄️ balances ❌
    ❄️ accounts ❌
accounts => balances 1:1
```
❗ The amount of accounts supported by this circuit currently needs to be fixed.

❗ This is not suitable for a real-world transaction system.



# 1. Typeology

## 1.1. Public inputs

```Rust
    balances: [u64;n], 
    accounts: [[u8;32];n]
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

Represents state (public and private inputs)

Methods:

```Rust
build_message
process_message
update_balance
```

1. build_message

```
Uses message data to construct the inputs to the signature verifier (for a given transaction at index i).
```

2. process_message

```
Applies state transitions according to the message data (includes index) -> transactions are processed one-by-one.
Will revert should an invalid signature appear.
```

3. update_balance

```
returns a new StateMachine with the updated balance infromation.
```

## Message

Represents a transaction

```Rust
    index: u8,
    recipient: [u8;32],
    amount: [u8;2],
    message: [u8],
    message_hash: [u8;32]
```

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
