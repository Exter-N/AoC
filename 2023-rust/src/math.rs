use std::ops::{Div, Mul, Rem};

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
