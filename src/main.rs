use fhe::bfv::BfvParameters;
use fhe_traits::Deserialize;

pub fn main() {
    let degree: usize = 1024;
    let plaintext_modulus: u64 = 65537;
    let moduli: Vec<u64> = vec![1152921504606584833];

    let (prove_params, verify_params) = guest::build_generate_params();

    let (output, proof) = prove_params(degree, plaintext_modulus, moduli);
    let is_valid = verify_params(proof);

    let params = BfvParameters::try_deserialize(&output).unwrap();

    println!("output: {:?}", params);
    println!("valid: {}", is_valid);
}
