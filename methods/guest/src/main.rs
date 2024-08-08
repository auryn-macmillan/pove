use fhe::bfv::{self, BfvParameters, Ciphertext, Encoding, Plaintext, PublicKey, SecretKey};
use fhe_traits::{
    Deserialize, DeserializeParametrized, FheDecoder, FheEncoder, FheEncrypter, Serialize,
};
use risc0_zkvm::guest::env;
use std::sync::Arc;
use rand::thread_rng;

fn main() {
    // read the public input from the journal
    let params_bytes: Vec<u8> = env::read();
    let pk_bytes: Vec<u8> = env::read();
    let input: u64 = env::read();

    let params: Arc<BfvParameters> =
        Arc::new(BfvParameters::try_deserialize(&params_bytes).unwrap());
    println!(
        "Degree: {}, Plaintext: {}, Moduli: {:?}",
        params.degree(),
        params.plaintext(),
        params.moduli()
    );

    let pk: PublicKey = PublicKey::from_bytes(&pk_bytes, &params).unwrap();

    // Check if inputs are between 0 and 10
    if input < 0 || input > 10 {
        panic!("Input must be between 0 and 10");
    }

    println!("Public key: {:?}", pk);
    println!("Inputs: {:?}", input);

    let mut rng = thread_rng();

    let pt = Plaintext::try_encode(&vec![input], Encoding::poly(), &params).unwrap();
    let ct: Ciphertext = pk.try_encrypt(&pt, &mut rng).unwrap();

    // write public output to the journal
    env::commit(&ct.to_bytes());
}
