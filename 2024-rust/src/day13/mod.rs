use std::{error::Error, fmt::Display};

use aoc_common_rs::{
    day::Day,
    line_stream::{parse_full_string, LineStreamHandler},
    math::{solve_linear_diophantine, LinearDiophantineSolution},
    mem::take_all3,
};
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
}

fn print_dioph_sol<T: Display>(a: T, b: T, c: T, sol: LinearDiophantineSolution<T>) {
    println!(
        "{} = {} * ({} + {}k) + {} * ({} + {}k)",
        c, a, sol.x, sol.x_step, b, sol.y, sol.y_step
    );
}

fn solve_print_dioph(a: i64, b: i64, c: i64) {
    match solve_linear_diophantine(a, b, c) {
        Some(sol) => {
            print_dioph_sol(a, b, c, sol);
        }
        None => {
            println!("{} = {} * ??? + {} * ???", c, a, b);
        }
    }
}

impl Day13 {
    fn new() -> Self {
        Self {
            button_a: None,
            button_b: None,
            prize: None,
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
        let xds = match solve_linear_diophantine(button_a.0, button_b.0, prize.0) {
            Some(s) => s,
            None => {
                return;
            }
        };
        let yds = match solve_linear_diophantine(button_a.1, button_b.1, prize.1) {
            Some(s) => s,
            None => {
                return;
            }
        };
        println!("A:     {:?}", button_a);
        println!("B:     {:?}", button_b);
        println!("Prize: {:?}", prize);

        print_dioph_sol(button_a.0, button_b.0, prize.0, xds);
        print_dioph_sol(button_a.1, button_b.1, prize.1, yds);

        solve_print_dioph(xds.x_step, -yds.x_step, yds.x - xds.x);
        solve_print_dioph(xds.y_step, -yds.y_step, yds.y - xds.y);
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

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(13, "Claw Contraption", Day13::new()))
}
