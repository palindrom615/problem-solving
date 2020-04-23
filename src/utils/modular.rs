use std::cmp::PartialEq;
use std::ops::{Add, Mul, Sub};

#[derive(Debug)]
pub struct Mod_u32 {
    val: u32,
    remainder: u32,
}

impl PartialEq<u32> for Mod_u32 {
    fn eq(&self, other: &u32) -> bool {
        self.val == other % self.remainder
    }
}

impl Add<u32> for Mod_u32 {
    type Output = Mod_u32;

    fn add(self, other: u32) -> Mod_u32 {
        Mod_u32 {
            val: (self.val + other) % self.remainder,
            remainder: self.remainder,
        }
    }
}

impl Add<Mod_u32> for Mod_u32 {
    type Output = Mod_u32;

    fn add(self, other: Mod_u32) -> Mod_u32 {
        Mod_u32 {
            val: (self.val + other.val) % self.remainder,
            remainder: self.remainder,
        }
    }
}

impl Mul<u32> for Mod_u32 {
    type Output = Mod_u32;

    fn mul(self, other: u32) -> Mod_u32 {
        let ret: u64 = (self.val * other) as u64 % self.remainder as u64;
        Mod_u32 {
            val: ret as u32,
            remainder: self.remainder,
        }
    }
}

impl Mul<Mod_u32> for Mod_u32 {
    type Output = Mod_u32;

    fn mul(self, other: Mod_u32) -> Mod_u32 {
        let ret: u64 = (self.val * other.val) as u64 % self.remainder as u64;
        Mod_u32 {
            val: ret as u32,
            remainder: self.remainder,
        }
    }
}

impl Mod_u32 {
    fn pow(&self, n: usize) -> Mod_u32 {
        let mut val = 1;
        for _ in 0..n {
            val = val * self.val % self.remainder;
        }
        Mod_u32 {
            val: val,
            remainder: self.remainder,
        }
    }
    fn from(val: u32, remainder: u32) -> Mod_u32 {
        Mod_u32 {
            val: val % remainder,
            remainder: remainder,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_eq() {
        let hundred_14 = Mod_u32::from(114, 107);
        assert_eq!(hundred_14, 7u32)
    }
    #[test]
    fn test_add() {
        let hundred = Mod_u32::from(100, 107);
        assert_eq!(hundred + 10, 3)
    }
    #[test]
    fn test_mul() {
        let hundred = Mod_u32::from(100, 107);
        assert_eq!(hundred + 10, 3)
    }
    #[test]
    fn test_pow() {
        let hundred = Mod_u32::from(2, 1_000_000_007);
        assert_eq!(hundred.pow(30), 73741817)
    }
}
