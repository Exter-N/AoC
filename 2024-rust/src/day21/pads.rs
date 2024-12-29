use std::{
    cmp::Ordering,
    collections::{hash_map::Entry, HashMap},
    fmt::{Display, Formatter, Result as FmtResult, Write},
    hash::Hash,
    mem::replace,
    ops::{Add, AddAssign},
};

use aoc_common_rs::{digit::Digit, point::Direction2};

pub trait Key
where
    Self: Sized,
{
    fn all() -> impl Iterator<Item = Self>;

    fn next_towards(self, direction: Direction2) -> Option<Self>;
}

#[derive(Clone, Debug)]
pub struct KeyPressCost {
    pub cost: usize,
}

impl KeyPressCost {
    pub fn zero() -> Self {
        Self { cost: 0 }
    }

    pub fn one() -> Self {
        Self { cost: 1 }
    }
}

impl PartialEq for KeyPressCost {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for KeyPressCost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cost.partial_cmp(&other.cost)
    }
}

impl Add for KeyPressCost {
    type Output = KeyPressCost;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            cost: self.cost + rhs.cost,
        }
    }
}

impl AddAssign for KeyPressCost {
    fn add_assign(&mut self, rhs: Self) {
        self.cost += rhs.cost;
    }
}

impl Display for KeyPressCost {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        Display::fmt(&self.cost, f)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum DpadKey {
    Direction(Direction2),
    A,
}

impl DpadKey {
    pub fn initial_cost_matrix() -> HashMap<(DpadKey, DpadKey), KeyPressCost> {
        let mut matrix = HashMap::new();
        for from in Self::all() {
            for to in Self::all() {
                matrix.insert((from, to), KeyPressCost::one());
            }
        }
        matrix
    }
}

impl Key for DpadKey {
    fn all() -> impl Iterator<Item = Self> {
        (0u8..=4).map(|key| {
            if key == 0 {
                Self::A
            } else {
                Direction2::try_from(key - 1).unwrap().into()
            }
        })
    }

    fn next_towards(self, direction: Direction2) -> Option<Self> {
        match (self, direction) {
            (Self::Direction(Direction2::Up), Direction2::Right) => Some(Self::A),
            (Self::Direction(Direction2::Up), Direction2::Down) => Some(Direction2::Down.into()),
            (Self::Direction(Direction2::Left), Direction2::Right) => Some(Direction2::Down.into()),
            (Self::Direction(Direction2::Down), dir) => {
                if matches!(dir, Direction2::Down) {
                    None
                } else {
                    Some(dir.into())
                }
            }
            (Self::Direction(Direction2::Right), Direction2::Up) => Some(Self::A),
            (Self::Direction(Direction2::Right), Direction2::Left) => Some(Direction2::Down.into()),
            (Self::A, Direction2::Left) => Some(Direction2::Up.into()),
            (Self::A, Direction2::Down) => Some(Direction2::Right.into()),
            (_, _) => None,
        }
    }
}

impl From<Direction2> for DpadKey {
    fn from(value: Direction2) -> Self {
        Self::Direction(value)
    }
}

impl From<DpadKey> for char {
    fn from(value: DpadKey) -> Self {
        match value {
            DpadKey::Direction(Direction2::Up) => '^',
            DpadKey::Direction(Direction2::Right) => '>',
            DpadKey::Direction(Direction2::Down) => 'v',
            DpadKey::Direction(Direction2::Left) => '<',
            DpadKey::A => 'A',
        }
    }
}

impl TryFrom<char> for DpadKey {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction2::Up.into()),
            '>' => Ok(Direction2::Right.into()),
            'v' => Ok(Direction2::Down.into()),
            '<' => Ok(Direction2::Left.into()),
            'A' => Ok(DpadKey::A),
            _ => Err(()),
        }
    }
}

impl Display for DpadKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_char(char::from(*self))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum NumpadKey {
    Digit(Digit<10>),
    A,
}

impl NumpadKey {
    fn digit(value: u8) -> Option<Self> {
        Digit::new(value).map(|digit| Self::Digit(digit))
    }
}

impl Key for NumpadKey {
    fn all() -> impl Iterator<Item = Self> {
        (0u8..=10).map(|key| {
            if key == 0 {
                Self::A
            } else {
                Digit::new(key - 1).unwrap().into()
            }
        })
    }

    fn next_towards(self, direction: Direction2) -> Option<Self> {
        match self {
            NumpadKey::Digit(digit) => match (digit.get(), direction) {
                (1 | 2 | 3 | 4 | 5 | 6, Direction2::Up) => Self::digit(digit.get() + 3),
                (1 | 2 | 4 | 5 | 7 | 8, Direction2::Right) => Self::digit(digit.get() + 1),
                (2 | 3 | 5 | 6 | 8 | 9, Direction2::Left) => Self::digit(digit.get() - 1),
                (4 | 5 | 6 | 7 | 8 | 9, Direction2::Down) => Self::digit(digit.get() - 3),
                (2, Direction2::Down) => Self::digit(0),
                (3, Direction2::Down) => Some(Self::A),
                (0, Direction2::Up) => Self::digit(2),
                (0, Direction2::Right) => Some(Self::A),
                (_, _) => None,
            },
            NumpadKey::A => match direction {
                Direction2::Left => Self::digit(0),
                Direction2::Up => Self::digit(3),
                _ => None,
            },
        }
    }
}

impl From<Digit<10>> for NumpadKey {
    fn from(value: Digit<10>) -> Self {
        Self::Digit(value)
    }
}

impl From<NumpadKey> for char {
    fn from(value: NumpadKey) -> Self {
        match value {
            NumpadKey::Digit(digit) => digit.into(),
            NumpadKey::A => 'A',
        }
    }
}

impl TryFrom<char> for NumpadKey {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        if let Ok(digit) = Digit::try_from(value) {
            Ok(Self::Digit(digit))
        } else if value == 'A' {
            Ok(Self::A)
        } else {
            Err(())
        }
    }
}

impl Display for NumpadKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_char(char::from(*self))
    }
}

pub fn next_cost_matrix<T>(
    previous: &HashMap<(DpadKey, DpadKey), KeyPressCost>,
) -> HashMap<(T, T), KeyPressCost>
where
    T: Key + Copy + Eq + Hash,
{
    let mut matrix = HashMap::new();
    for from in T::all() {
        let mut submatrix = HashMap::new();
        let mut best_path_costs = HashMap::new();
        submatrix.insert((from, DpadKey::A), KeyPressCost::zero());
        while !submatrix.is_empty() {
            for ((to, last), cost_so_far) in replace(&mut submatrix, HashMap::new()) {
                let closing_cost = previous.get(&(last, DpadKey::A)).unwrap();
                let total_cost = cost_so_far.clone() + closing_cost.clone();
                matrix
                    .entry((from, to))
                    .and_modify(|e_cost| {
                        if &total_cost < e_cost {
                            *e_cost = total_cost.clone();
                        }
                    })
                    .or_insert(total_cost);
                let better = match best_path_costs.entry((to, last)) {
                    Entry::Occupied(entry) => {
                        let e_cost = entry.into_mut();
                        if &cost_so_far < e_cost {
                            *e_cost = cost_so_far.clone();
                            true
                        } else {
                            false
                        }
                    }
                    Entry::Vacant(entry) => {
                        entry.insert(cost_so_far.clone());
                        true
                    }
                };
                if better {
                    for dir in Direction2::all() {
                        if let Some(neighbor) = to.next_towards(dir) {
                            let dir_key = DpadKey::Direction(dir);
                            let edge_cost = previous.get(&(last, dir_key)).unwrap();
                            let neighbor_cost = cost_so_far.clone() + edge_cost.clone();
                            submatrix
                                .entry((neighbor, dir_key))
                                .and_modify(|e_cost| {
                                    if &neighbor_cost < e_cost {
                                        *e_cost = neighbor_cost.clone();
                                    }
                                })
                                .or_insert(neighbor_cost);
                        }
                    }
                }
            }
        }
    }
    matrix
}
