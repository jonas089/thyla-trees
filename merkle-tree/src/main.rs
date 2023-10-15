extern crate sha256;
use sha256::{digest, try_digest};

fn main(){
    panic!("Run cargo test -- --nocapture instead!");
}

fn hash_string(input: String) -> String{
    digest(input)
}

#[derive(Debug, Clone)]
struct MerkleNode{
    data: String,
    left: Option<Box<MerkleNode>>,
    right: Option<Box<MerkleNode>>
}
impl MerkleNode{
    fn is_empty(self) -> bool{
        if self.data.len() == 0{
            return true;
        }
        false
    }
}

fn build_merkle_tree(tx: Vec<String>) -> Option<MerkleNode>{
    if tx.is_empty(){
        return None;
    }
    else if tx.len() == 1{
        return Some(MerkleNode{
            data: tx[0].clone(),
            left: None,
            right: None
        });
    }
    let mid: usize = tx.len() / 2;
    let left_subtree = build_merkle_tree(tx[..mid].to_vec());
    let right_subtree = build_merkle_tree(tx[mid..].to_vec());
    let mut node = MerkleNode{
        data: hash_string(left_subtree.as_ref().unwrap().data.clone() + &right_subtree.as_ref().unwrap().data.clone()),
        left: None,
        right: None
    };
    node.left = left_subtree.map(Box::new);
    node.right = right_subtree.map(Box::new);
    Some(node)
}

fn find_leaf_path(root: MerkleNode, target: String, path: Vec<String>) -> Option<Vec<String>>{
    if root.clone().is_empty(){
        return None;
    }
    if root.data == target{
        return Some(path);
    }

    let mut path_cp_left = path.clone();
    path_cp_left.push(root.data.clone());
    let mut path_cp_right = path.clone();
    path_cp_right.push(root.data.clone());

    if let Some(ref left) = root.left{
        let left_path = find_leaf_path(*root.clone().left.unwrap(), target.clone(), path_cp_left);
        if !left_path.is_none(){
            return left_path;
        }
    }

    if let Some(ref right) = root.right{
        let right_path = find_leaf_path(*root.clone().right.unwrap(), target, path_cp_right);
        if !right_path.is_none(){
            return right_path;
        }
    }

    return None;
}

fn find_leaf_parent(root: MerkleNode, target: String) -> Option<MerkleNode>{
    // check if target in children
    if let Some(ref left) = root.left{
        if root.clone().left.unwrap().data == target{
            return Some(root);
        }
    }
    if let Some(ref right) = root.right{
        if root.clone().right.unwrap().data == target{
            return Some(root);
        }
    }
    /*
    if root.clone().left.unwrap().data == target || root.clone().right.unwrap().data == target{
        // return node with child
        return Some(root);
    };
    */
    if let Some(ref left) = root.left{
        let left_node = find_leaf_parent(*root.clone().left.unwrap(), target.clone());
        if !left_node.is_none(){
            return  left_node;
        }
    }

    if let Some(ref right) = root.right{
        let right_node = find_leaf_parent(*root.clone().right.unwrap(), target.clone());
        if !right_node.is_none(){
            return right_node;
        }
    }

    return None
}

fn find_leaf_sibling(root: MerkleNode, target: String) -> Option<MerkleNode>{
    if let Some(ref left) = root.left{
        if root.clone().left.unwrap().data == target{
            return Some(*root.clone().right.unwrap());
        }
        else{
            return Some(*root.clone().left.unwrap());
        }
    }
    else if let Some(ref right) = root.right{
        if root.clone().left.unwrap().data == target{
            return Some(*root.clone().right.unwrap());
        }
        else{
            return Some(*root.clone().left.unwrap());
        } 
    }
    else{
        return None;
    }
}

fn verify_merkle_proof(merkle_root: String, mut proof_path: Vec<String>, ls: bool) -> bool {
    // Ensure the proof_path has an even number of elements
    if proof_path.len() % 2 != 0 {
        panic!("Proof path is not even!");
        return false;
    }
    
    let mut current_hash = String::new();
    while !proof_path.is_empty() {
        let sibling = proof_path.pop().unwrap_or_default();
        let node = proof_path.pop().unwrap_or_default();
        if ls == true{
            current_hash = hash_string(node + &sibling);   
        }
        else {
            current_hash = hash_string(sibling + &node);
        }
    }
    //assert_eq!(merkle_root, current_hash);
    merkle_root == current_hash
}

#[test]
fn test(){
    // would have to ensure always even:
    // if tx.len() % 0 != 0 => tx.append(tx[tx.len()])

    /*
    let transactions = vec![String::from("0x00"), String::from("0x01"), String::from("0x02"), String::from("0x03"), String::from("0x04"), String::from("0x05"), String::from("0x06"), String::from("0x07")];
    let merkle_tree = build_merkle_tree(transactions);
    println!("[DEBUG] {:?}", merkle_tree);

    let tx_path_0x02 = find_leaf_path(merkle_tree.clone().unwrap(), String::from("0x02"), Vec::new());
    let tx_path_0x00 = find_leaf_path(merkle_tree.clone().unwrap(), String::from("0x00"), Vec::new());
    println!("[DEBUG] 0x02: {:?}", tx_path_0x02);
    println!("[DEBUG] 0x00: {:?}", tx_path_0x00);

    println!("[DEBUG] {:?}", find_leaf_parent(merkle_tree.clone().unwrap(), String::from("0x02")));

    // test sibling matcher
    let parent_0x00 = find_leaf_parent(merkle_tree.clone().unwrap(), String::from("0x00"));
    let parent_0x02 = find_leaf_parent(merkle_tree.clone().unwrap(), String::from("0x02"));
    let sibling_0x00 = find_leaf_sibling(parent_0x00.unwrap(), String::from("0x00")).unwrap();
    let sibling_0x02 = find_leaf_sibling(parent_0x02.unwrap(), String::from("0x02")).unwrap();

    println!("[DEBUG] Sibling 0x00: {:?}", sibling_0x00);
    println!("[DEBUG] Sibling 0x02: {:?}", sibling_0x02);
    */
    // try to prove a transaction's inclusion
    /*
    let transactions = vec![String::from("0x00"), String::from("0x01"), String::from("0x02"), String::from("0x03"), String::from("0x04"), String::from("0x05"), String::from("0x06"), String::from("0x07")];
    let merkle_tree = build_merkle_tree(transactions);

    let merkle_root = String::from("7fecc42e1d62c53d6fe0cb9d35a66fef81be9d9c137d7e6808744d71d2730055");
    let id: String = String::from("0x00");
    let path: Vec<String> = find_leaf_path(merkle_tree.clone().unwrap(), id.clone(), Vec::new()).unwrap();
    let mut proof_path: Vec<String> = Vec::new();
    // enumerate and skip root
    for leaf in &path[1..]{
        proof_path.push(String::from(leaf.clone()));
        let leaf_parent = find_leaf_parent(merkle_tree.clone().unwrap(), String::from(leaf.clone()));
        let leaf_sibling = find_leaf_sibling(leaf_parent.clone().unwrap(), String::from(leaf.clone()));
        proof_path.push(leaf_sibling.unwrap().data);
    }
    println!("Proof path: {:?}", proof_path);
    assert_eq!(hash_string(proof_path[2].clone() + &proof_path[3]), proof_path[0]);
    assert_eq!(hash_string(proof_path[0].clone() + &proof_path[1]), merkle_root);

    assert_eq!(proof_path.len(), 4);
    println!("Verification fn result: {:?}", verify_merkle_proof(merkle_root.clone(), proof_path.clone()));
    assert_eq!(verify_merkle_proof(merkle_root, proof_path), true);
    */

    let mut transactions: Vec<String> = Vec::new();
    let mut ids: Vec<String> = Vec::new();
    for i in 0..100{
        let _id = format!("0x{}", i.to_string());
        transactions.push(_id.clone());
        ids.push(_id);
    }
    let merkle_tree = build_merkle_tree(transactions.clone());
    let merkle_root = merkle_tree.clone().unwrap().data;
    for (index, id) in ids.iter().enumerate(){
        let path: Vec<String> = find_leaf_path(merkle_tree.clone().unwrap(), id.clone(), Vec::new()).unwrap();
        let mut proof_path: Vec<String> = Vec::new();
        // enumerate and skip root
        for leaf in &path[1..]{
            proof_path.push(String::from(leaf.clone()));
            let leaf_parent = find_leaf_parent(merkle_tree.clone().unwrap(), String::from(leaf.clone()));
            let leaf_sibling = find_leaf_sibling(leaf_parent.clone().unwrap(), String::from(leaf.clone()));
            proof_path.push(leaf_sibling.unwrap().data);
        };
        let ls = {
            if index < ids.len() / 2{
                true
            }
            else {
                false
            }
        };
        if verify_merkle_proof(merkle_root.clone(), proof_path, ls) == false{
            println!("Failed to verify: {:?}", id);
        }
        else{
            println!("Verified: {:?}", id);
        }
        //assert_eq!(verify_merkle_proof(merkle_root.clone(), proof_path), true);
    }

    let mut proof_path: Vec<String> = Vec::new();
    println!("Full merkle tree: {:?}", &merkle_tree);
    let test_path = find_leaf_path(merkle_tree.clone().unwrap(), String::from("0x1"), Vec::new()).unwrap();
    for leaf in &test_path[1..]{
        proof_path.push(String::from(leaf.clone()));
        let leaf_parent = find_leaf_parent(merkle_tree.clone().unwrap(), String::from(leaf.clone()));
        let leaf_sibling = find_leaf_sibling(leaf_parent.clone().unwrap(), String::from(leaf.clone()));
        proof_path.push(leaf_sibling.unwrap().data);
    };
    println!("Full proof path: {:?}", &proof_path);
    println!("{:?}", hash_string(proof_path[0].clone() + &proof_path[1]) )
}



/*

Full merkle tree: Some(MerkleNode { data: "00ada7f0393fced15bbb1fa02b200e487d1ea2562e63acff56ad8a753de9f981", 
left: Some(MerkleNode { data: "cc68c6ed4c7ec3ee1340b3227035ad94e33cf9a7a59345af0a5a49ee1723dcad", 
left: Some(MerkleNode { data: "0x0", left: None, right: None }), 
right: Some(MerkleNode { data: "0x1", left: None, right: None }) }), 
right: Some(MerkleNode { data: "7e844584f83a63208c5bb8851057910d2040eb253de53bce3057c33286270f7c", 
left: Some(MerkleNode { data: "0x2", left: None, right: None }), 
right: Some(MerkleNode { data: "0x3", left: None, right: None }) }) })
Full proof path: ["7e844584f83a63208c5bb8851057910d2040eb253de53bce3057c33286270f7c", "cc68c6ed4c7ec3ee1340b3227035ad94e33cf9a7a59345af0a5a49ee1723dcad"]
"b362b7973f7361652ace62f053ffb6ef485911434b2fcfd1cec0717be840b8bd"

Full proof path: ["cc68c6ed4c7ec3ee1340b3227035ad94e33cf9a7a59345af0a5a49ee1723dcad", "7e844584f83a63208c5bb8851057910d2040eb253de53bce3057c33286270f7c"]
"00ada7f0393fced15bbb1fa02b200e487d1ea2562e63acff56ad8a753de9f981"

    merkle_root
 7e8        cc68
0x00 0x01 0x02 0x03

*/