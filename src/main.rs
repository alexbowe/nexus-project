#![no_std]
#![no_main]

use nexus_rt::{print, Write};

use nexus_project::rsa::generate_rsa_key;

use crypto_bigint::{
    modular::runtime_mod::{DynResidue, DynResidueParams},
    Encoding, NonZero, Uint, U1024, U2048, U256,
};

use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaChaRng,
};

// use core::arch::asm;
// fn read_seed_register() -> u32 {
//     let mut seed: u32;
//     unsafe {
//         //asm!("csrrw {}, seed, x0", out(reg) seed);
//         asm!("csrr {}, mentropy", out(reg) seed);
//     }
//     seed
// }

#[nexus_rt::main]
fn main() {
    let rsa_key = generate_rsa_key();

    let message = 42;
    let encrypted = rsa_key.encrypt(message);
    let decrypted = rsa_key.decrypt(encrypted);
    assert_eq!(message, decrypted);

    print!("{} -> {} -> {}\n", message, encrypted, decrypted);

    // TODO: Insecure. Need good entropy source (riscv asm?)
    // Use seed register to get 16 bits of entropy at a time,
    // combining them using sha to get 256 bits of entropy.
    let entropy = [0u8; 32];
    let mut rng: ChaChaRng = ChaChaRng::from_seed(entropy);
    let mut buf = [0u8; 128];

    rng.fill_bytes(&mut buf[..]);
    let a = U1024::from_be_bytes(buf);
    print!("a: {}\n", a);

    rng.fill_bytes(&mut buf[..]);
    let b = U1024::from_be_bytes(buf);
    print!("b: {}\n", b);

    let product: U2048 = a * b;
    print!("product: {}\n", product);

    // Define the base, exponent, and modulus
    let base = U256::from_u64(4);
    let exponent = U256::from_u64(13);
    let modulus = U256::from_u64(497);

    let non_zero_modulus = NonZero::new(modulus).unwrap();
    let params = DynResidueParams::new(&non_zero_modulus);
    let residue_base = DynResidue::new(&base, params);
    let result = residue_base.pow(&exponent);
    let result_u256 = result.retrieve();

    print!("Result: {}\n", result_u256);
}
