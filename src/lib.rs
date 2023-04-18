pub mod algorithm;
pub mod biguint;

use crate::algorithm::{generate_multi_prime_key, EXP};
use crate::biguint::BigUint;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Decryption,
    Encryption,
    NprimesTooSmall,
    TooFewPrimes,
}

pub trait RandPrime {
    /// Generate a random prime number with as many bits as given.
    fn gen_prime(&mut self, bits: usize) -> BigUint;
}

#[derive(Clone, Debug)]
pub struct PublicKey {
    n: BigUint,
    e: BigUint,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct PrivateKey {
    public_key: PublicKey,
    d: BigUint,
    primes: Vec<BigUint>,
}

impl PrivateKey {
    pub fn new<R: RandPrime>(rng: &mut R, bit_size: usize) -> Result<Self> {
        return generate_multi_prime_key(rng, 2, bit_size);
    }
    pub fn from_public_key(
        public_key: &PublicKey,
        d: BigUint,
        primes: Vec<BigUint>,
    ) -> Result<Self> {
        Ok(PrivateKey {
            public_key: public_key.clone(),
            d,
            primes,
        })
    }
    pub fn from_components(
        n: BigUint,
        e: BigUint,
        d: BigUint,
        primes: Vec<BigUint>,
    ) -> Result<Self> {
        Ok(PrivateKey {
            public_key: PublicKey { n, e },
            d,
            primes,
        })
    }
    pub fn decrypt(self, c: &BigUint) -> Result<BigUint> {
        Ok(c.modpow(&self.d, &self.public_key.n))
    }
}

impl From<&PrivateKey> for PublicKey {
    fn from(private_key: &PrivateKey) -> Self {
        private_key.public_key.clone()
    }
}

impl PublicKey {
    pub fn new(p: BigUint, q: BigUint) -> Self {
        return PublicKey {
            n: p.mul(q),
            e: BigUint::from(EXP),
        };
    }
    pub fn encrypt(self, m: &BigUint) -> BigUint {
        m.modpow(&self.e, &self.n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rsa() {
        let public_key = PublicKey::new(BigUint::from(2_u64), BigUint::from(3_u64));
        let private_key = PrivateKey::from_public_key(
            &public_key,
            BigUint::from(2_u64),
            vec![BigUint::from(2_u64), BigUint::from(3_u64)],
        )
            .expect("failed to generate a private key");
        println!("Private Key: {:?}", private_key);
        let public_key = PublicKey::from(&private_key);
        println!("Public Key: {:?}", public_key);
        let m = BigUint::from_bytes_le(&[42]);
        println!("Message: {:?}", m);
        let c = public_key.encrypt(&m);
        println!("Encrypted message: {:?}", c);
        let m_decrypted = private_key.decrypt(&c).expect("failed to decrypt");
        println!("Decrypted message: {:?}", m_decrypted);
        assert_eq!(m, m_decrypted);
    }
}
