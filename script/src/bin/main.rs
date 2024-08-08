//! An end-to-end example of using the SP1 SDK to generate a proof of a program that can be executed
//! or have a core proof generated.
//!
//! You can run this script using the following command:
//! ```shell
//! RUST_LOG=info cargo run --release -- --execute
//! ```
//! or
//! ```shell
//! RUST_LOG=info cargo run --release -- --prove
//! ```

use alloy_sol_types::SolType;
use bfv_lib::PublicValuesStruct;
use clap::Parser;
use sp1_sdk::{ProverClient, SP1Stdin};

/// The ELF (executable and linkable format) file for the Succinct RISC-V zkVM.
pub const BFV_ELF: &[u8] = include_bytes!("../../../elf/riscv32im-succinct-zkvm-elf");

/// The arguments for the command.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(long)]
    execute: bool,

    #[clap(long)]
    prove: bool,

    #[clap(long, default_value = "1024")]
    degree: u64,

    #[clap(long, default_value = "65537")]
    plaintext_modulus: u64,

    #[clap(long, default_value = "1152921504606584833")]
    moduli: Vec<u64>,
}

fn main() {
    // Setup the logger.
    sp1_sdk::utils::setup_logger();

    // Parse the command line arguments.
    let args = Args::parse();

    if args.execute == args.prove {
        eprintln!("Error: You must specify either --execute or --prove");
        std::process::exit(1);
    }

    // Setup the prover client.
    let client = ProverClient::new();

    // Setup the inputs.
    let mut stdin = SP1Stdin::new();
    stdin.write(&args.degree);
    stdin.write(&args.plaintext_modulus);
    stdin.write(&args.moduli);

    println!("degree: {}", args.degree);
    println!("plaintext_modulus: {}", args.plaintext_modulus);
    println!("moduli: {:?}", args.moduli);

    if args.execute {
        // Execute the program
        let (output, report) = client.execute(BFV_ELF, stdin).run().unwrap();
        println!("Program executed successfully.");

        // Read the output.
        let decoded = PublicValuesStruct::abi_decode(output.as_slice(), true).unwrap();
        let PublicValuesStruct {
            degree,
            plaintext_modulus,
            moduli,
            params,
        } = decoded;
        println!("degree: {}", degree);
        println!("plaintext_modulus: {}", plaintext_modulus);
        println!("moduli: {:?}", moduli);
        println!("params: {:?}", params);

        let expected = bfv_lib::params(degree, plaintext_modulus, moduli);
        assert_eq!(params, expected);
        println!("Values are correct!");

        // Record the number of cycles executed.
        println!("Number of cycles: {}", report.total_instruction_count());
    } else {
        // Setup the program for proving.
        let (pk, vk) = client.setup(BFV_ELF);

        // Generate the proof
        let proof = client
            .prove(&pk, stdin)
            .run()
            .expect("failed to generate proof");

        println!("Successfully generated proof!");

        // Verify the proof.
        client.verify(&proof, &vk).expect("failed to verify proof");
        println!("Successfully verified proof!");
    }
}
