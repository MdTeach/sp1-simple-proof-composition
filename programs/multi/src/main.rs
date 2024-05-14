//! A simple program to be proven inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);
use sha2::Digest;
use sha2::Sha256;


pub fn main() {
    let vkey = sp1_zkvm::io::read::<[u32;8]>();
    let public_values = sp1_zkvm::io::read::<Vec<u8>>();
    let public_values_digest = Sha256::digest(public_values.clone());
    
    let input_num = u32::from_str_radix(&hex::encode(&public_values),16).expect("Error reading input");
    let prod = input_num*2;

    sp1_zkvm::precompiles::verify::verify_sp1_proof(&vkey, &public_values_digest.into());
    sp1_zkvm::io::commit(&prod);
}
