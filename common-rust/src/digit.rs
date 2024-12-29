use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{Add, Div, Mul, Sub},
};

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Digit<const RADIX: u8>(u8);

impl<const RADIX: u8> Digit<RADIX> {
    pub const fn new(value: u8) -> Option<Self> {
        if value < RADIX {
            Some(Digit(value))
        } else {
            None
        }
    }

    pub const fn get(self) -> u8 {
        self.0
    }

    pub fn extract_lowest<T>(value: T) -> (T, Self)
    where
        T: Copy + From<u8> + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
        u8: TryFrom<T, Error: Debug>,
    {
        let radix = T::from(RADIX);
        let rest = value / radix;
        (rest, Digit(u8::try_from(value - rest * radix).unwrap()))
    }

    pub fn append_lowest<T>(value: T, digit: Self) -> T
    where
        T: From<u8> + Add<Output = T> + Mul<Output = T>,
    {
        value * T::from(RADIX) + T::from(digit.0)
    }
}

impl<const RADIX: u8> TryFrom<u8> for Digit<RADIX> {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if value < RADIX {
            Ok(Digit(value))
        } else {
            Err(())
        }
    }
}

impl<const RADIX: u8> TryFrom<u16> for Digit<RADIX> {
    type Error = ();

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        if value < RADIX as u16 {
            Ok(Digit(value as u8))
        } else {
            Err(())
        }
    }
}

impl<const RADIX: u8> TryFrom<u32> for Digit<RADIX> {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        if value < RADIX as u32 {
            Ok(Digit(value as u8))
        } else {
            Err(())
        }
    }
}

impl<const RADIX: u8> TryFrom<u64> for Digit<RADIX> {
    type Error = ();

    fn try_from(value: u64) -> Result<Self, Self::Error> {
        if value < RADIX as u64 {
            Ok(Digit(value as u8))
        } else {
            Err(())
        }
    }
}

impl<const RADIX: u8> TryFrom<u128> for Digit<RADIX> {
    type Error = ();

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        if value < RADIX as u128 {
            Ok(Digit(value as u8))
        } else {
            Err(())
        }
    }
}

impl<const RADIX: u8> TryFrom<char> for Digit<RADIX> {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if let Some(value) = value.to_digit(RADIX as u32) {
            Ok(Digit(value as u8))
        } else {
            Err(())
        }
    }
}

impl<const RADIX: u8> From<Digit<RADIX>> for u8 {
    fn from(value: Digit<RADIX>) -> Self {
        value.0
    }
}

impl<const RADIX: u8> From<Digit<RADIX>> for u16 {
    fn from(value: Digit<RADIX>) -> Self {
        value.0 as u16
    }
}

impl<const RADIX: u8> From<Digit<RADIX>> for u32 {
    fn from(value: Digit<RADIX>) -> Self {
        value.0 as u32
    }
}

impl<const RADIX: u8> From<Digit<RADIX>> for u64 {
    fn from(value: Digit<RADIX>) -> Self {
        value.0 as u64
    }
}

impl<const RADIX: u8> From<Digit<RADIX>> for u128 {
    fn from(value: Digit<RADIX>) -> Self {
        value.0 as u128
    }
}

impl<const RADIX: u8> From<Digit<RADIX>> for char {
    fn from(value: Digit<RADIX>) -> Self {
        char::from_digit(value.0 as u32, RADIX as u32).unwrap()
    }
}

impl<const RADIX: u8> Display for Digit<RADIX> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.0, f)
    }
}

impl<const RADIX: u8> Debug for Digit<RADIX> {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Debug::fmt(&self.0, f)
    }
}
