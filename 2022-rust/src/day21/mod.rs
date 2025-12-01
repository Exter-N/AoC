use std::error::Error;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, i64};
use nom::combinator::map;
use nom::error::Error as NomError;
use nom::sequence::separated_pair;

use aoc_common_rs::{
    cc::FourCC,
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};

mod monkeys;

use monkeys::{Monkeys, Operation, HUMAN, ROOT_MONKEY};

#[derive(Debug, Default)]
struct Day21 {
    gold: bool,
    monkeys: Monkeys,
}

impl Day21 {
    fn new(gold: bool) -> Self {
        Self {
            gold,
            ..Default::default()
        }
    }
}

impl LineStreamHandler for Day21 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let four_cc = move || {
            map(
                (anychar::<&str, NomError<&str>>, anychar, anychar, anychar),
                FourCC::from,
            )
        };
        let (id, op) = parse_full_string(
            line,
            separated_pair(
                four_cc(),
                tag(": "),
                alt((
                    map(i64, |num| Operation::Const(num)),
                    map(
                        (
                            four_cc(),
                            char(' '),
                            alt((char('+'), char('-'), char('*'), char('/'))),
                            char(' '),
                            four_cc(),
                        ),
                        |(monkey1, _, op, _, monkey2)| match op {
                            '+' => Operation::Add(monkey1, monkey2),
                            '-' => Operation::Sub(monkey1, monkey2),
                            '*' => Operation::Mul(monkey1, monkey2),
                            '/' => Operation::Div(monkey1, monkey2),
                            _ => panic!("got char that was not in the alt"),
                        },
                    ),
                )),
            ),
        )?;
        self.monkeys.insert(id, op);

        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        if self.gold {
            self.monkeys.remove(&HUMAN);
            if let Some(_) = self.monkeys.resolve(ROOT_MONKEY) {
                return Err(Box::from("root doesn't depend on you"));
            } else {
                let root_op = self.monkeys[&ROOT_MONKEY].to_owned();
                if let Some((left_id, right_id)) = root_op.ops() {
                    if !Operation::Sub(left_id, right_id).assert(0, &mut self.monkeys) {
                        return Err(Box::from("cannot assert equality of root's operands"));
                    }
                } else {
                    return Err(Box::from("root has no operands"));
                }
                if let Some(num) = self.monkeys.resolve(HUMAN) {
                    println!("[{}] You shall yell {}", GOLD_ANSI, num);
                } else {
                    return Err(Box::from("can't determine what you shall yell"));
                }
            }
        } else {
            if let Some(num) = self.monkeys.resolve(ROOT_MONKEY) {
                println!("[{}] Root will yell {}", SILVER_ANSI, num);
            } else {
                return Err(Box::from("can't determine what root will yell"));
            }
        }

        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(21, "Monkey Math", Day21::new(gold)))
}
