use std::error::Error;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, u64};
use nom::combinator::{map, opt};
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded};

use super::{parse_full_string, LineStreamHandler, GOLD_ANSI, SILVER_ANSI};

mod state;
use state::{Monkey, Operation, State};

#[derive(Default)]
struct Day11 {
    gold: bool,
    verbosity: u8,
    current: usize,
    state: State,
}

impl Day11 {
    fn new(gold: bool, verbosity: u8) -> Self {
        Self {
            gold,
            verbosity,
            ..Default::default()
        }
    }
    fn play_rounds(&mut self, rounds: u64) {
        for _ in 0..rounds {
            self.state.play_round();
            if self.verbosity > 2 {
                self.state.dump_items();
            }
            if self.verbosity > 1 {
                self.state.dump_inspections();
            }
        }
    }
    fn select_monkey(&mut self, monkey: usize) {
        self.state.ensure_monkey_exists(monkey);
        self.current = monkey;
    }
    fn current_monkey_mut(&mut self) -> &mut Monkey {
        &mut self.state.monkeys[self.current]
    }
}

enum Line {
    SelectMonkey(usize),
    SetItems(Vec<u64>),
    SetOperation(Operation),
    SetTestDivisibleBy(u64),
    SetNextIfTrue(usize),
    SetNextIfFalse(usize),
}

impl Line {
    fn update_state(self, state: &mut Day11) -> Result<(), Box<dyn Error>> {
        Ok(match self {
            Self::SelectMonkey(n) => {
                state.select_monkey(n);
            }
            Self::SetItems(items) => {
                state.current_monkey_mut().items = items;
            }
            Self::SetOperation(operation) => {
                state.current_monkey_mut().operation = operation;
            }
            Self::SetTestDivisibleBy(divisor) => {
                state.current_monkey_mut().test_divisible_by = divisor;
            }
            Self::SetNextIfTrue(monkey) => {
                state.state.ensure_monkey_exists(monkey);
                state.current_monkey_mut().next_if_true = monkey;
            }
            Self::SetNextIfFalse(monkey) => {
                state.state.ensure_monkey_exists(monkey);
                state.current_monkey_mut().next_if_false = monkey;
            }
        })
    }
}

impl LineStreamHandler for Day11 {
    fn update(&mut self, line: &str) -> Result<Option<Box<dyn LineStreamHandler>>, Box<dyn Error>> {
        let parsed_line = parse_full_string(
            line,
            opt(alt((
                map(delimited(tag("Monkey "), u64, char(':')), |monkey| {
                    Line::SelectMonkey(monkey as usize)
                }),
                map(
                    preceded(tag("  Starting items: "), separated_list0(tag(", "), u64)),
                    |items| Line::SetItems(items),
                ),
                map(
                    preceded(
                        tag("  Operation: new = old "),
                        alt((
                            map(preceded(tag("+ "), u64), |n| Operation::Add(n)),
                            preceded(
                                tag("* "),
                                alt((
                                    map(u64, |n| Operation::Mul(n)),
                                    map(tag("old"), |_| Operation::Square),
                                )),
                            ),
                        )),
                    ),
                    |operation| Line::SetOperation(operation),
                ),
                map(preceded(tag("  Test: divisible by "), u64), |divisor| {
                    Line::SetTestDivisibleBy(divisor)
                }),
                map(
                    preceded(tag("    If true: throw to monkey "), u64),
                    |monkey| Line::SetNextIfTrue(monkey as usize),
                ),
                map(
                    preceded(tag("    If false: throw to monkey "), u64),
                    |monkey| Line::SetNextIfFalse(monkey as usize),
                ),
            ))),
        )?;

        if let Some(l) = parsed_line {
            l.update_state(self)?;
        }

        Ok(None)
    }

    fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        self.state.determine_post_operation(self.gold);
        self.play_rounds(if self.gold { 10000 } else { 20 });

        if self.verbosity == 1 {
            self.state.dump_inspections();
        }
        println!(
            "[{}] Monkey business level: {}",
            if self.gold { GOLD_ANSI } else { SILVER_ANSI },
            self.state.monkey_business_level()
        );

        Ok(())
    }
}

pub fn new(
    gold: bool,
    verbosity: u8,
) -> Result<(u8, &'static str, Box<dyn LineStreamHandler>), Box<dyn Error>> {
    Ok((
        11,
        "Monkey in the Middle",
        Box::new(Day11::new(gold, verbosity)),
    ))
}
