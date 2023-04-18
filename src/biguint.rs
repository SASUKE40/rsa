use core::cmp::Ordering::{self, Equal, Greater, Less};
use std::fmt::{Display, Formatter};
use std::ops::{Add, Mul, MulAssign, Sub};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct BigUint {
    pub(crate) data: Vec<u8>,
}

impl BigUint {
    pub fn new(num: u64) -> Self {
        let data = num
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .rev()
            .collect();
        BigUint { data }
    }
    pub fn zero() -> Self {
        BigUint { data: vec![0_u8] }
    }
    pub fn one() -> Self {
        BigUint { data: vec![1_u8] }
    }
    pub fn bits(&self) -> usize {
        self.data.len()
    }
    pub fn mod_inverse(&self, m: BigUint) -> Option<BigUint> {
        todo!()
    }
}

impl From<u64> for BigUint {
    fn from(value: u64) -> Self {
        BigUint::new(value)
    }
}

impl Display for BigUint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.data.iter().rev().map(|d| d.to_string()).collect()
        )
    }
}

impl Add for BigUint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = BigUint { data: Vec::new() };
        let mut carry = 0_u8;
        for (index, num) in self.data.iter().enumerate() {
            let tmp = num + rhs[index] + carry;
            result.data.push(tmp % 10);
            carry = tmp / 10;
        }
        result
    }
}

impl<'a> Mul<&'a BigUint> for BigUint {
    type Output = BigUint;

    fn mul(self, rhs: &Self) -> Self::Output {
        todo!()
    }
}

impl<'a> MulAssign<&'a BigUint> for BigUint {
    fn mul_assign(&mut self, rhs: &'a Self) {
        self * rhs
    }
}

impl MulAssign for BigUint {
    fn mul_assign(&mut self, rhs: Self) {
        self * rhs
    }
}

impl<'a> Sub<BigUint> for &'a BigUint {
    type Output = BigUint;

    fn sub(self, rhs: BigUint) -> Self::Output {
        todo!()
    }
}

impl PartialOrd for BigUint {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for BigUint {
    fn cmp(&self, rhs: &Self) -> Ordering {
        let (a_len, b_len) = (self.len(), rhs.len());
        if a_len < b_len {
            return Less;
        }
        if a_len > b_len {
            return Greater;
        }
        for (&ai, &bi) in self.iter().rev().zip(rhs.iter().rev()) {
            if ai < bi {
                return Less;
            }
            if ai > bi {
                return Greater;
            }
        }
        Equal
    }
}

#[cfg(test)]
mod tests {
    use crate::biguint::BigUint;

    #[test]
    fn biguint_add() {
        let a = BigUint::from(114514);
        let b = BigUint::from(114514);
        let c = a + b;
        assert_eq!(c, BigUint::from(229028));
    }
}
