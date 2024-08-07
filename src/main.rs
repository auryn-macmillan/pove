use fhe::bfv::{BfvParameters, BfvParametersBuilder};
use fhe_traits::Serialize;
use std::{sync::Arc, usize};

pub fn main() {
    let degree: usize = 1024;
    let plaintext_modulus: u64 = 65537;
    let moduli: Vec<u64> = vec![1152921504606584833];

    let params: Arc<BfvParameters> = BfvParametersBuilder::new()
        .set_degree(degree)
        .set_plaintext_modulus(plaintext_modulus)
        .set_moduli(&moduli)
        .build_arc()
        .unwrap();

    let (prove_params, verify_params) = guest::build_deserialize_params();

    let (_return, proof) = prove_params(params.to_bytes());
    let is_valid = verify_params(proof);

    println!("output:\n{}\n{}\n{:?}", degree, plaintext_modulus, moduli);
    println!("valid: {}", is_valid);
}
