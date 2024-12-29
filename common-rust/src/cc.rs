use std::error::Error;
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct TwoCC(pub u16);

impl TwoCC {
    #[inline(always)]
    pub const fn new(first: char, second: char) -> Self {
        Self(((first as u16) << 8) | (second as u16))
    }
    #[inline(always)]
    pub const fn first(self) -> char {
        (self.0 >> 8) as u8 as char
    }
    #[inline(always)]
    pub const fn second(self) -> char {
        (self.0 & 255) as u8 as char
    }
    #[inline(always)]
    pub const fn contains(self, ch: char) -> bool {
        self.first() == ch || self.second() == ch
    }
    pub fn from_lax(value: &str) -> Self {
        let mut chars = value.chars();
        let first = chars.next().unwrap_or('\0');
        let second = chars.next().unwrap_or('\0');
        Self::new(first, second)
    }
    pub fn to_string_lax(self) -> String {
        let mut s = String::with_capacity(4);
        let first = self.first();
        let second = self.second();
        if first != '\0' || second != '\0' {
            s.push(first);
            if second != '\0' {
                s.push(second);
            }
        }

        s
    }
}

impl From<&TwoCC> for u16 {
    fn from(id: &TwoCC) -> Self {
        id.0
    }
}

impl From<&TwoCC> for [u8; 2] {
    fn from(id: &TwoCC) -> Self {
        [(id.0 >> 8) as u8, (id.0 & 255) as u8]
    }
}

impl From<&TwoCC> for (char, char) {
    fn from(id: &TwoCC) -> Self {
        (id.first(), id.second())
    }
}

impl From<&TwoCC> for String {
    fn from(id: &TwoCC) -> Self {
        let mut s = String::with_capacity(2);
        s.push(id.first());
        s.push(id.second());

        s
    }
}

impl From<u16> for TwoCC {
    fn from(id: u16) -> Self {
        Self(id)
    }
}

impl From<[u8; 2]> for TwoCC {
    fn from(id: [u8; 2]) -> Self {
        Self(((id[0] as u16) << 8) | (id[1] as u16))
    }
}

impl From<(char, char)> for TwoCC {
    fn from((first, second): (char, char)) -> Self {
        Self::new(first, second)
    }
}

impl TryFrom<&str> for TwoCC {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut chars = value.chars();
        if let Some(first) = chars.next() {
            if let Some(second) = chars.next() {
                if chars.next().is_none() {
                    return Ok(Self::new(first, second));
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
pub struct ThreeCC(pub u8, pub u8, pub u8);

impl ThreeCC {
    pub const fn new(first: char, second: char, third: char) -> Self {
        Self(first as u8, second as u8, third as u8)
    }
    #[inline(always)]
    pub const fn first(self) -> char {
        self.0 as char
    }
    #[inline(always)]
    pub const fn second(self) -> char {
        self.1 as char
    }
    #[inline(always)]
    pub const fn third(self) -> char {
        self.2 as char
    }
    #[inline(always)]
    pub const fn contains(self, ch: char) -> bool {
        self.first() == ch || self.second() == ch || self.third() == ch
    }
    pub fn from_lax(value: &str) -> Self {
        let mut chars = value.chars();
        let first = chars.next().unwrap_or('\0');
        let second = chars.next().unwrap_or('\0');
        let third = chars.next().unwrap_or('\0');
        Self::new(first, second, third)
    }
    pub fn to_string_lax(self) -> String {
        let mut s = String::with_capacity(4);
        let first = self.first();
        let second = self.second();
        let third = self.third();
        if first != '\0' || second != '\0' || third != '\0' {
            s.push(first);
            if second != '\0' || third != '\0' {
                s.push(second);
                if third != '\0' {
                    s.push(third);
                }
            }
        }

        s
    }
}

impl From<&ThreeCC> for u32 {
    fn from(id: &ThreeCC) -> Self {
        ((id.0 as u32) << 16) | ((id.1 as u32) << 8) | (id.2 as u32)
    }
}

impl From<&ThreeCC> for [u8; 3] {
    fn from(id: &ThreeCC) -> Self {
        [id.0, id.1, id.2]
    }
}

impl From<&ThreeCC> for (char, char, char) {
    fn from(id: &ThreeCC) -> Self {
        (id.first(), id.second(), id.third())
    }
}

impl From<&ThreeCC> for String {
    fn from(id: &ThreeCC) -> Self {
        let mut s = String::with_capacity(3);
        s.push(id.first());
        s.push(id.second());
        s.push(id.third());

        s
    }
}

impl From<u32> for ThreeCC {
    fn from(id: u32) -> Self {
        Self(
            ((id >> 16) & 255) as u8,
            ((id >> 8) & 255) as u8,
            (id & 255) as u8,
        )
    }
}

impl From<[u8; 3]> for ThreeCC {
    fn from(id: [u8; 3]) -> Self {
        Self(id[0], id[1], id[2])
    }
}

impl From<(char, char, char)> for ThreeCC {
    fn from((first, second, third): (char, char, char)) -> Self {
        Self::new(first, second, third)
    }
}

impl TryFrom<&str> for ThreeCC {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut chars = value.chars();
        if let Some(first) = chars.next() {
            if let Some(second) = chars.next() {
                if let Some(third) = chars.next() {
                    if chars.next().is_none() {
                        return Ok(Self::new(first, second, third));
                    }
                }
            }
        }
        Err(Box::from("not a 3-char string"))
    }
}

impl Display for ThreeCC {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_str(&String::from(self))
    }
}

impl Debug for ThreeCC {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{:?}", String::from(self))
    }
}

#[derive(Clone, Copy, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct FourCC(pub u32);

impl FourCC {
    pub const fn new(first: char, second: char, third: char, fourth: char) -> Self {
        Self(
            ((first as u32) << 24)
                | ((second as u32) << 16)
                | ((third as u32) << 8)
                | (fourth as u32),
        )
    }
    #[inline(always)]
    pub const fn first(self) -> char {
        (self.0 >> 24) as u8 as char
    }
    #[inline(always)]
    pub const fn second(self) -> char {
        ((self.0 >> 16) & 255) as u8 as char
    }
    #[inline(always)]
    pub const fn third(self) -> char {
        ((self.0 >> 8) & 255) as u8 as char
    }
    #[inline(always)]
    pub const fn fourth(self) -> char {
        (self.0 & 255) as u8 as char
    }
    #[inline(always)]
    pub const fn contains(self, ch: char) -> bool {
        self.first() == ch || self.second() == ch || self.third() == ch || self.fourth() == ch
    }
    pub fn from_lax(value: &str) -> Self {
        let mut chars = value.chars();
        let first = chars.next().unwrap_or('\0');
        let second = chars.next().unwrap_or('\0');
        let third = chars.next().unwrap_or('\0');
        let fourth = chars.next().unwrap_or('\0');
        Self::new(first, second, third, fourth)
    }
    pub fn to_string_lax(self) -> String {
        let mut s = String::with_capacity(4);
        let first = self.first();
        let second = self.second();
        let third = self.third();
        let fourth = self.fourth();
        if first != '\0' || second != '\0' || third != '\0' || fourth != '\0' {
            s.push(first);
            if second != '\0' || third != '\0' || fourth != '\0' {
                s.push(second);
                if third != '\0' || fourth != '\0' {
                    s.push(third);
                    if fourth != '\0' {
                        s.push(fourth);
                    }
                }
            }
        }

        s
    }
}

impl From<&FourCC> for u32 {
    fn from(id: &FourCC) -> Self {
        id.0
    }
}

impl From<&FourCC> for [u8; 4] {
    fn from(id: &FourCC) -> Self {
        [
            (id.0 >> 24) as u8,
            ((id.0 >> 16) & 255) as u8,
            ((id.0 >> 8) & 255) as u8,
            (id.0 & 255) as u8,
        ]
    }
}

impl From<&FourCC> for (char, char, char, char) {
    fn from(id: &FourCC) -> Self {
        (id.first(), id.second(), id.third(), id.fourth())
    }
}

impl From<&FourCC> for String {
    fn from(id: &FourCC) -> Self {
        let mut s = String::with_capacity(4);
        s.push(id.first());
        s.push(id.second());
        s.push(id.third());
        s.push(id.fourth());

        s
    }
}

impl From<u32> for FourCC {
    fn from(id: u32) -> Self {
        Self(id)
    }
}

impl From<[u8; 4]> for FourCC {
    fn from(id: [u8; 4]) -> Self {
        Self(
            ((id[0] as u32) << 24)
                | ((id[1] as u32) << 16)
                | ((id[2] as u32) << 8)
                | (id[3] as u32),
        )
    }
}

impl From<(char, char, char, char)> for FourCC {
    fn from((first, second, third, fourth): (char, char, char, char)) -> Self {
        Self::new(first, second, third, fourth)
    }
}

impl TryFrom<&str> for FourCC {
    type Error = Box<dyn Error>;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut chars = value.chars();
        if let Some(first) = chars.next() {
            if let Some(second) = chars.next() {
                if let Some(third) = chars.next() {
                    if let Some(fourth) = chars.next() {
                        if chars.next().is_none() {
                            return Ok(Self::new(first, second, third, fourth));
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
