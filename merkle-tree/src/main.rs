mod experiment;
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

#[test]
fn test(){

}