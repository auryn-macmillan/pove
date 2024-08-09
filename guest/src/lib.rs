#![no_main]

use fhe::bfv;
use fhe_traits::Serialize;
use getrandom::register_custom_getrandom;
use std::sync::Arc;

#[jolt::provable]
fn generate_params(degree: usize, plaintext_modulus: u64, moduli: Vec<u64>) -> Vec<u8> {
    register_custom_getrandom!(my_custom_getrandom);
    let params: Arc<bfv::BfvParameters> = bfv::BfvParametersBuilder::new()
        .set_degree(degree)
        .set_plaintext_modulus(plaintext_modulus)
        .set_moduli(&moduli)
        .build_arc()
        .unwrap();

    params.to_bytes()
}

fn my_custom_getrandom(dest: &mut [u8]) -> Result<(), getrandom::Error> {
    for byte in dest {
        *byte = 42; // Replace 42 with your static number
    }
    Ok(())
}
