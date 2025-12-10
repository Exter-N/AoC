use std::error::Error;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{LineStreamHandler, parse_full_string},
};
#[cfg(feature = "z3")]
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{char, one_of, u32, usize},
    combinator::map,
    multi::{fold_many0, separated_list1},
    sequence::{preceded, separated_pair, terminated},
};
#[cfg(feature = "z3")]
use z3::{Solver, ast::Int};

#[derive(Debug)]
struct Machine {
    target_lights: usize,
    button_lights: Vec<usize>,
    #[cfg(feature = "z3")]
    joltages: Vec<u32>,
}

impl Machine {
    #[cfg(feature = "z3")]
    fn from_parser_output(value: (usize, (Vec<usize>, Vec<u32>))) -> Self {
        Self {
            target_lights: value.0,
            button_lights: value.1.0,
            joltages: value.1.1,
        }
    }

    #[cfg(not(feature = "z3"))]
    fn from_parser_output(value: (usize, (Vec<usize>, Vec<u32>))) -> Self {
        Self {
            target_lights: value.0,
            button_lights: value.1.0,
        }
    }

    fn combination_lights(&self, combination: usize) -> usize {
        let mut combination_lights = 0usize;
        for (lights, i) in self.button_lights.iter().zip(0usize..) {
            if combination & (1 << i) != 0 {
                combination_lights ^= *lights;
            }
        }
        combination_lights
    }

    fn min_presses_silver(&self) -> u32 {
        let mut min_presses = u32::MAX;
        for combination in 0usize..(1 << self.button_lights.len()) {
            let combination_lights = self.combination_lights(combination);
            if combination_lights == self.target_lights {
                min_presses = min_presses.min(combination.count_ones());
            }
        }
        min_presses
    }

    #[cfg(feature = "z3")]
    fn min_presses_gold(&self) -> u32 {
        let buttons = (0..self.button_lights.len())
            .map(|_| Int::fresh_const("button"))
            .collect_vec();
        let solver = Solver::new();
        for button in buttons.iter() {
            solver.assert(button.ge(0));
        }
        for (joltage, i) in self.joltages.iter().zip(0usize..) {
            let buttons = self
                .button_lights
                .iter()
                .zip(buttons.iter())
                .filter_map(|(lights, button)| {
                    if (*lights & (1 << i)) != 0 {
                        Some(button)
                    } else {
                        None
                    }
                })
                .collect_vec();
            solver.assert(Int::add(&buttons).eq(*joltage));
        }
        let mut min_presses = u32::MAX;
        for solution in solver.into_solutions(buttons, false) {
            min_presses = min_presses.min(
                solution
                    .into_iter()
                    .map(|x| x.as_u64().unwrap() as u32)
                    .sum::<u32>(),
            );
        }
        min_presses
    }

    #[cfg(not(feature = "z3"))]
    fn min_presses_gold(&self) -> u32 {
        unimplemented!("This requires the \"z3\" feature.")
    }
}

fn parse_machine(line: &str) -> Result<Machine, nom::error::Error<usize>> {
    parse_full_string(
        line,
        map(
            preceded(
                char('['),
                terminated(
                    separated_pair(
                        map(
                            fold_many0(
                                one_of(".#"),
                                || (0usize, 0usize),
                                |(acc, len), ch| {
                                    (
                                        match ch {
                                            '.' => acc,
                                            '#' => acc | (1 << len),
                                            _ => unreachable!(),
                                        },
                                        len + 1,
                                    )
                                },
                            ),
                            |(result, _)| result,
                        ),
                        tag("] ("),
                        separated_pair(
                            separated_list1(
                                tag(") ("),
                                map(separated_list1(char(','), usize), |vec| {
                                    vec.into_iter().fold(0usize, |acc, n| acc | (1 << n))
                                }),
                            ),
                            tag(") {"),
                            separated_list1(char(','), u32),
                        ),
                    ),
                    char('}'),
                ),
            ),
            Machine::from_parser_output,
        ),
    )
}

struct Day10 {
    min_presses: u32,
    gold: bool,
}

impl Day10 {
    fn new(gold: bool) -> Self {
        Self {
            min_presses: 0,
            gold,
        }
    }
}

impl LineStreamHandler for Day10 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let machine = parse_machine(line)?;
        self.min_presses += if self.gold {
            machine.min_presses_gold()
        } else {
            machine.min_presses_silver()
        };
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Minimum button presses: {}",
            if self.gold { GOLD_ANSI } else { SILVER_ANSI },
            self.min_presses
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(10, "Factory", Day10::new(gold)))
}
