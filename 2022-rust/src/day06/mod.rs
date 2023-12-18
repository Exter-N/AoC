use std::error::Error;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
};

const SILVER_MARKER_LENGTH: usize = 4;
const GOLD_MARKER_LENGTH: usize = 14;

struct Day6 {
    marker_length: usize,
}

impl Day6 {
    fn new(marker_length: usize) -> Self {
        Self { marker_length }
    }
}

fn has_all_distinct_chars(s: &str) -> Result<bool, Box<dyn Error>> {
    let mut mask = 0u128;
    for ch in s.chars() {
        let ch_ord = ch as u32;
        if ch_ord > 127 {
            return Err(Box::from("non-ASCII character found"));
        }
        let ch_mask = 1u128 << ch_ord;
        if 0 != (mask & ch_mask) {
            return Ok(false);
        } else {
            mask |= ch_mask;
        }
    }

    Ok(true)
}

impl LineStreamHandler for Day6 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let line_len = line.len();
        let prefix = if self.marker_length == GOLD_MARKER_LENGTH {
            GOLD_ANSI
        } else {
            SILVER_ANSI
        };
        let description = if self.marker_length == GOLD_MARKER_LENGTH {
            "message"
        } else {
            "packet"
        };
        for i in self.marker_length..=line_len {
            if has_all_distinct_chars(&line[(i - self.marker_length)..i])? {
                println!("[{}] Start of {}: {}", prefix, description, i);
                return Ok(());
            }
        }
        println!("[{}] No start of {}", prefix, description);

        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(
        6,
        "Tuning Trouble",
        Day6::new(if gold {
            GOLD_MARKER_LENGTH
        } else {
            SILVER_MARKER_LENGTH
        }),
    ))
}
