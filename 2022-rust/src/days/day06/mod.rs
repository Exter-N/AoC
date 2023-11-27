use std::error::Error;

use crate::days::{GOLD_ANSI, SILVER_ANSI};

use super::LineStreamHandler;

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
    fn update(&mut self, line: &str) -> Result<Option<Box<dyn LineStreamHandler>>, Box<dyn Error>> {
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
                return Ok(None);
            }
        }
        println!("[{}] No start of {}", prefix, description);

        Ok(None)
    }

    fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<(u8, &'static str, Box<dyn LineStreamHandler>), Box<dyn Error>> {
    Ok((
        6,
        "Tuning Trouble",
        Box::new(Day6::new(if gold {
            GOLD_MARKER_LENGTH
        } else {
            SILVER_MARKER_LENGTH
        })),
    ))
}
