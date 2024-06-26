use crypto_bigint::{
    modular::runtime_mod::{DynResidue, DynResidueParams},
    CheckedSub, NonZero,
};

pub use crypto_bigint::{Random, Uint};
use crypto_primes::generate_prime_with_rng;

use rand_core::CryptoRngCore;

// Should use at least 4096 bits in production.
// TODO: Make RsaKey struct generic to support user specified widths.
const BITWIDTH: usize = 64;
const LIMBS: usize = BITWIDTH / 32;
const HALF_LIMBS: usize = LIMBS / 2;

pub type BigUint = Uint<LIMBS>;
type HalfSizeBigUint = Uint<HALF_LIMBS>;

pub struct RsaKey {
    // TODO: support public-only keys using Option<>
    e: BigUint,
    d: BigUint,
    n: BigUint,
}

impl RsaKey {
    pub fn new(rng: &mut impl CryptoRngCore) -> RsaKey {
        generate_rsa_key(rng)
    }

    pub fn public_key(&self) -> (BigUint, BigUint) {
        (self.e, self.n)
    }

    pub fn private_key(&self) -> (BigUint, BigUint) {
        (self.d, self.n)
    }

    pub fn encrypt(&self, message: BigUint) -> BigUint {
        let (exponent, modulus) = self.public_key();
        mod_pow(message, exponent, modulus)
    }

    pub fn decrypt(&self, ciphertext: BigUint) -> BigUint {
        let (exponent, modulus) = self.private_key();
        mod_pow(ciphertext, exponent, modulus)
    }
}

fn generate_rsa_key<R: CryptoRngCore>(rng: &mut R) -> RsaKey {
    let p = generate_random_prime(rng);
    let q = generate_random_prime(rng);
    let n = p * q;
    let phi = totient(p, q);

    // Choose e such that 1 < e < φ(n) and gcd(e, φ(n)) = 1
    // 65537 is a common choice, but lets check our assumptions
    let e = BigUint::from_u64(65537);
    assert!(BigUint::ONE < e && e < phi);
    assert!(gcd(e, phi) == BigUint::ONE);

    let d = e.inv_mod(&phi).0;

    RsaKey { e, d, n }
}

// Euler's totient function φ(n)
fn totient(p: HalfSizeBigUint, q: HalfSizeBigUint) -> BigUint {
    let p_sub_1 = p.checked_sub(&HalfSizeBigUint::ONE).unwrap();
    let q_sub_1 = q.checked_sub(&HalfSizeBigUint::ONE).unwrap();
    p_sub_1 * q_sub_1
}

fn generate_random_prime<R: CryptoRngCore>(rng: &mut R) -> HalfSizeBigUint {
    generate_prime_with_rng(rng, Some(HalfSizeBigUint::BITS))
}

#[inline]
fn modulo(a: &BigUint, b: &BigUint) -> BigUint {
    a.div_rem(&NonZero::new(*b).unwrap()).1
}

fn gcd(mut a: BigUint, mut b: BigUint) -> BigUint {
    while b != BigUint::ZERO {
        let temp = b;
        b = modulo(&a, &b);
        a = temp;
    }
    a
}

fn mod_pow(base: BigUint, exponent: BigUint, modulus: BigUint) -> BigUint {
    let non_zero_modulus = NonZero::new(modulus).unwrap();
    let params = DynResidueParams::new(&non_zero_modulus);
    let residue_base = DynResidue::new(&base, params);
    let result = residue_base.pow(&exponent).retrieve();
    result
}
