#![no_std]
#![no_main]

use nexus_rt::{print, Write};

use nexus_project::rsa::{BigUint, RsaKey};

use rand_core::SeedableRng;

// TODO: Change to ChaCha20Rng for more rounds
use rand_chacha::ChaCha8Rng;

#[nexus_rt::main]
fn main() {
    // TODO: Get entropy for seed from private input tape,
    const ENTROPY_SIZE: usize = 32; // 256 bits = 32 bytes
    let entropy = [0u8; ENTROPY_SIZE];
    let mut rng = ChaCha8Rng::from_seed(entropy);

    let rsa_key = RsaKey::new(&mut rng);
    print!("Generated RSA Key\n");

    // TODO: Add encode() function to encode strings
    let message = BigUint::from_u32(42);
    let encrypted = rsa_key.encrypt(message);
    let decrypted = rsa_key.decrypt(encrypted);
    // assert_eq!(message, decrypted);

    print!("{} -> {} -> {}\n", message, encrypted, decrypted);
}
