use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TwoCC(pub u16);

impl TwoCC {
    pub const fn new(a: char, b: char) -> Self {
        Self(((a as u16) << 8) | (b as u16))
    }
}

impl From<&TwoCC> for u16 {
    fn from(id: &TwoCC) -> Self {
        id.0
    }
}

impl From<&TwoCC> for (char, char) {
    fn from(id: &TwoCC) -> Self {
        ((id.0 >> 8) as u8 as char, (id.0 & 255) as u8 as char)
    }
}

impl From<&TwoCC> for String {
    fn from(id: &TwoCC) -> Self {
        let (a, b): (char, char) = id.into();
        let mut s = String::with_capacity(2);
        s.push(a);
        s.push(b);

        s
    }
}

impl From<u16> for TwoCC {
    fn from(id: u16) -> Self {
        Self(id)
    }
}

impl From<(char, char)> for TwoCC {
    fn from((a, b): (char, char)) -> Self {
        Self::new(a, b)
    }
}

impl TryFrom<&str> for TwoCC {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut chars = value.chars();
        if let Some(a) = chars.next() {
            if let Some(b) = chars.next() {
                if chars.next().is_none() {
                    return Ok(Self::new(a, b));
                }
            }
        }
        Err(Box::from("not a 2-char string"))
    }
}

impl Display for TwoCC {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&String::from(self))
    }
}

impl Debug for TwoCC {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", String::from(self))
    }
}

#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FourCC(pub u32);

impl FourCC {
    pub const fn new(a: char, b: char, c: char, d: char) -> Self {
        Self(((a as u32) << 24) | ((b as u32) << 16) | ((c as u32) << 8) | (d as u32))
    }
}

impl From<&FourCC> for u32 {
    fn from(id: &FourCC) -> Self {
        id.0
    }
}

impl From<&FourCC> for (char, char, char, char) {
    fn from(id: &FourCC) -> Self {
        (
            (id.0 >> 24) as u8 as char,
            ((id.0 >> 16) & 255) as u8 as char,
            ((id.0 >> 8) & 255) as u8 as char,
            (id.0 & 255) as u8 as char,
        )
    }
}

impl From<&FourCC> for String {
    fn from(id: &FourCC) -> Self {
        let (a, b, c, d): (char, char, char, char) = id.into();
        let mut s = String::with_capacity(4);
        s.push(a);
        s.push(b);
        s.push(c);
        s.push(d);

        s
    }
}

impl From<u32> for FourCC {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

impl From<(char, char, char, char)> for FourCC {
    fn from((a, b, c, d): (char, char, char, char)) -> Self {
        Self::new(a, b, c, d)
    }
}

impl TryFrom<&str> for FourCC {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut chars = value.chars();
        if let Some(a) = chars.next() {
            if let Some(b) = chars.next() {
                if let Some(c) = chars.next() {
                    if let Some(d) = chars.next() {
                        if chars.next().is_none() {
                            return Ok(Self::new(a, b, c, d));
                        }
                    }
                }
            }
        }
        Err(Box::from("not a 4-char string"))
    }
}

impl Display for FourCC {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&String::from(self))
    }
}

impl Debug for FourCC {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", String::from(self))
    }
}
