use std::{
    fmt::{Debug, Display, Formatter, Result as FmtResult},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, Sub, SubAssign},
};

use itertools::Either;
use num_traits::{Euclid, One};

use super::{extended_gcd, gcd};

#[doc = "ax + by = c"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinearBivariateDiophantineEquation<T> {
    pub a: T,
    pub b: T,
    pub c: T,
}

impl<T> LinearBivariateDiophantineEquation<T>
where
    T: Copy
        + Default
        + PartialEq
        + PartialOrd
        + One
        + Neg<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Euclid,
{
    pub fn solve(&self) -> Option<LinearBivariateDiophantineSolution<T>> {
        let egcd = extended_gcd(self.a, self.b);
        if egcd.gcd == <T as Default>::default() || self.c % egcd.gcd != <T as Default>::default() {
            return None;
        }
        let multiplier = self.c / egcd.gcd;
        Some(
            LinearBivariateDiophantineSolution {
                x: multiplier * egcd.x,
                x_step: -self.b / egcd.gcd,
                y: multiplier * egcd.y,
                y_step: self.a / egcd.gcd,
            }
            .normalize()
            .canonicalize_x(),
        )
    }
}

impl<T> LinearBivariateDiophantineEquation<T>
where
    T: Copy + Default + PartialEq + Div<Output = T> + Rem<Output = T>,
{
    pub fn simplify(&self) -> Self {
        let gcd = gcd(gcd(self.a, self.b), self.c);
        if gcd == <T as Default>::default() {
            return *self;
        }

        Self {
            a: self.a / gcd,
            b: self.b / gcd,
            c: self.c / gcd,
        }
    }
}

impl<T> LinearBivariateDiophantineEquation<T>
where
    T: Copy + Default + PartialEq + DivAssign + Rem<Output = T>,
{
    pub fn inplace_simplify(&mut self) {
        let gcd = gcd(gcd(self.a, self.b), self.c);
        if gcd == <T as Default>::default() {
            return;
        }

        self.a /= gcd;
        self.b /= gcd;
        self.c /= gcd;
    }
}

impl<T> LinearBivariateDiophantineEquation<T>
where
    T: Copy
        + Default
        + One
        + PartialEq
        + PartialOrd
        + Neg<Output = T>
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Rem<Output = T>
        + Euclid,
{
    pub fn solve_with(
        &self,
        other: &Self,
    ) -> Option<Either<(T, T), LinearBivariateDiophantineSolution<T>>> {
        let mut eq1 = *self;
        let mut eq2 = *other;

        let a_gcd = gcd(eq1.a, eq2.a);
        eq2 = (eq2 * eq1.a / a_gcd - eq1 * eq2.a / a_gcd).simplify();

        if eq2.b == <T as Default>::default() {
            return if eq2.c != <T as Default>::default() {
                None
            } else {
                todo!();
                // eq1.simplify().solve().map(|sol| Either::Right(sol))
            };
        }

        if eq2.b != <T as One>::one() {
            return None;
        }

        eq1 = (eq1 - eq2 * eq1.b).simplify();

        if eq1.a != <T as One>::one() {
            return None;
        }

        Some(Either::Left((eq1.c, eq2.c)))
    }
}

impl<T> Neg for LinearBivariateDiophantineEquation<T>
where
    T: Neg,
{
    type Output = LinearBivariateDiophantineEquation<<T as Neg>::Output>;

    fn neg(self) -> Self::Output {
        Self::Output {
            a: -self.a,
            b: -self.b,
            c: -self.c,
        }
    }
}

impl<T, U> Add<LinearBivariateDiophantineEquation<U>> for LinearBivariateDiophantineEquation<T>
where
    T: Add<U>,
{
    type Output = LinearBivariateDiophantineEquation<<T as Add<U>>::Output>;

    fn add(self, rhs: LinearBivariateDiophantineEquation<U>) -> Self::Output {
        Self::Output {
            a: self.a + rhs.a,
            b: self.b + rhs.b,
            c: self.c + rhs.c,
        }
    }
}

impl<T, U> AddAssign<LinearBivariateDiophantineEquation<U>>
    for LinearBivariateDiophantineEquation<T>
where
    T: AddAssign<U>,
{
    fn add_assign(&mut self, rhs: LinearBivariateDiophantineEquation<U>) {
        self.a += rhs.a;
        self.b += rhs.b;
        self.c += rhs.c;
    }
}

impl<T, U> Sub<LinearBivariateDiophantineEquation<U>> for LinearBivariateDiophantineEquation<T>
where
    T: Sub<U>,
{
    type Output = LinearBivariateDiophantineEquation<<T as Sub<U>>::Output>;

    fn sub(self, rhs: LinearBivariateDiophantineEquation<U>) -> Self::Output {
        Self::Output {
            a: self.a - rhs.a,
            b: self.b - rhs.b,
            c: self.c - rhs.c,
        }
    }
}

impl<T, U> SubAssign<LinearBivariateDiophantineEquation<U>>
    for LinearBivariateDiophantineEquation<T>
where
    T: SubAssign<U>,
{
    fn sub_assign(&mut self, rhs: LinearBivariateDiophantineEquation<U>) {
        self.a -= rhs.a;
        self.b -= rhs.b;
        self.c -= rhs.c;
    }
}

impl<T, U> Mul<U> for LinearBivariateDiophantineEquation<T>
where
    T: Mul<U>,
    U: Copy,
{
    type Output = LinearBivariateDiophantineEquation<<T as Mul<U>>::Output>;

    fn mul(self, rhs: U) -> Self::Output {
        Self::Output {
            a: self.a * rhs,
            b: self.b * rhs,
            c: self.c * rhs,
        }
    }
}

impl<T, U> MulAssign<U> for LinearBivariateDiophantineEquation<T>
where
    T: MulAssign<U>,
    U: Copy,
{
    fn mul_assign(&mut self, rhs: U) {
        self.a *= rhs;
        self.b *= rhs;
        self.c *= rhs;
    }
}

impl<T, U> Div<U> for LinearBivariateDiophantineEquation<T>
where
    T: Div<U>,
    U: Copy,
{
    type Output = LinearBivariateDiophantineEquation<<T as Div<U>>::Output>;

    fn div(self, rhs: U) -> Self::Output {
        Self::Output {
            a: self.a / rhs,
            b: self.b / rhs,
            c: self.c / rhs,
        }
    }
}

impl<T, U> DivAssign<U> for LinearBivariateDiophantineEquation<T>
where
    T: DivAssign<U>,
    U: Copy,
{
    fn div_assign(&mut self, rhs: U) {
        self.a /= rhs;
        self.b /= rhs;
        self.c /= rhs;
    }
}

impl<T> Display for LinearBivariateDiophantineEquation<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}x + {}y = {}", self.a, self.b, self.c)
    }
}

#[doc = "a*(x + x_step*k) + b(y + y_step*k) = c"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LinearBivariateDiophantineSolution<T> {
    pub x: T,
    pub x_step: T,
    pub y: T,
    pub y_step: T,
}

impl<T> LinearBivariateDiophantineSolution<T>
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

impl<T> LinearBivariateDiophantineSolution<T>
where
    T: Copy + Default + PartialOrd + Neg<Output = T>,
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

impl<T> LinearBivariateDiophantineSolution<T>
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
