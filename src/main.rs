use fhe::bfv::BfvParameters;
use fhe_traits::Deserialize;
use getrandom::register_custom_getrandom;

register_custom_getrandom!(guest::my_custom_getrandom);
pub fn main() {
    let degree: usize = 1024;
    let plaintext_modulus: u64 = 65537;
    let moduli: Vec<u64> = vec![1152921504606584833];

    println!("Generating parameters for BFV scheme with degree = {}, plaintext_modulus = {}, and moduli = {:?}", degree, plaintext_modulus, moduli);

    let (prove_params, verify_params) = guest::build_generate_params();

    println!("Proving and verifying parameters...");

    let (output, proof) = prove_params(degree, plaintext_modulus, moduli);
    let is_valid = verify_params(proof);

    let params = BfvParameters::try_deserialize(&output).unwrap();

    println!("output: {:?}", params);
    println!("valid: {}", is_valid);
}
