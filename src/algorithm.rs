use crate::RandPrime;
use crate::PrivateKey;
use crate::{Error, Result};
use crate::biguint::BigUint;

pub const EXP: u64 = 65537;

pub fn generate_multi_prime_key<R: RandPrime + ?Sized>(
    rng: &mut R,
    nprimes: usize,
    bit_size: usize,
) -> Result<PrivateKey> {
    let exp = BigUint::new(EXP);
    generate_multi_prime_key_with_exp(rng, nprimes, bit_size, &exp)
}

pub fn generate_multi_prime_key_with_exp<R: RandPrime + ?Sized>(
    rng: &mut R,
    nprimes: usize,
    bit_size: usize,
    exp: &BigUint,
) -> Result<PrivateKey> {
    if nprimes < 2 {
        return Err(Error::NprimesTooSmall);
    }

    if bit_size < 64 {
        let prime_limit = (1u64 << (bit_size / nprimes) as u64) as f64;
        let mut pi = prime_limit / (prime_limit.ln() - 1f64);
        pi /= 4f64;
        pi /= 2f64;

        if pi < nprimes as f64 {
            return Err(Error::TooFewPrimes);
        }
    }

    let mut primes = vec![BigUint::zero(); nprimes];
    let n_final: BigUint;
    let d_final: BigUint;

    'next: loop {
        let mut todo = bit_size;
        if nprimes >= 7 {
            todo += (nprimes - 2) / 5;
        }

        for (i, prime) in primes.iter_mut().enumerate() {
            *prime = rng.gen_prime(todo / (nprimes - i));
            todo -= prime.bits();
        }

        // Makes sure that primes is pairwise unequal.
        for (i, prime1) in primes.iter().enumerate() {
            for prime2 in primes.iter().take(i) {
                if prime1 == prime2 {
                    continue 'next;
                }
            }
        }

        let mut n = BigUint::one();
        let mut totient = BigUint::one();

        for prime in &primes {
            n *= prime;
            totient *= prime - BigUint::one();
        }

        if n.bits() != bit_size {
            continue 'next;
        }

        if let Some(d) = exp.mod_inverse(totient) {
            n_final = n;
            d_final = d;
            break;
        }
    }

    PrivateKey::from_components(n_final, exp.clone(), d_final, primes)
}
