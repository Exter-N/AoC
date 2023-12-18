use std::error::Error;

use nom::{
    character::complete::{i32, multispace1},
    multi::separated_list1,
};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};

fn extrapolate(values: Vec<i32>) -> Option<i32> {
    if values.len() < 2 {
        return None;
    } else if values.len() == 2 {
        return Some(values[1] * 2 - values[0]);
    }
    let mut diff: Vec<i32> = Vec::with_capacity(values.len() - 1);
    let mut const_diff = true;
    for (before, after) in values.iter().zip(values.iter().skip(1)) {
        diff.push(after - before);
        if after - before != diff[0] {
            const_diff = false;
        }
    }
    if const_diff {
        Some(diff[0])
    } else {
        extrapolate(diff)
    }
    .map(|d| values.last().unwrap() + d)
}

struct Day9 {
    gold: bool,
    sum: i32,
}

impl Day9 {
    fn new(gold: bool) -> Self {
        Self { gold, sum: 0 }
    }
}

impl LineStreamHandler for Day9 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut sequence = parse_full_string(line, separated_list1(multispace1, i32))?;
        if self.gold {
            sequence.reverse();
        }
        if let Some(extrapolated) = extrapolate(sequence) {
            self.sum += extrapolated;
            Ok(())
        } else {
            Err("could not extrapolate".into())
        }
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Sum of extrapolated values: {}",
            if self.gold { GOLD_ANSI } else { SILVER_ANSI },
            self.sum
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(9, "Mirage Maintenance", Day9::new(gold)))
}
