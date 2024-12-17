use std::{error::Error, mem::take};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, u64, u8},
    combinator::map,
    multi::separated_list1,
    sequence::preceded,
};

mod vm;

enum Instruction {
    A(u64),
    B(u64),
    C(u64),
    Program(Vec<u8>),
}

struct Day17 {
    a: Option<u64>,
    b: Option<u64>,
    c: Option<u64>,
    program: Option<Vec<u8>>,
    gold: bool,
}

impl Day17 {
    fn new(gold: bool) -> Self {
        Self {
            a: None,
            b: None,
            c: None,
            program: None,
            gold,
        }
    }
}

fn search_for_quine_a(
    b: u64,
    c: u64,
    program: &Vec<u8>,
    pos: usize,
    a_guess: u64,
) -> Result<Option<u64>, Box<dyn Error>> {
    for digit in 0u64..8 {
        let a = a_guess | (digit << (pos * 3));
        let mut vm = vm::VirtualMachine::new(a, b, c, program);
        vm.try_run_to_completion()?;
        let output = vm.into_output();
        if let Some(output_pos) = (pos + output.len()).checked_sub(program.len()) {
            if output[output_pos] == program[pos] {
                if pos == 0 {
                    return Ok(Some(a));
                }
                if let Some(a_final) = search_for_quine_a(b, c, program, pos - 1, a)? {
                    return Ok(Some(a_final));
                }
            }
        }
    }
    Ok(None)
}

impl LineStreamHandler for Day17 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if line.is_empty() {
            return Ok(());
        }
        match parse_full_string(
            line,
            alt((
                map(preceded(tag("Register A: "), u64), |value| {
                    Instruction::A(value)
                }),
                map(preceded(tag("Register B: "), u64), |value| {
                    Instruction::B(value)
                }),
                map(preceded(tag("Register C: "), u64), |value| {
                    Instruction::C(value)
                }),
                map(
                    preceded(tag("Program: "), separated_list1(char(','), u8)),
                    |value| Instruction::Program(value),
                ),
            )),
        )? {
            Instruction::A(value) => {
                self.a = Some(value);
            }
            Instruction::B(value) => {
                self.b = Some(value);
            }
            Instruction::C(value) => {
                self.c = Some(value);
            }
            Instruction::Program(value) => {
                self.program = Some(value);
            }
        }

        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        let program = take(&mut self.program).unwrap();
        if self.gold {
            // Corners have been cut, this may not be correct in all cases.
            let b = self.b.unwrap();
            let c = self.c.unwrap();
            let a = search_for_quine_a(b, c, &program, program.len() - 1, 0)?;
            println!("[{}] Register A: {}", GOLD_ANSI, a.unwrap());
        } else {
            let mut vm = vm::VirtualMachine::new(
                self.a.unwrap(),
                self.b.unwrap(),
                self.c.unwrap(),
                &program,
            );
            vm.try_run_to_completion()?;
            let output = vm.into_output();
            print!("[{}] Output: ", SILVER_ANSI);
            for (out, i) in output.into_iter().zip(0usize..) {
                if i > 0 {
                    print!(",{}", out);
                } else {
                    print!("{}", out);
                }
            }
            println!();
        }
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(17, "Chronospatial Computer", Day17::new(gold)))
}
