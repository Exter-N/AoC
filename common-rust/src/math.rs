use std::ops::{Div, Mul, Rem, Sub};

use num_traits::{one, zero, One, Zero};

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
