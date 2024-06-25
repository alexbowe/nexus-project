#![no_std]
#![no_main]

use nexus_rt::{print, Write};

use nexus_project::rsa::generate_rsa_key;

use crypto_bigint::{U2048};

#[nexus_rt::main]
fn main() {
    let rsa_key = generate_rsa_key();

    let message = 42;
    let encrypted = rsa_key.encrypt(message);
    let decrypted = rsa_key.decrypt(encrypted);
    assert_eq!(message, decrypted);

    print!("{} -> {} -> {}\n", message, encrypted, decrypted);

    let random_number = U2048::from_u32(5);
    print!("Random number: {}\n", random_number);
}
