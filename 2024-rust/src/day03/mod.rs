use std::error::Error;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, u32},
    combinator::map,
    multi::fold_many1,
    sequence::{preceded, separated_pair, terminated},
};

struct Day3 {
    accumulator: Accumulator,
}

impl Day3 {
    fn new(gold: bool) -> Self {
        Self {
            accumulator: Accumulator::new(gold),
        }
    }
}

#[derive(Clone, Copy)]
enum Instruction {
    Add(u32),
    Disable,
    Enable,
}

#[derive(Clone, Copy)]
struct Accumulator {
    can_disable: bool,
    enabled: bool,
    sum: u32,
}

impl Accumulator {
    fn new(can_disable: bool) -> Self {
        Self {
            can_disable,
            enabled: true,
            sum: 0,
        }
    }
    fn with(self, instruction: Instruction) -> Self {
        match instruction {
            Instruction::Add(num) => {
                if self.enabled {
                    Self {
                        sum: self.sum + num,
                        ..self
                    }
                } else {
                    self
                }
            }
            Instruction::Disable => {
                if self.can_disable {
                    Self {
                        enabled: false,
                        ..self
                    }
                } else {
                    self
                }
            }
            Instruction::Enable => Self {
                enabled: true,
                ..self
            },
        }
    }
}

impl LineStreamHandler for Day3 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        self.accumulator = parse_full_string(
            line,
            fold_many1(
                alt((
                    map(
                        preceded(
                            tag("mul("),
                            terminated(separated_pair(u32, char(','), u32), char(')')),
                        ),
                        |(left, right)| Instruction::Add(left * right),
                    ),
                    map(tag("don't()"), |_| Instruction::Disable),
                    map(tag("do()"), |_| Instruction::Enable),
                    map(anychar, |_| Instruction::Add(0)),
                )),
                || self.accumulator,
                |acc, instruction| acc.with(instruction),
            ),
        )?;
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Sum of products: {}",
            if self.accumulator.can_disable {
                GOLD_ANSI
            } else {
                SILVER_ANSI
            },
            self.accumulator.sum
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(3, "Mull It Over", Day3::new(gold)))
}
