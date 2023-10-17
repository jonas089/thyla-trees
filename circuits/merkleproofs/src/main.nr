use dep::std;

fn hash_sha(message: [u8]) -> [u8; 32]{
    std::hash::sha256(message)
}

// input size depends on tree depth
fn main(x:[str;2]) {

}

#[test]
fn test_main() {
    main(["asdf", "asdf"]);
}


/*

* sort hash path before submission according to L / R position
* simple re-hashing in reverse order
* [0x1, 0x2, ax33, fb44, cd59]
-> c = H(H(H(H(0x1, 0x2), ax33), fb44), cd59)
assert(c == merkle_hash)
! fixed size array

*/