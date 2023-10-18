mod experiment;
use experiment::hash_string;
fn main(){
    panic!("Run cargo test -- --nocapture instead!");
}

/* Program outline
    * construct an empty merkle tree of fixed size
    * hash each level and store each level's hash as a fixed const index (similar to tornadocash)
    * replace the first empty node in the tree with a new entry
    * re-construct the merkle tree (-> make use of the constants)
    * implement verification logic similar to what exists in 'experiment.rs'
*/

#[derive(Debug, Clone)]
struct MerkleNode{
    data: String,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>
}

fn build_merkle_tree(tx: Vec<String>) -> Option<MerkleNode> {
    if tx.is_empty() {
        return None;
    }
    let mut nodes = tx.into_iter()
        .map(|t| MerkleNode {
            data: t,
            left: None,
            right: None,
        })
        .collect::<Vec<_>>();
    while nodes.len() > 1 {
        if nodes.len() % 2 != 0 {
            let last = nodes.last().unwrap().clone();
            nodes.push(last);
        }
        nodes = nodes.chunks(2).map(|pair| {
            let left = Box::new(pair[0].clone());
            let right = Box::new(pair[1].clone());
            MerkleNode {
                data: hash_string(format!("{}{}", left.data, right.data)),
                left: Some(left),
                right: Some(right),
            }
        }).collect();
    }
    nodes.pop()
}

#[test]
fn test(){
    // new merkle tree that'll support 1000 transactions
    let zeros: Vec<String> = vec!["0".to_string(); 1000];
    
}