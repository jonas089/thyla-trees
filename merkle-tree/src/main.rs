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

fn find_parent(root: MerkleNode, target: String) -> Option<MerkleNode>{
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
        let left_node = find_parent(*root.clone().left.unwrap(), target.clone());
        if !left_node.is_none(){
            return  left_node;
        }
    }

    if let Some(ref right) = root.right{
        let right_node = find_parent(*root.clone().right.unwrap(), target.clone());
        if !right_node.is_none(){
            return right_node;
        }
    }

    return None
}

#[test]
fn test(){
    assert_eq!(hash_string(String::from("hello")), String::from("2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824"));
    // would have to ensure always even:
    // if tx.len() % 0 != 0 => tx.append(tx[tx.len()])
    let transactions = vec![String::from("0x00"), String::from("0x01"), String::from("0x02"), String::from("0x03")];
    let merkle_tree = build_merkle_tree(transactions);
    println!("{:?}", merkle_tree);

    let tx_path_0x02 = find_leaf_path(merkle_tree.clone().unwrap(), String::from("0x02"), Vec::new());
    let tx_path_0x00 = find_leaf_path(merkle_tree.clone().unwrap(), String::from("0x00"), Vec::new());
    println!("0x02: {:?}", tx_path_0x02);
    println!("0x00: {:?}", tx_path_0x00);

    println!("{:?}", find_parent(merkle_tree.clone().unwrap(), String::from("0x02")));

    assert_eq!(merkle_tree.unwrap().data, hash_string(tx_path_0x00.unwrap()[1].clone() + &tx_path_0x02.unwrap()[1]));
}