# Zero knowledge transaction rollup written in Rust and Noir


# Rust Merkle tree
Found in `merkle-tree/src/main.rs`

To generate a merkle proof for a transaction using the default configuration, run:

```bash
    cargo test produce_merkle_proof -- --nocapture
```

This command will output all information required by the prover.

Example output:

```
Transaction to prove: [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
Sibling #0: [2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2] : false
Sibling #1: [178, 137, 222, 169, 44, 165, 171, 165, 242, 225, 137, 26, 26, 241, 27, 226, 121, 20, 196, 136, 84, 219, 15, 229, 180, 187, 149, 193, 55, 224, 242, 214] : false
Sibling #2: [248, 211, 204, 204, 180, 196, 230, 213, 226, 254, 251, 255, 140, 104, 170, 245, 141, 86, 82, 142, 59, 109, 142, 191, 7, 180, 33, 12, 239, 230, 161, 241] : false
Sibling #3: [175, 132, 242, 248, 185, 9, 188, 62, 34, 213, 240, 199, 176, 177, 75, 99, 187, 215, 70, 226, 72, 67, 45, 66, 103, 218, 50, 31, 1, 52, 216, 168] : false
Merkle root: [122, 111, 152, 168, 16, 14, 202, 82, 72, 133, 213, 28, 57, 178, 64, 160, 4, 58, 202, 252, 110, 5, 87, 19, 48, 234, 78, 220, 229, 87, 141, 223]
```

## Left / Right
`false` or `true` answers the question `is_left`. If `true`, the sibling is on the left side and if `false` the sibling is on the right side of the tree (relative to current position in the tree). 

L/R is relevant information for the prover. The L/R information for each level is passed as either `0` or `1` as part of the `positions` input of the circuit.

## Tree Depth

The default depth of the merkle tree is 5. The depth can be adjusted by overriding the `DEFAULT_DEPTH` constant in `merkle-tree/src/config.rs`

## Optimization

Merkle tree functions can be further optimized to support a higher transaction throughput.

# Noir circuit

The circuit to prove merkle paths can be found in `circuits/merkleproofs/src/main.rs`. It takes as input the following information:

```
1. The transaction to be proven
2. Siblings for each level in the tree
3. Position of each sibling for each level in the tree
4. Current merkle root
```

To run the circuit with the sample input from the sample tree, run:

```
nargo test
```

in the circuit directory.

# What the noir proof means

A proof verified by this circuit implies a high probability that one knows a valid transaction in the given merkle tree with the provided merkle root.


# L2 rollup concept

## Rollup circuit
The rollup circuit can generate proofs for the merkle path and the validity of message signatures.

A chunk of transactions is processed and the proof and root hash are committed.

