use std::error::Error;
use std::ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign};

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
#[repr(transparent)]
pub struct ItemSet(u64);

impl ItemSet {
    pub fn empty() -> Self {
        Self(0u64)
    }
    pub fn singleton_priority(self) -> Result<u32, Box<dyn Error>> {
        if 1 != self.0.count_ones() {
            Err(Box::from("not a singleton"))
        } else {
            Ok(self.0.trailing_zeros() + 1)
        }
    }
}

impl BitAnd for ItemSet {
    type Output = Self;

    fn bitand(self, rhs: Self) -> Self::Output {
        Self(self.0 & rhs.0)
    }
}

impl BitAndAssign for ItemSet {
    fn bitand_assign(&mut self, rhs: Self) {
        self.0 &= rhs.0
    }
}

impl BitOr for ItemSet {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        Self(self.0 | rhs.0)
    }
}

impl BitOrAssign for ItemSet {
    fn bitor_assign(&mut self, rhs: Self) {
        self.0 |= rhs.0
    }
}

impl TryFrom<char> for ItemSet {
    type Error = Box<dyn Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if value >= 'a' && value <= 'z' {
            Ok(Self(1u64 << (value as u32 - 'a' as u32)))
        } else if value >= 'A' && value <= 'Z' {
            Ok(Self(1u64 << (value as u32 - 'A' as u32 + 26)))
        } else {
            Err(Box::from("invalid item character"))
        }
    }
}

impl TryFrom<&str> for ItemSet {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut set = Self::empty();
        for ch in value.chars() {
            set |= ItemSet::try_from(ch)?;
        }

        Ok(set)
    }
}

impl From<u64> for ItemSet {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<ItemSet> for u64 {
    fn from(value: ItemSet) -> Self {
        value.0
    }
}
