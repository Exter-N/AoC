use std::{
    mem::replace,
    ops::{Add, Div, Mul, Neg, Rem, Sub},
};

use num_traits::{one, zero, Euclid, One, Zero};

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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinearDiophantineSolution<T> {
    pub x: T,
    pub x_step: T,
    pub y: T,
    pub y_step: T,
}

impl<T> LinearDiophantineSolution<T>
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    pub fn step(&self, k: T) -> Self {
        Self {
            x: self.x + (self.x_step * k),
            y: self.y + (self.y_step * k),
            ..*self
        }
    }
}

impl<T> LinearDiophantineSolution<T>
where
    T: Copy + Default + Ord + Neg<Output = T>,
{
    pub fn normalize(&self) -> Self {
        if self.x_step < <T as Default>::default() {
            Self {
                x_step: -self.x_step,
                y_step: -self.y_step,
                ..*self
            }
        } else {
            *self
        }
    }
}

impl<T> LinearDiophantineSolution<T>
where
    T: Copy + Neg<Output = T> + Add<Output = T> + Mul<Output = T> + Euclid,
{
    pub fn canonicalize_x(&self) -> Self {
        self.step(-self.x.div_euclid(&self.x_step))
    }

    pub fn canonicalize_y(&self) -> Self {
        self.step(-self.y.div_euclid(&self.y_step))
    }
}

pub fn solve_linear_diophantine<T>(a: T, b: T, c: T) -> Option<LinearDiophantineSolution<T>>
where
    T: Copy
        + Default
        + PartialEq
        + Ord
        + One
        + Neg<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Euclid,
{
    let egcd = extended_gcd(a, b);
    if c % egcd.gcd != <T as Default>::default() {
        return None;
    }
    let multiplier = c / egcd.gcd;
    Some(
        LinearDiophantineSolution {
            x: multiplier * egcd.x,
            x_step: -b / egcd.gcd,
            y: multiplier * egcd.y,
            y_step: a / egcd.gcd,
        }
        .normalize()
        .canonicalize_x(),
    )
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
