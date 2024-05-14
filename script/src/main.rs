//! A simple script to generate and verify the proof of a given program.

use sp1_sdk::{HashableKey, ProverClient, SP1Stdin};

const FACTOR_ELF: &[u8] = include_bytes!("../../programs/factor/elf/riscv32im-succinct-zkvm-elf");
const MULTI_ELF: &[u8] = include_bytes!("../../programs/multi/elf/riscv32im-succinct-zkvm-elf");

fn main() {
    // Generate proof.
    let n = (3u32, 5u32);
    let client = ProverClient::new();
    let (factor_pk, factor_vk) = client.setup(FACTOR_ELF);
    let (aggregation_pk, aggregation_vk) =client.setup(MULTI_ELF);

    let factor_proof = tracing::info_span!("generate factor proof").in_scope(|| {
        let mut stdin = SP1Stdin::new();
        stdin.write(&n);

        client
            .prove_compressed(&factor_pk, stdin)
            .expect("proving failed")
    });


    // Proof Composition
    let mul_proof = tracing::info_span!("generate tracing proof").in_scope(||{
        let mut stdin = SP1Stdin::new();
        stdin.write::<[u32; 8]>(&factor_vk.hash_u32());
        stdin.write::<Vec<u8>>(&factor_proof.public_values.to_vec());
        stdin.write_proof(factor_proof.proof, factor_vk.vk);
        client.prove(&aggregation_pk, stdin).expect("aggeration proving failed")
    });

    client.verify(&mul_proof, &aggregation_vk).expect("proof verification failed")
}