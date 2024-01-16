use crate::helpers::hashLeftRight;
//use crate::error::MerkleTreeError;

#[derive(Debug, Clone, PartialEq)]
pub struct MerkleNode{
    pub data: Vec<u8>,
    pub left: Option<Box<MerkleNode>>,
    pub right: Option<Box<MerkleNode>>
}

#[derive(Debug, Clone, PartialEq)]
pub struct MerkleTree{
    pub root: Option<MerkleNode>,
}

impl MerkleTree{
    pub fn build(&mut self, leafs: Vec<Vec<u8>>){
        let mut current_level: Vec<MerkleNode> = leafs
            .iter()
            .map(|leaf| MerkleNode{data: leaf.to_owned(), left: None, right: None})
            .collect();
        current_level.reverse();
        while current_level.len() > 1{
            let mut next_level: Vec<MerkleNode> = Vec::new();
            if current_level.len() % 2 != 0{
                current_level.push(MerkleNode{
                    data: vec![3, 19, 16, 18],
                    left: None,
                    right: None
                });
            }
            while current_level.len() > 1{
                let left: MerkleNode = current_level.pop().expect("Missing left node!");
                let right: MerkleNode = current_level.pop().expect("Missing left node!");
                next_level.push(
                    MerkleNode{
                        data: hashLeftRight(left.clone().data, right.clone().data),
                        left: Some(Box::new(left)),
                        right: Some(Box::new(right))
                    }
                );
            };
            current_level = next_level.clone();
        }
        self.root = current_level.pop();
    }
    pub fn discover_sibling(&self, parent: &MerkleNode, target: &Vec<u8>) -> Option<(Option<MerkleNode>, u8)>{
        if let Some(ref left) = parent.left{
            if parent.clone().left.unwrap().data == target.to_vec(){
                return Some((Some(*parent.clone().right.unwrap()), 0));
            }
            else{
                return Some((Some(*parent.clone().left.unwrap()), 1));
            }
        }
        else{
            return None;
        }
    }
    pub fn discover_parent(&self, node: &MerkleNode, target: &Vec<u8>) -> Option<MerkleNode> {
        if let Some(ref left) = node.left {
            if left.data == *target {
                return Some(node.clone());
            }
        }
        if let Some(ref right) = node.right {
            if right.data == *target {
                return Some(node.clone());
            }
        }
        if let Some(ref left) = node.left {
            if let Some(found) = self.discover_parent(left, target) {
                return Some(found);
            }
        }
        if let Some(ref right) = node.right {
            if let Some(found) = self.discover_parent(right, target) {
                return Some(found);
            }
        }
        None
    }
}

#[test]
fn binary_merkle_tree(){
    let mut leafs: Vec<Vec<u8>> = Vec::new();
    for i in 0..255{
        leafs.push(vec![0,0,i]);
    };
    let mut tree: MerkleTree = MerkleTree{
        root: None
    };
    tree.build(leafs);
    let root: Vec<u8> = tree.clone().root.unwrap().data;
    let mut path: Vec<(Vec<u8>, u8)> = Vec::new();
    let mut target: Vec<u8> = vec![0,0,0];
    let mut target_parent: MerkleNode = tree.clone().discover_parent(&tree.clone().root.unwrap(), &target).unwrap();
    while &target != &root{
        let target_sibling: (Option<MerkleNode>, u8) = tree.clone().discover_sibling(&target_parent, &target).unwrap();
        let target_sibling_node = target_sibling.0.unwrap();
        let target_sibling_lr: u8 = target_sibling.1;
        path.push((target_sibling_node.clone().data, target_sibling_lr));
        target = target_parent.clone().data;
        if &target != &root{
            target_parent = tree.clone().discover_parent(&tree.clone().root.unwrap(), &target).unwrap();
        };
    }
    path.reverse();
    println!("Path: {:?}", &path);
    let mut current_hash = vec![0,0,0];
    while !path.is_empty(){
        let sibling: (Vec<u8>, u8) = path.pop().unwrap();
        if sibling.1 == 0{
            current_hash = hashLeftRight(current_hash, sibling.0);
        }
        else if sibling.1 == 1{
            current_hash = hashLeftRight(sibling.0, current_hash);
        }
    }
    assert_eq!(&current_hash, &root);
}