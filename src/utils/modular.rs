use std::cmp::PartialEq;
use std::ops::{Add, Mul};

#[derive(Debug)]
pub struct ModU32 {
    val: u32,
    remainder: u32,
}

impl PartialEq<u32> for ModU32 {
    fn eq(&self, other: &u32) -> bool {
        self.val == other % self.remainder
    }
}

impl Add<u32> for ModU32 {
    type Output = ModU32;

    fn add(self, other: u32) -> ModU32 {
        ModU32 {
            val: (self.val + other) % self.remainder,
            remainder: self.remainder,
        }
    }
}

impl Add<ModU32> for ModU32 {
    type Output = ModU32;

    fn add(self, other: ModU32) -> ModU32 {
        ModU32 {
            val: (self.val + other.val) % self.remainder,
            remainder: self.remainder,
        }
    }
}

impl Mul<u32> for ModU32 {
    type Output = ModU32;

    fn mul(self, other: u32) -> ModU32 {
        let ret: u64 = (self.val * other) as u64 % self.remainder as u64;
        ModU32 {
            val: ret as u32,
            remainder: self.remainder,
        }
    }
}

impl Mul<ModU32> for ModU32 {
    type Output = ModU32;

    fn mul(self, other: ModU32) -> ModU32 {
        let ret: u64 = (self.val * other.val) as u64 % self.remainder as u64;
        ModU32 {
            val: ret as u32,
            remainder: self.remainder,
        }
    }
}

impl ModU32 {
    pub fn pow(&self, n: usize) -> ModU32 {
        let mut val = 1;
        for _ in 0..n {
            val = val * self.val % self.remainder;
        }
        ModU32 {
            val: val,
            remainder: self.remainder,
        }
    }
    pub fn from(val: u32, remainder: u32) -> ModU32 {
        ModU32 {
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
        let hundred_14 = ModU32::from(114, 107);
        assert_eq!(hundred_14, 7u32)
    }
    #[test]
    fn test_add() {
        let hundred = ModU32::from(100, 107);
        assert_eq!(hundred + 10, 3)
    }
    #[test]
    fn test_mul() {
        let hundred = ModU32::from(100, 107);
        assert_eq!(hundred + 10, 3)
    }
    #[test]
    fn test_pow() {
        let hundred = ModU32::from(2, 1_000_000_007);
        assert_eq!(hundred.pow(30), 73741817)
    }
}
