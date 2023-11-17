mod hash_prover;
mod merkle_prover;
mod sig_prover;

pub fn generate_circuit_inputs(){
    /* Inputs
        leaf: [u8;32],
        path: [[u8;32];4],
        position: [u8;4],
        root: [u8;32],

        pub_x: [u8;32],
        pub_y: [u8;32],
        signature: [u8;64],



        where for signature message == leaf
    */
    
}

