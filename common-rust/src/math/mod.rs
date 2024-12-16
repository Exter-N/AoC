use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    mem::replace,
    ops::{Add, Div, Mul, Rem, Sub},
};

use num_traits::{one, zero, One, Zero};

pub mod diophantine;

pub fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + Default + PartialEq + Rem<Output = T>,
{
    let mut aa = a;
    let mut bb = b;
    while <T as Default>::default() != bb {
        let r = aa % bb;
        aa = bb;
        bb = r;
    }

    aa
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BezoutCoefficients<T> {
    pub gcd: T,
    pub x: T,
    pub y: T,
}

impl<T> Display for BezoutCoefficients<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}a + {}b = {}", self.x, self.y, self.gcd)
    }
}

pub fn extended_gcd<T>(a: T, b: T) -> BezoutCoefficients<T>
where
    T: Copy
        + Default
        + PartialEq
        + One
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>,
{
    let mut aa = a;
    let mut bb = b;
    let mut s = <T as One>::one();
    let mut ss = <T as Default>::default();
    let mut t = <T as Default>::default();
    let mut tt = <T as One>::one();
    while <T as Default>::default() != bb {
        let q = aa / bb;
        let r = aa - bb * q;
        let s_new = s - ss * q;
        let t_new = t - tt * q;
        aa = replace(&mut bb, r);
        s = replace(&mut ss, s_new);
        t = replace(&mut tt, t_new);
    }

    BezoutCoefficients {
        gcd: aa,
        x: s,
        y: t,
    }
}

pub fn lcm<T>(a: T, b: T) -> T
where
    T: Copy + Default + Div<Output = T> + Mul<Output = T> + PartialEq + Rem<Output = T>,
{
    (a / gcd(a, b)) * b
}

pub fn scale_u8(x: u8, y: u8) -> u8 {
    ((x as u16) * (y as u16) / 255u16) as u8
}

pub fn abs_diff<T>(a: T, b: T) -> T
where
    T: Ord + Sub<T, Output = T>,
{
    if a < b {
        b - a
    } else {
        a - b
    }
}

pub fn is_integer<T>() -> bool
where
    T: Div<T, Output = T> + PartialEq + One + Zero,
{
    one::<T>() / (one::<T>() + one::<T>()) == zero::<T>()
}
