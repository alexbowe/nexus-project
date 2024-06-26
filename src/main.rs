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
    const ENTROPY_BITS: usize = 256; // ChaCha wants 256 of entropy
    const ENTROPY_BYTES: usize = ENTROPY_BITS / 8;
    let entropy = [0u8; ENTROPY_BYTES];
    let mut rng = ChaCha8Rng::from_seed(entropy);

    let rsa_key = RsaKey::new(&mut rng);

    // TODO: Add encode() function to encode strings
    let message = BigUint::from_u32(42);
    let encrypted = rsa_key.encrypt(message);
    let decrypted = rsa_key.decrypt(encrypted);
    assert_eq!(message, decrypted);

    print!("{} -> {} -> {}\n", message, encrypted, decrypted);
}
