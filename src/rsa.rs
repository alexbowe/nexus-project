use crate::math::mod_pow;

// TODO: Implement basic BigUint, or try to get crypto-bigint working without std
pub struct RsaKey {
    e: u64,
    d: u64,
    // TODO: support public-only keys using Option<u64>
    n: u64,
}

impl RsaKey {
    pub fn public_key(&self) -> (u64, u64) {
        (self.e, self.n)
    }

    pub fn private_key(&self) -> (u64, u64) {
        (self.d, self.n)
    }

    pub fn encrypt(&self, message: u64) -> u64 {
        let (exponent, modulus) = self.public_key();
        mod_pow(message, exponent, modulus)
    }

    pub fn decrypt(&self, ciphertext: u64) -> u64 {
        let (exponent, modulus) = self.private_key();
        mod_pow(ciphertext, exponent, modulus)
    }
}

pub fn generate_rsa_key() -> RsaKey {
    let p: u64 = 61;
    let q: u64 = 53;
    let n: u64 = p * q;

    // Euler's totient function φ(n)
    let phi: u64 = (p - 1) * (q - 1);

    // Choose e such that 1 < e < φ(n) and gcd(e, φ(n)) = 1
    // 65537 is a common choice, but too large for our small primes
    let e: u64 = 17;

    let mut d: u64 = 2;
    while (d * e) % phi != 1 {
        d += 1;
    }

    RsaKey { e, d, n }
}
