use fhe::bfv::{BfvParameters, Ciphertext, Encoding, Plaintext, PublicKey};
use fhe_traits::{Deserialize, DeserializeParametrized, FheEncoder, FheEncrypter, Serialize};
use rand::thread_rng;
use risc0_zkvm::guest::env;
use std::sync::Arc;

fn main() {
    // read the public input from the journal
    let params_bytes: Vec<u8> = env::read();
    let pk_bytes: Vec<u8> = env::read();
    let input: Vec<u64> = env::read();

    let params: Arc<BfvParameters> =
        Arc::new(BfvParameters::try_deserialize(&params_bytes).unwrap());
    println!(
        "Degree: {}, Plaintext: {}, Moduli: {:?}",
        params.degree(),
        params.plaintext(),
        params.moduli()
    );

    let pk: PublicKey = PublicKey::from_bytes(&pk_bytes, &params).unwrap();

    // checksum
    let sum: u64 = input.iter().sum();
    if sum > 1 {
        panic!("Too many votes cast");
    }

    println!("Public key: {:?}", pk);
    println!("Inputs: {:?}", input);

    let mut rng = thread_rng();

    let pt = Plaintext::try_encode(&input, Encoding::poly(), &params).unwrap();
    let ct: Ciphertext = pk.try_encrypt(&pt, &mut rng).unwrap();

    // write public output to the journal
    env::commit(&ct.to_bytes());
}
