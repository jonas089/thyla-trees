use std::thread::current;

use crate::helpers::{hashLeftRight, hash_bytes};

#[derive(Debug, Clone, PartialEq)]
struct TornadoTree{
    zero_node: Vec<u8>,
    zero_levels: Vec<Vec<u8>>,
    root_history: Vec<Vec<u8>>,
    filled: Vec<Vec<u8>>,
    index: usize,
    depth: usize
}

impl TornadoTree{
    fn calculate_zero_levels(&mut self){
        let mut zero_levels: Vec<Vec<u8>> = vec![self.zero_node.clone()];
        for i in 0..self.depth - 1{
            zero_levels.push(hashLeftRight(zero_levels[zero_levels.len()-1].clone(), zero_levels[zero_levels.len()-1].clone()));
        };
        self.zero_levels = zero_levels;
    }
    fn add_leaf(&mut self, leaf: Vec<u8>) {
        let mut current_index = self.index;
        let mut current_hash = leaf.clone();

        for i in 0..self.depth {
            if current_index % 2 == 0 {
                // If index is even, left is the new leaf or its hash, right is the zero node at this level
                self.filled[i] = current_hash.clone(); // push the leaf or its hash
                current_hash = hashLeftRight(current_hash, self.zero_levels[i].clone());
            } else {
                // If index is odd, left is the previous leaf or its hash at this level, right is the new leaf or its hash
                let left = self.filled[i].clone();
                current_hash = hashLeftRight(left, current_hash);
            }
            current_index /= 2;
        }

        self.index += 1;
    }
}

#[test]
fn test_tornado(){
    let mut tree = TornadoTree{
        zero_node: hash_bytes(vec![0;32]),
        zero_levels: Vec::new(),
        root_history: Vec::new(),
        filled: vec![vec![], vec![], vec![]],
        index: 0,
        depth: 3
    };
    tree.calculate_zero_levels();
    tree.add_leaf(vec![1;32]);
    tree.root_history.push(tree.filled[tree.filled.len() - 1].clone());
    tree.add_leaf(vec![2;32]);
    tree.root_history.push(tree.filled[tree.filled.len() - 1].clone());
    tree.add_leaf(vec![3;32]);
    tree.root_history.push(tree.filled[tree.filled.len() - 1].clone());
    //tree.add_leaf(vec![4;32]);
    println!("Tree: {:?}", tree.filled);

    println!("Root history: {:?}", &tree.root_history);
}

/*
- If the index is even, the sibling is the next node (index + 1) at the same level.
- If the index is odd, the sibling is the previous node (index - 1).
*/

/* Information needed for merkle proof
- index of leaf
- sibling at each level

- if leaf index is even, check
*/