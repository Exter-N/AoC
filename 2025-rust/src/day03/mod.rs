use std::error::Error;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
    some_or_continue,
};

fn insert(buffer: &mut Vec<u64>, digit: u64) {
    let len = buffer.len();
    for i in 1..len {
        if buffer[i] > buffer[i - 1] {
            buffer.copy_within(i..len, i - 1);
            *buffer.last_mut().unwrap() = digit;
            return;
        }
    }

    let last = buffer.last_mut().unwrap();
    if digit > *last {
        *last = digit;
    }
}

struct Day3 {
    total: u64,
    batteries_per_bank: usize,
}

impl Day3 {
    fn new(gold: bool) -> Day3 {
        Day3 {
            total: 0,
            batteries_per_bank: if gold { 12 } else { 2 },
        }
    }
}

impl LineStreamHandler for Day3 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut buffer = Vec::with_capacity(self.batteries_per_bank);
        buffer.resize(self.batteries_per_bank, 0u64);
        for ch in line.chars() {
            let digit = some_or_continue!(ch.to_digit(10)) as u64;
            insert(&mut buffer, digit);
        }
        self.total += buffer.into_iter().fold(0, |acc, digit| acc * 10 + digit);

        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Total output joltage: {}",
            if self.batteries_per_bank > 10 {
                GOLD_ANSI
            } else {
                SILVER_ANSI
            },
            self.total
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(3, "Lobby", Day3::new(gold)))
}
