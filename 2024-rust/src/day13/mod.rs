use std::error::Error;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
    math::diophantine::LinearBivariateDiophantineEquation,
    mem::take_all3,
};
use itertools::Either;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::i64,
    combinator::map,
    sequence::{preceded, separated_pair},
};

enum Op {
    ButtonA,
    ButtonB,
    Prize,
}

#[derive(Debug)]
struct Day13 {
    button_a: Option<(i64, i64)>,
    button_b: Option<(i64, i64)>,
    prize: Option<(i64, i64)>,
    prize_offset: i64,
    total_tokens: i64,
}

impl Day13 {
    fn new(gold: bool) -> Self {
        Self {
            button_a: None,
            button_b: None,
            prize: None,
            prize_offset: if gold { 10_000_000_000_000 } else { 0 },
            total_tokens: 0,
        }
    }

    fn process(&mut self) {
        let (button_a, button_b, prize) =
            match take_all3(&mut self.button_a, &mut self.button_b, &mut self.prize) {
                Some(vals) => vals,
                None => {
                    return;
                }
            };

        let xde = LinearBivariateDiophantineEquation {
            a: button_a.0,
            b: button_b.0,
            c: prize.0 + self.prize_offset,
        };
        let yde = LinearBivariateDiophantineEquation {
            a: button_a.1,
            b: button_b.1,
            c: prize.1 + self.prize_offset,
        };

        let solution = match xde.solve_with(&yde) {
            Some(s) => s,
            None => {
                return;
            }
        };

        if let Either::Left(sol) = solution {
            if sol.0 >= 0 && sol.1 >= 0 {
                self.total_tokens += sol.0 * 3 + sol.1;
            }
            return;
        }

        todo!()
    }
}

impl LineStreamHandler for Day13 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if line.is_empty() {
            self.process();
        } else {
            let (op, args) = parse_full_string(
                line,
                alt((
                    map(
                        preceded(tag("Button A: X+"), separated_pair(i64, tag(", Y+"), i64)),
                        |vec| (Op::ButtonA, vec),
                    ),
                    map(
                        preceded(tag("Button B: X+"), separated_pair(i64, tag(", Y+"), i64)),
                        |vec| (Op::ButtonB, vec),
                    ),
                    map(
                        preceded(tag("Prize: X="), separated_pair(i64, tag(", Y="), i64)),
                        |vec| (Op::Prize, vec),
                    ),
                )),
            )?;
            match op {
                Op::ButtonA => {
                    self.button_a = Some(args);
                }
                Op::ButtonB => {
                    self.button_b = Some(args);
                }
                Op::Prize => {
                    self.prize = Some(args);
                }
            }
        }
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        self.process();
        println!(
            "[{}] Fewest tokens: {}",
            if self.prize_offset != 0 {
                GOLD_ANSI
            } else {
                SILVER_ANSI
            },
            self.total_tokens
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(13, "Claw Contraption", Day13::new(gold)))
}
