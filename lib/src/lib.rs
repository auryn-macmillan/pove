use alloy_sol_types::sol;
use fhe::bfv::{BfvParameters, BfvParametersBuilder};
use fhe_traits::Serialize;
use std::sync::Arc;

sol! {
    /// The public values encoded as a struct that can be easily deserialized inside Solidity.
    struct PublicValuesStruct {
        uint64 degree;
        uint64 plaintext_modulus;
        uint64[] moduli;
        uint8[] params;
    }
}

/// Compute the BFV parameters with the given inputs.
pub fn params(degree: u64, plaintext_modulus: u64, moduli: Vec<u64>) -> Vec<u8> {
    let params: Arc<BfvParameters> = BfvParametersBuilder::new()
        .set_degree(degree as usize)
        .set_plaintext_modulus(plaintext_modulus)
        .set_moduli(&moduli)
        .build_arc()
        .unwrap();

    params.to_bytes()
}
