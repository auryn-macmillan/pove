#![no_main]

use fhe::bfv::BfvParameters;
use fhe_traits::Deserialize;

#[jolt::provable]
fn deserialize_params(param_bytes: Vec<u8>) {
    BfvParameters::try_deserialize(&param_bytes).unwrap();
}
