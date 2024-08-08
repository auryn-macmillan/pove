use fhe::bfv::{BfvParameters, BfvParametersBuilder};
use fhe_traits::Serialize;
use risc0_zkvm::guest::env;
use std::sync::Arc;

fn main() {
    let degree: u64 = env::read();
    let plaintext_modulus: u64 = env::read();
    let moduli: Vec<u64> = env::read();

    let params: Arc<BfvParameters> = BfvParametersBuilder::new()
        .set_degree(degree as usize)
        .set_plaintext_modulus(plaintext_modulus)
        .set_moduli(&moduli)
        .build_arc()
        .unwrap();

    let serialized_params: Vec<u8> = params.to_bytes();

    // let vectorized_params: Vec<u8> = serialized_params;

    // write public output to the journal
    env::commit(&serialized_params);
}
