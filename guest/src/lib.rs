#![no_main]

use fhe::bfv;
use fhe_traits::Serialize;
use std::{sync::Arc, usize};

#[jolt::provable]
fn generate_params(degree: usize, plaintext_modulus: u64, moduli: Vec<u64>) -> Vec<u8> {
    let params: Arc<bfv::BfvParameters> = bfv::BfvParametersBuilder::new()
        .set_degree(degree)
        .set_plaintext_modulus(plaintext_modulus)
        .set_moduli(&moduli)
        .build_arc()
        .unwrap();

    params.to_bytes()
}
