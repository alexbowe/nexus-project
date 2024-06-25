#![no_std]
#![no_main]

extern crate alloc;

use nexus_rt::write_log;

use alloc::string::ToString;

fn mod_pow(base: u64, exponent: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base % modulus;
    let mut exp = exponent;
    
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        base = (base * base) % modulus;
        exp /= 2;
    }
    
    result
}

fn rsa_encrypt(message: u64, public_key: (u64, u64)) -> u64 {
    let (e, n) = public_key;
    mod_pow(message, e, n)
}

fn rsa_decrypt(ciphertext: u64, private_key: (u64, u64)) -> u64 {
    let (d, n) = private_key;
    mod_pow(ciphertext, d, n)
}

#[nexus_rt::main]
fn main() {
    write_log("Starting...\n");

    let public_key = (65537, 3233);  // (e, n)
    let private_key = (2753, 3233);  // (d, n)
    let message = 42;
    let encrypted = rsa_encrypt(message, public_key);
    let decrypted = rsa_decrypt(encrypted, private_key);

    write_log(encrypted.to_string().as_str());
    write_log(decrypted.to_string().as_str());
    
    write_log("Done\n");
}
