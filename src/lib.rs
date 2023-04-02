use num_bigint::BigUint;
pub mod algorithm;
use crate::algorithm::generate_multi_prime_key;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Decryption,
    Encryption,
    NprimesTooSmall,
    TooFewPrimes,
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
    pub fn new() -> Result<Self> {
        let mut rng = rand::thread_rng();
        let bit_size = 4096;
        return generate_multi_prime_key(&mut rng, 2, bit_size);
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
    pub fn encrypt(self, m: &BigUint) -> BigUint {
        m.modpow(&self.e, &self.n)
    }
}
#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_rsa() {
        let private_key = PrivateKey::new().expect("failed to generate a private key");
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
