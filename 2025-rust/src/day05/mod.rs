use std::error::Error;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{LineStreamHandlerOnce, parse_full_string},
    multi_range::MultiRangeInclusive,
};
use nom::{
    character::complete::{char, u64},
    sequence::separated_pair,
};

struct Day5 {
    fresh_ids: MultiRangeInclusive<u64>,
    fresh_avail: usize,
}

impl Day5 {
    fn new() -> Self {
        Self {
            fresh_ids: MultiRangeInclusive::new(),
            fresh_avail: 0,
        }
    }
}

struct Day5Fresh(Day5);

impl LineStreamHandlerOnce for Day5Fresh {
    fn update(
        mut self: Box<Self>,
        line: &str,
    ) -> Result<Box<dyn LineStreamHandlerOnce>, Box<dyn Error>> {
        if line.is_empty() {
            return Ok(Box::new(Day5Available(self.0)));
        }

        let (from, to) = parse_full_string(line, separated_pair(u64, char('-'), u64))?;
        self.0.fresh_ids.insert(from..=to);
        Ok(self)
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        unreachable!()
    }
}

struct Day5Available(Day5);

impl LineStreamHandlerOnce for Day5Available {
    fn update(
        mut self: Box<Self>,
        line: &str,
    ) -> Result<Box<dyn LineStreamHandlerOnce>, Box<dyn Error>> {
        let id = parse_full_string(line, u64)?;
        if self.0.fresh_ids.contains(&id) {
            self.0.fresh_avail += 1;
        }

        Ok(self)
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Fresh available ingredients: {}",
            SILVER_ANSI, self.0.fresh_avail
        );
        println!(
            "[{}] Total fresh ingredients:     {}",
            GOLD_ANSI,
            self.0.fresh_ids.count()
        );
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new_once(5, "Cafeteria", Day5Fresh(Day5::new())))
}
