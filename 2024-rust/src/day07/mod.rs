use std::error::Error;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};
use nom::{
    bytes::complete::tag,
    character::complete::{char, u64},
    multi::separated_list1,
    sequence::separated_pair,
};

struct Day7 {
    sum: u64,
    allow_concat: bool,
}

impl Day7 {
    fn new(allow_concat: bool) -> Self {
        Self {
            sum: 0,
            allow_concat,
        }
    }
}

fn concat(left: u64, right: u64) -> u64 {
    left * 10u64.pow(right.ilog10() + 1) + right
}

fn can_get_result(result: u64, accumulator: u64, rest: &[u64], allow_concat: bool) -> bool {
    if rest.is_empty() {
        accumulator == result
    } else if accumulator > result {
        false
    } else {
        can_get_result(result, accumulator + rest[0], &rest[1..], allow_concat)
            || can_get_result(result, accumulator * rest[0], &rest[1..], allow_concat)
            || allow_concat
                && can_get_result(
                    result,
                    concat(accumulator, rest[0]),
                    &rest[1..],
                    allow_concat,
                )
    }
}

impl LineStreamHandler for Day7 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let (result, operands) = parse_full_string(
            line,
            separated_pair(u64, tag(": "), separated_list1(char(' '), u64)),
        )?;
        if can_get_result(result, operands[0], &operands[1..], self.allow_concat) {
            self.sum += result;
        }
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Total calibration result: {}",
            if self.allow_concat {
                GOLD_ANSI
            } else {
                SILVER_ANSI
            },
            self.sum
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(7, "Bridge Repair", Day7::new(gold)))
}
