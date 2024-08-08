//! A program to prove the various steps of the BFV encryption scheme.

// These two lines are necessary for the program to properly compile.
//
// Under the hood, the main function is wrapped with some extra code so that it behaves properly
// inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

use alloy_sol_types::SolType;
use bfv_lib::{params, PublicValuesStruct};

pub fn main() {
    // Read an input to the program.
    //
    // Behind the scenes, this compiles down to a custom system call which handles reading inputs
    // from the prover.
    let degree = sp1_zkvm::io::read::<u64>();
    let plaintext_modulus = sp1_zkvm::io::read::<u64>();
    let moduli = sp1_zkvm::io::read::<Vec<u64>>();

    // Compute the BFV paramters with the given inputs.
    let params = params(degree, plaintext_modulus, moduli.clone());

    // Encode the public values of the program.
    let bytes = PublicValuesStruct::abi_encode(&PublicValuesStruct {
        degree,
        plaintext_modulus,
        moduli,
        params,
    });

    // Commit to the public values of the program. The final proof will have a commitment to all the
    // bytes that were committed to.
    sp1_zkvm::io::commit_slice(&bytes);
}
