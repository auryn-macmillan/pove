#![no_main]
use fhe::bfv::BfvParametersBuilder;
use fhe_traits::Serialize;


#[jolt::provable]
fn generate_params(degree: usize, plaintext_modulus: u64, moduli: Vec<u64>) -> Vec<u8> {
    let params = BfvParametersBuilder::new()
        .set_degree(degree)
        .set_plaintext_modulus(plaintext_modulus)
        .set_moduli(&moduli)
        .build_arc()
        .unwrap();

    params.to_bytes()
}

pub fn my_custom_getrandom(dest: &mut [u8]) -> Result<(), getrandom::Error> {
    for byte in dest {
        *byte = 42;
    }
    Ok(())
}
