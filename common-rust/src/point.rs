use std::cmp::{max, min, Ordering};
use std::error::Error;
use std::hash::Hash;
use std::ops::{Add, AddAssign, Index, IndexMut, Neg, Sub, SubAssign};

use num_traits::identities::{one, zero};
use num_traits::ops::overflowing::{OverflowingAdd, OverflowingSub};
use num_traits::{One, Signed, Zero};

use crate::math::abs_diff;

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Point2<T>(pub T, pub T);

impl<T> Point2<T>
where
    T: Zero,
{
    pub fn zero() -> Self {
        Self(zero(), zero())
    }
}

impl<T> Point2<T>
where
    T: Add<T, Output = T> + Ord + Sub<T, Output = T>,
{
    pub fn manhattan_distance(self, other: Self) -> T {
        abs_diff(self.0, other.0) + abs_diff(self.1, other.1)
    }
}

impl<T> Point2<T>
where
    T: Ord + Sub<T, Output = T>,
{
    pub fn chebyshev_distance(self, other: Self) -> T {
        max(abs_diff(self.0, other.0), abs_diff(self.1, other.1))
    }
}

impl<T> Point2<T>
where
    T: Copy,
{
    pub const fn with_x(self, x: T) -> Self {
        Self(x, self.1)
    }
    pub const fn with_y(self, y: T) -> Self {
        Self(self.0, y)
    }
}

impl<T> Point2<T>
where
    T: Ord,
{
    pub fn direction_towards(&self, towards: &Self) -> Option<Direction2> {
        match self.0.cmp(&towards.0) {
            Ordering::Less => {
                if self.1 == towards.1 {
                    Some(Direction2::Right)
                } else {
                    None
                }
            }
            Ordering::Equal => match self.1.cmp(&towards.1) {
                Ordering::Less => Some(Direction2::Down),
                Ordering::Equal => None,
                Ordering::Greater => Some(Direction2::Up),
            },
            Ordering::Greater => {
                if self.1 == towards.1 {
                    Some(Direction2::Left)
                } else {
                    None
                }
            }
        }
    }
    pub fn componentwise_min(self, other: Self) -> Self {
        Self(min(self.0, other.0), min(self.1, other.1))
    }
    pub fn componentwise_max(self, other: Self) -> Self {
        Self(max(self.0, other.0), max(self.1, other.1))
    }
}

impl<T> Point2<T>
where
    T: Add<T, Output = T> + Copy + Sub<T, Output = T>,
{
    pub fn towards(self, towards: Direction2, distance: T) -> Self {
        match towards {
            Direction2::Right => self.with_x(self.0 + distance),
            Direction2::Down => self.with_y(self.1 + distance),
            Direction2::Left => self.with_x(self.0 - distance),
            Direction2::Up => self.with_y(self.1 - distance),
        }
    }
}

impl<T> Point2<T>
where
    T: OverflowingAdd + Copy + OverflowingSub,
{
    pub fn try_towards(self, towards: Direction2, distance: T) -> Option<Self> {
        match towards {
            Direction2::Right => {
                let (new_x, overflow) = self.0.overflowing_add(&distance);
                if overflow {
                    None
                } else {
                    Some(self.with_x(new_x))
                }
            }
            Direction2::Down => {
                let (new_y, overflow) = self.1.overflowing_add(&distance);
                if overflow {
                    None
                } else {
                    Some(self.with_y(new_y))
                }
            }
            Direction2::Left => {
                let (new_x, overflow) = self.0.overflowing_sub(&distance);
                if overflow {
                    None
                } else {
                    Some(self.with_x(new_x))
                }
            }
            Direction2::Up => {
                let (new_y, overflow) = self.1.overflowing_sub(&distance);
                if overflow {
                    None
                } else {
                    Some(self.with_y(new_y))
                }
            }
        }
    }
}

impl<T> Point2<T>
where
    T: Add<T, Output = T> + Copy + One + Sub<T, Output = T>,
{
    pub fn next_towards(self, towards: Direction2) -> Self {
        self.towards(towards, one())
    }
}

impl<T> Point2<T>
where
    T: OverflowingAdd + Copy + One + OverflowingSub,
{
    pub fn try_next_towards(self, towards: Direction2) -> Option<Self> {
        self.try_towards(towards, one())
    }
}

impl<T> Point2<T>
where
    T: Add<T, Output = T> + Copy + One,
{
    pub fn next_right(self) -> Self {
        self.with_x(self.0 + one())
    }
    pub fn next_down(self) -> Self {
        self.with_y(self.1 + one())
    }
}

impl<T> Point2<T>
where
    T: OverflowingAdd + Copy + One,
{
    pub fn try_next_right(self) -> Option<Self> {
        let (new_x, overflow) = self.0.overflowing_add(&one());
        if overflow {
            None
        } else {
            Some(self.with_x(new_x))
        }
    }
    pub fn try_next_down(self) -> Option<Self> {
        let (new_y, overflow) = self.1.overflowing_add(&one());
        if overflow {
            None
        } else {
            Some(self.with_y(new_y))
        }
    }
}

impl<T> Point2<T>
where
    T: Sub<T, Output = T> + Copy + One,
{
    pub fn next_left(self) -> Self {
        self.with_x(self.0 - one())
    }
    pub fn next_up(self) -> Self {
        self.with_y(self.1 - one())
    }
}

impl<T> Point2<T>
where
    T: OverflowingSub + Copy + One,
{
    pub fn try_next_left(self) -> Option<Self> {
        let (new_x, overflow) = self.0.overflowing_sub(&one());
        if overflow {
            None
        } else {
            Some(self.with_x(new_x))
        }
    }
    pub fn try_next_up(self) -> Option<Self> {
        let (new_y, overflow) = self.1.overflowing_sub(&one());
        if overflow {
            None
        } else {
            Some(self.with_y(new_y))
        }
    }
}

impl<T> PartialOrd for Point2<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Some(cmp_x) = self.0.partial_cmp(&other.0) {
            if let Some(cmp_y) = self.1.partial_cmp(&other.1) {
                if cmp_x == cmp_y {
                    return Some(cmp_x);
                }
            }
        }
        None
    }
}

impl<T, U> Add<Point2<U>> for Point2<T>
where
    T: Add<U>,
{
    type Output = Point2<<T as Add<U>>::Output>;

    fn add(self, rhs: Point2<U>) -> Self::Output {
        Point2(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl<T, U> AddAssign<Point2<U>> for Point2<T>
where
    T: AddAssign<U>,
{
    fn add_assign(&mut self, rhs: Point2<U>) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl<T, U> Sub<Point2<U>> for Point2<T>
where
    T: Sub<U>,
{
    type Output = Point2<<T as Sub<U>>::Output>;

    fn sub(self, rhs: Point2<U>) -> Self::Output {
        Point2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl<T, U> SubAssign<Point2<U>> for Point2<T>
where
    T: SubAssign<U>,
{
    fn sub_assign(&mut self, rhs: Point2<U>) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub struct Point3<T>(pub T, pub T, pub T);

impl<T> Point3<T>
where
    T: Zero,
{
    pub fn zero() -> Self {
        Self(zero(), zero(), zero())
    }
}

impl<T> Point3<T>
where
    T: Add<T, Output = T> + Ord + Sub<T, Output = T>,
{
    pub fn manhattan_distance(self, other: Self) -> T {
        abs_diff(self.0, other.0) + abs_diff(self.1, other.1) + abs_diff(self.2, other.2)
    }
}

impl<T> Point3<T>
where
    T: Ord + Sub<T, Output = T>,
{
    pub fn chebyshev_distance(self, other: Self) -> T {
        max(
            max(abs_diff(self.0, other.0), abs_diff(self.1, other.1)),
            abs_diff(self.2, other.2),
        )
    }
}

impl<T> Point3<T>
where
    T: Copy,
{
    pub const fn with_x(self, x: T) -> Self {
        Self(x, self.1, self.2)
    }
    pub const fn with_y(self, y: T) -> Self {
        Self(self.0, y, self.2)
    }
    pub const fn with_z(self, z: T) -> Self {
        Self(self.0, self.1, z)
    }
}

impl<T> Point3<T>
where
    T: Ord,
{
    pub fn direction_towards(&self, towards: &Self) -> Option<Direction3> {
        match self.0.cmp(&towards.0) {
            Ordering::Less => {
                if self.1 == towards.1 && self.2 == towards.2 {
                    Some(Direction3::Right)
                } else {
                    None
                }
            }
            Ordering::Equal => match self.1.cmp(&towards.1) {
                Ordering::Less => {
                    if self.2 == towards.2 {
                        Some(Direction3::Down)
                    } else {
                        None
                    }
                }
                Ordering::Equal => match self.2.cmp(&towards.2) {
                    Ordering::Less => Some(Direction3::Back),
                    Ordering::Equal => None,
                    Ordering::Greater => Some(Direction3::Front),
                },
                Ordering::Greater => {
                    if self.2 == towards.2 {
                        Some(Direction3::Up)
                    } else {
                        None
                    }
                }
            },
            Ordering::Greater => {
                if self.1 == towards.1 && self.2 == towards.2 {
                    Some(Direction3::Left)
                } else {
                    None
                }
            }
        }
    }
    pub fn componentwise_min(self, other: Self) -> Self {
        Self(
            min(self.0, other.0),
            min(self.1, other.1),
            min(self.2, other.2),
        )
    }
    pub fn componentwise_max(self, other: Self) -> Self {
        Self(
            max(self.0, other.0),
            max(self.1, other.1),
            max(self.2, other.2),
        )
    }
}

impl<T> Point3<T>
where
    T: Add<T, Output = T> + Copy + Sub<T, Output = T>,
{
    pub fn towards(self, towards: Direction3, distance: T) -> Self {
        match towards {
            Direction3::Right => self.with_x(self.0 + distance),
            Direction3::Down => self.with_y(self.1 + distance),
            Direction3::Back => self.with_z(self.2 + distance),
            Direction3::Left => self.with_x(self.0 - distance),
            Direction3::Up => self.with_y(self.1 - distance),
            Direction3::Front => self.with_z(self.2 - distance),
        }
    }
}

impl<T> Point3<T>
where
    T: OverflowingAdd + Copy + OverflowingSub,
{
    pub fn try_towards(self, towards: Direction3, distance: T) -> Option<Self> {
        match towards {
            Direction3::Right => {
                let (new_x, overflow) = self.0.overflowing_add(&distance);
                if overflow {
                    None
                } else {
                    Some(self.with_x(new_x))
                }
            }
            Direction3::Down => {
                let (new_y, overflow) = self.1.overflowing_add(&distance);
                if overflow {
                    None
                } else {
                    Some(self.with_y(new_y))
                }
            }
            Direction3::Back => {
                let (new_z, overflow) = self.2.overflowing_add(&distance);
                if overflow {
                    None
                } else {
                    Some(self.with_z(new_z))
                }
            }
            Direction3::Left => {
                let (new_x, overflow) = self.0.overflowing_sub(&distance);
                if overflow {
                    None
                } else {
                    Some(self.with_x(new_x))
                }
            }
            Direction3::Up => {
                let (new_y, overflow) = self.1.overflowing_sub(&distance);
                if overflow {
                    None
                } else {
                    Some(self.with_y(new_y))
                }
            }
            Direction3::Front => {
                let (new_z, overflow) = self.2.overflowing_sub(&distance);
                if overflow {
                    None
                } else {
                    Some(self.with_z(new_z))
                }
            }
        }
    }
}

impl<T> Point3<T>
where
    T: Add<T, Output = T> + Copy + One + Sub<T, Output = T>,
{
    pub fn next_towards(self, towards: Direction3) -> Self {
        self.towards(towards, one())
    }
}

impl<T> Point3<T>
where
    T: OverflowingAdd + Copy + One + OverflowingSub,
{
    pub fn try_next_towards(self, towards: Direction3) -> Option<Self> {
        self.try_towards(towards, one())
    }
}

impl<T> Point3<T>
where
    T: Add<T, Output = T> + Copy + One,
{
    pub fn next_right(self) -> Self {
        self.with_x(self.0 + one())
    }
    pub fn next_down(self) -> Self {
        self.with_y(self.1 + one())
    }
    pub fn next_back(self) -> Self {
        self.with_z(self.2 + one())
    }
}

impl<T> Point3<T>
where
    T: OverflowingAdd + Copy + One,
{
    pub fn try_next_right(self) -> Option<Self> {
        let (new_x, overflow) = self.0.overflowing_add(&one());
        if overflow {
            None
        } else {
            Some(self.with_x(new_x))
        }
    }
    pub fn try_next_down(self) -> Option<Self> {
        let (new_y, overflow) = self.1.overflowing_add(&one());
        if overflow {
            None
        } else {
            Some(self.with_y(new_y))
        }
    }
    pub fn try_next_back(self) -> Option<Self> {
        let (new_z, overflow) = self.2.overflowing_add(&one());
        if overflow {
            None
        } else {
            Some(self.with_z(new_z))
        }
    }
}

impl<T> Point3<T>
where
    T: Sub<T, Output = T> + Copy + One,
{
    pub fn next_left(self) -> Self {
        self.with_x(self.0 - one())
    }
    pub fn next_up(self) -> Self {
        self.with_y(self.1 - one())
    }
    pub fn next_front(self) -> Self {
        self.with_z(self.2 - one())
    }
}

impl<T> Point3<T>
where
    T: OverflowingSub + Copy + One,
{
    pub fn try_next_left(self) -> Option<Self> {
        let (new_x, overflow) = self.0.overflowing_sub(&one());
        if overflow {
            None
        } else {
            Some(self.with_x(new_x))
        }
    }
    pub fn try_next_up(self) -> Option<Self> {
        let (new_y, overflow) = self.1.overflowing_sub(&one());
        if overflow {
            None
        } else {
            Some(self.with_y(new_y))
        }
    }
    pub fn try_next_front(self) -> Option<Self> {
        let (new_z, overflow) = self.2.overflowing_sub(&one());
        if overflow {
            None
        } else {
            Some(self.with_z(new_z))
        }
    }
}

impl<T> PartialOrd for Point3<T>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if let Some(cmp_x) = self.0.partial_cmp(&other.0) {
            if let Some(cmp_y) = self.1.partial_cmp(&other.1) {
                if let Some(cmp_z) = self.2.partial_cmp(&other.2) {
                    if cmp_x == cmp_y && cmp_x == cmp_z {
                        return Some(cmp_x);
                    }
                }
            }
        }
        None
    }
}

impl<T, U> Add<Point3<U>> for Point3<T>
where
    T: Add<U>,
{
    type Output = Point3<<T as Add<U>>::Output>;

    fn add(self, rhs: Point3<U>) -> Self::Output {
        Point3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl<T, U> AddAssign<Point3<U>> for Point3<T>
where
    T: AddAssign<U>,
{
    fn add_assign(&mut self, rhs: Point3<U>) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl<T, U> Sub<Point3<U>> for Point3<T>
where
    T: Sub<U>,
{
    type Output = Point3<<T as Sub<U>>::Output>;

    fn sub(self, rhs: Point3<U>) -> Self::Output {
        Point3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl<T, U> SubAssign<Point3<U>> for Point3<T>
where
    T: SubAssign<U>,
{
    fn sub_assign(&mut self, rhs: Point3<U>) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
        self.2 -= rhs.2;
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Direction2 {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

impl Direction2 {
    pub fn all() -> impl Iterator<Item = Self> {
        (0u8..=3).map(|dir| Self::try_from(dir).unwrap())
    }
    pub fn clockwise(self) -> Self {
        match self {
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Up => Self::Right,
        }
    }
    pub fn counterclockwise(self) -> Self {
        match self {
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Up => Self::Left,
        }
    }
}

impl TryFrom<u8> for Direction2 {
    type Error = Box<dyn Error>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Right),
            1 => Ok(Self::Down),
            2 => Ok(Self::Left),
            3 => Ok(Self::Up),
            _ => Err(Box::from("invalid direction")),
        }
    }
}

impl<T> From<Direction2> for Point2<T>
where
    T: Signed,
{
    fn from(value: Direction2) -> Self {
        match value {
            Direction2::Right => Point2(one(), zero()),
            Direction2::Down => Point2(zero(), one()),
            Direction2::Left => Point2(-one::<T>(), zero()),
            Direction2::Up => Point2(zero(), -one::<T>()),
        }
    }
}

impl Neg for Direction2 {
    type Output = Direction2;

    fn neg(self) -> Self::Output {
        match self {
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Up => Self::Down,
        }
    }
}

impl TryFrom<char> for Direction2 {
    type Error = Box<dyn Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Self::Right),
            'D' => Ok(Self::Down),
            'L' => Ok(Self::Left),
            'U' => Ok(Self::Up),
            _ => Err(Box::from("invalid direction character")),
        }
    }
}

impl TryFrom<Direction3> for Direction2 {
    type Error = Box<dyn Error>;

    fn try_from(value: Direction3) -> Result<Self, Self::Error> {
        match value {
            Direction3::Right => Ok(Self::Right),
            Direction3::Down => Ok(Self::Down),
            Direction3::Left => Ok(Self::Left),
            Direction3::Up => Ok(Self::Up),
            _ => Err(Box::from("not a 2D direction")),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Direction3 {
    Right = 0,
    Down = 1,
    Back = 2,
    Left = 3,
    Up = 4,
    Front = 5,
}

impl Direction3 {
    pub fn all() -> impl Iterator<Item = Self> {
        (0u8..=5).map(|dir| Self::try_from(dir).unwrap())
    }
    pub fn cross(self, other: Self) -> Option<Self> {
        if self == other || self == -other {
            None
        } else {
            let neg_self =
                self == Direction3::Left || self == Direction3::Up || self == Direction3::Front;
            let neg_other =
                other == Direction3::Left || other == Direction3::Up || other == Direction3::Front;
            if neg_self {
                if neg_other {
                    Some((-self).cross_internal(-other))
                } else {
                    Some(-((-self).cross_internal(other)))
                }
            } else {
                if neg_other {
                    Some(-(self.cross_internal(-other)))
                } else {
                    Some(self.cross_internal(other))
                }
            }
        }
    }
    fn cross_internal(self, other: Self) -> Self {
        match (self, other) {
            (Direction3::Right, Direction3::Down) => Direction3::Back,
            (Direction3::Right, Direction3::Back) => Direction3::Up,
            (Direction3::Down, Direction3::Right) => Direction3::Front,
            (Direction3::Down, Direction3::Back) => Direction3::Right,
            (Direction3::Back, Direction3::Right) => Direction3::Down,
            (Direction3::Back, Direction3::Down) => Direction3::Left,
            _ => unreachable!(),
        }
    }
}

impl TryFrom<u8> for Direction3 {
    type Error = Box<dyn Error>;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Right),
            1 => Ok(Self::Down),
            2 => Ok(Self::Back),
            3 => Ok(Self::Left),
            4 => Ok(Self::Up),
            5 => Ok(Self::Front),
            _ => Err(Box::from("invalid direction")),
        }
    }
}

impl<T> From<Direction3> for Point3<T>
where
    T: Signed,
{
    fn from(value: Direction3) -> Self {
        match value {
            Direction3::Right => Point3(one(), zero(), zero()),
            Direction3::Down => Point3(zero(), one(), zero()),
            Direction3::Back => Point3(zero(), zero(), one()),
            Direction3::Left => Point3(-one::<T>(), zero(), zero()),
            Direction3::Up => Point3(zero(), -one::<T>(), zero()),
            Direction3::Front => Point3(zero(), zero(), -one::<T>()),
        }
    }
}

impl Neg for Direction3 {
    type Output = Direction3;

    fn neg(self) -> Self::Output {
        match self {
            Self::Right => Self::Left,
            Self::Down => Self::Up,
            Self::Back => Self::Front,
            Self::Left => Self::Right,
            Self::Up => Self::Down,
            Self::Front => Self::Back,
        }
    }
}

impl TryFrom<char> for Direction3 {
    type Error = Box<dyn Error>;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'R' => Ok(Self::Right),
            'D' => Ok(Self::Down),
            'B' => Ok(Self::Back),
            'L' => Ok(Self::Left),
            'U' => Ok(Self::Up),
            'F' => Ok(Self::Front),
            _ => Err(Box::from("invalid direction character")),
        }
    }
}

impl From<Direction2> for Direction3 {
    fn from(value: Direction2) -> Self {
        match value {
            Direction2::Right => Self::Right,
            Direction2::Down => Self::Down,
            Direction2::Left => Self::Left,
            Direction2::Up => Self::Up,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Directional2<T> {
    pub right: T,
    pub down: T,
    pub left: T,
    pub up: T,
}

impl<T> From<T> for Directional2<T>
where
    T: Clone,
{
    fn from(value: T) -> Self {
        Self {
            right: value.clone(),
            down: value.clone(),
            left: value.clone(),
            up: value,
        }
    }
}

impl<T> Index<Direction2> for Directional2<T> {
    type Output = T;

    fn index(&self, index: Direction2) -> &Self::Output {
        match index {
            Direction2::Right => &self.right,
            Direction2::Down => &self.down,
            Direction2::Left => &self.left,
            Direction2::Up => &self.up,
        }
    }
}

impl<T> IndexMut<Direction2> for Directional2<T> {
    fn index_mut(&mut self, index: Direction2) -> &mut Self::Output {
        match index {
            Direction2::Right => &mut self.right,
            Direction2::Down => &mut self.down,
            Direction2::Left => &mut self.left,
            Direction2::Up => &mut self.up,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Default)]
pub struct Directional3<T> {
    pub right: T,
    pub down: T,
    pub back: T,
    pub left: T,
    pub up: T,
    pub front: T,
}

impl<T> From<T> for Directional3<T>
where
    T: Clone,
{
    fn from(value: T) -> Self {
        Self {
            right: value.clone(),
            down: value.clone(),
            back: value.clone(),
            left: value.clone(),
            up: value.clone(),
            front: value,
        }
    }
}

impl<T> Index<Direction3> for Directional3<T> {
    type Output = T;

    fn index(&self, index: Direction3) -> &Self::Output {
        match index {
            Direction3::Right => &self.right,
            Direction3::Down => &self.down,
            Direction3::Back => &self.back,
            Direction3::Left => &self.left,
            Direction3::Up => &self.up,
            Direction3::Front => &self.front,
        }
    }
}

impl<T> IndexMut<Direction3> for Directional3<T> {
    fn index_mut(&mut self, index: Direction3) -> &mut Self::Output {
        match index {
            Direction3::Right => &mut self.right,
            Direction3::Down => &mut self.down,
            Direction3::Back => &mut self.back,
            Direction3::Left => &mut self.left,
            Direction3::Up => &mut self.up,
            Direction3::Front => &mut self.front,
        }
    }
}
