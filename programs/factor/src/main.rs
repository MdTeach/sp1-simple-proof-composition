//! A simple program to be proven inside the zkVM.
#![no_main]
sp1_zkvm::entrypoint!(main);

pub fn main() {
    let (a1,a2) = sp1_zkvm::io::read::<(u32,u32)>();
    let prod = a1*a2;
    sp1_zkvm::io::commit(&prod);
}
