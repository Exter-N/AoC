use std::error::Error;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::combinator::map;
use nom::sequence::preceded;

use super::{parse_full_string, LineStreamHandler, GOLD_ANSI, SILVER_ANSI};

#[derive(Default)]
struct Day10 {
    verbose: bool,
    time: u32,
    register: i32,
    sum: i32,
    screen: [u64; 6],
}

impl Day10 {
    fn new(verbose: bool) -> Self {
        Self {
            verbose,
            register: 1,
            ..Default::default()
        }
    }
    fn tick(&mut self, time: u32) {
        for _ in 0..time {
            if self.time < 240 {
                let x = self.time % 40;
                if (x as i32).abs_diff(self.register) <= 1 {
                    self.screen[(self.time / 40) as usize] |= 1 << x;
                }
            }
            self.time += 1;
            if (self.time % 40) == 20 {
                self.sum += self.time as i32 * self.register;
                if self.verbose {
                    println!("[-] Cycle {}, X = {}", self.time, self.register);
                }
            }
        }
    }
    fn execute(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::NoOp => {
                self.tick(1);
            }
            Instruction::AddX(arg) => {
                self.tick(2);
                self.register += arg;
            }
        }
    }
}

#[derive(Debug)]
enum Instruction {
    NoOp,
    AddX(i32),
}

impl LineStreamHandler for Day10 {
    fn update(&mut self, line: &str) -> Result<Option<Box<dyn LineStreamHandler>>, Box<dyn Error>> {
        let instruction = parse_full_string(
            line,
            alt((
                map(tag("noop"), |_| Instruction::NoOp),
                map(preceded(tag("addx "), i32), |arg| Instruction::AddX(arg)),
            )),
        )?;
        self.execute(instruction);

        Ok(None)
    }

    fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        println!("[{}] Sum of strengths: {}", SILVER_ANSI, self.sum);
        println!("[{}] Screen contents:", GOLD_ANSI);
        for row in self.screen {
            for bit in 0..40 {
                print!("{}", if 0 != row & (1 << bit) { '#' } else { '.' });
            }
            println!();
        }

        Ok(())
    }
}

pub fn new(
    verbose: bool,
) -> Result<(u8, &'static str, Box<dyn LineStreamHandler>), Box<dyn Error>> {
    Ok((10, "Cathode-Ray Tube", Box::new(Day10::new(verbose))))
}
