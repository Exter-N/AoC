use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use either::Either;

use aoc_common_rs::cc::FourCC;

#[derive(Clone, Debug)]
pub enum Operation {
    Const(i64),
    Add(FourCC, FourCC),
    Sub(FourCC, FourCC),
    Mul(FourCC, FourCC),
    Div(FourCC, FourCC),
}

impl Operation {
    pub fn resolve(&self, monkeys: &mut Monkeys) -> Option<i64> {
        let ops = self.ops();
        let left = ops.and_then(|(m1, _)| monkeys.resolve(m1));
        let right = ops.and_then(|(_, m2)| monkeys.resolve(m2));
        match self {
            Operation::Const(num) => Some(*num),
            Operation::Add(_, _) => left.and_then(|v1| right.map(|v2| v1 + v2)),
            Operation::Sub(_, _) => left.and_then(|v1| right.map(|v2| v1 - v2)),
            Operation::Mul(_, _) => left.and_then(|v1| right.map(|v2| v1 * v2)),
            Operation::Div(_, _) => left.and_then(|v1| right.map(|v2| v1 / v2)),
        }
    }
    pub fn assert(&self, value: i64, monkeys: &mut Monkeys) -> bool {
        let ops = self.ops();
        let left = ops.and_then(|(m1, _)| monkeys.resolve(m1));
        let right = ops.and_then(|(_, m2)| monkeys.resolve(m2));
        let operand = if let Some(num) = left {
            if right.is_some() {
                return false;
            }

            Either::Left(num)
        } else if let Some(num) = right {
            Either::Right(num)
        } else {
            return false;
        };
        match self {
            Operation::Const(_) => false,
            Operation::Add(m1, m2) => match operand {
                Either::Left(num) /* num + ? = value */ => monkeys.assert(*m2, value - num),
                Either::Right(num) /* ? + num = value */ => monkeys.assert(*m1, value - num),
            },
            Operation::Sub(m1, m2) => match operand {
                Either::Left(num) /* num - ? = value */ => monkeys.assert(*m2, num - value),
                Either::Right(num) /* ? - num = value */ => monkeys.assert(*m1, value + num),
            },
            Operation::Mul(m1, m2) => match operand {
                Either::Left(num) /* num * ? = value */ => {
                    if value % num != 0 {
                        false
                    } else {
                        monkeys.assert(*m2, value / num)
                    }
                },
                Either::Right(num) /* ? * num = value */ => {
                    if value % num != 0 {
                        false
                    } else {
                        monkeys.assert(*m1, value / num)
                    }
                },
            },
            Operation::Div(m1, m2) => match operand {
                Either::Left(num) /* num / ? = value */ => {
                    if num % value != 0 {
                        false
                    } else {
                        monkeys.assert(*m2, num / value)
                    }
                },
                Either::Right(num) /* ? / num = value */ => monkeys.assert(*m1, value * num),
            },
        }
    }
    pub fn ops(&self) -> Option<(FourCC, FourCC)> {
        match self {
            Operation::Const(_) => None,
            Operation::Add(m1, m2) => Some((*m1, *m2)),
            Operation::Sub(m1, m2) => Some((*m1, *m2)),
            Operation::Mul(m1, m2) => Some((*m1, *m2)),
            Operation::Div(m1, m2) => Some((*m1, *m2)),
        }
    }
}

#[derive(Debug, Default)]
#[repr(transparent)]
pub struct Monkeys(HashMap<FourCC, Operation>);

impl Monkeys {
    pub fn resolve(&mut self, monkey: FourCC) -> Option<i64> {
        let op;
        if let Some(op_b) = self.0.get(&monkey) {
            op = op_b.to_owned();
        } else {
            return None;
        }

        if let Some(num) = op.resolve(self) {
            self.0.insert(monkey, Operation::Const(num));

            Some(num)
        } else {
            None
        }
    }
    pub fn assert(&mut self, monkey: FourCC, value: i64) -> bool {
        let op;
        if let Some(op_b) = self.0.get(&monkey) {
            op = op_b.to_owned();
        } else {
            self.0.insert(monkey, Operation::Const(value));

            return true;
        }

        op.assert(value, self)
    }
}

impl Deref for Monkeys {
    type Target = HashMap<FourCC, Operation>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Monkeys {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub const ROOT_MONKEY: FourCC = FourCC::new('r', 'o', 'o', 't');
pub const HUMAN: FourCC = FourCC::new('h', 'u', 'm', 'n');
