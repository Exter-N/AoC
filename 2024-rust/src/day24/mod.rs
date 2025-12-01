use std::error::Error;

use aoc_common_rs::{
    cc::ThreeCC,
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char},
    combinator::map,
    error::Error as NomError,
    sequence::separated_pair,
};

mod circuit;

use circuit::{Circuit, Wire};

#[derive(Debug)]
struct Day24 {
    circuit: Circuit,
}

impl Day24 {
    fn new() -> Self {
        Self {
            circuit: Circuit::new(),
        }
    }
}

impl LineStreamHandler for Day24 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if line.is_empty() {
            return Ok(());
        }

        let three_cc = move || {
            map(
                (anychar::<&str, NomError<&str>>, anychar, anychar),
                ThreeCC::from,
            )
        };
        let (definition, code) = parse_full_string(
            line,
            alt((
                map(
                    separated_pair(
                        three_cc(),
                        tag(": "),
                        alt((map(char('0'), |_| false), map(char('1'), |_| true))),
                    ),
                    |(code, value)| (Wire::Literal(value), code),
                ),
                separated_pair(
                    alt((
                        map(
                            separated_pair(three_cc(), tag(" AND "), three_cc()),
                            |(op1, op2)| Wire::And(op1, op2),
                        ),
                        map(
                            separated_pair(three_cc(), tag(" OR "), three_cc()),
                            |(op1, op2)| Wire::Or(op1, op2),
                        ),
                        map(
                            separated_pair(three_cc(), tag(" XOR "), three_cc()),
                            |(op1, op2)| Wire::Xor(op1, op2),
                        ),
                    )),
                    tag(" -> "),
                    three_cc(),
                ),
            )),
        )?;
        self.circuit.define(code, definition);
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        let mut evaluator = self.circuit.clone();
        println!("[-] Number on x wires: {}", evaluator.evaluate_number('x'));
        println!("[-] Number on y wires: {}", evaluator.evaluate_number('y'));
        println!(
            "[{}] Number on z wires: {}",
            SILVER_ANSI,
            evaluator.evaluate_number('z')
        );
        let mut errors: Vec<_> = self.circuit.clone().fix_adder().into_iter().collect();
        errors.sort();
        print!("[{}] Crossed wires:     ", GOLD_ANSI);
        let mut first = true;
        for code in errors {
            if first {
                first = false;
                print!("{}", code);
            } else {
                print!(",{}", code);
            }
        }
        println!();
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(24, "Crossed Wires", Day24::new()))
}
