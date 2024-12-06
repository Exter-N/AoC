use std::error::Error;

use aoc_common_rs::{
    bit_matrix::BitMatrix128,
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandlerOnce},
};
use nom::{
    character::complete::{char, u32, u8},
    multi::separated_list1,
    sequence::separated_pair,
};

struct Day5 {
    orderings: BitMatrix128,
    sum_of_middles: u32,
    sum_of_reordered_middles: u32,
}

impl Day5 {
    fn new() -> Self {
        Self {
            orderings: BitMatrix128::identity(),
            sum_of_middles: 0,
            sum_of_reordered_middles: 0,
        }
    }
    fn process_update(&mut self, mut update: Vec<u32>) {
        let mut reordered = false;
        loop {
            let mut reordered_this_turn = false;
            for i in 1..update.len() {
                if self
                    .orderings
                    .get(update[i] as usize, update[i - 1] as usize)
                {
                    reordered_this_turn = true;
                    update.swap(i, i - 1);
                }
            }
            if reordered_this_turn {
                reordered = true;
            } else {
                break;
            }
        }
        let middle = update[update.len() >> 1];
        if reordered {
            self.sum_of_reordered_middles += middle;
        } else {
            self.sum_of_middles += middle;
        }
    }
}

struct Day5Orderings(Day5);

impl LineStreamHandlerOnce for Day5Orderings {
    fn update(
        mut self: Box<Self>,
        line: &str,
    ) -> Result<Box<dyn LineStreamHandlerOnce>, Box<dyn Error>> {
        if line.is_empty() {
            return Ok(Box::new(Day5Updates(self.0)));
        }

        let (before, after) = parse_full_string(line, separated_pair(u8, char('|'), u8))?;
        self.0.orderings.set(before as usize, after as usize, true);
        Ok(self)
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        unreachable!()
    }
}

struct Day5Updates(Day5);

impl LineStreamHandlerOnce for Day5Updates {
    fn update(
        mut self: Box<Self>,
        line: &str,
    ) -> Result<Box<dyn LineStreamHandlerOnce>, Box<dyn Error>> {
        let pages = parse_full_string(line, separated_list1(char(','), u32))?;
        self.0.process_update(pages);
        Ok(self)
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Sum of middle pages:           {}",
            SILVER_ANSI, self.0.sum_of_middles
        );
        println!(
            "[{}] Sum of reordered middle pages: {}",
            GOLD_ANSI, self.0.sum_of_reordered_middles
        );
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new_once(5, "Print Queue", Day5Orderings(Day5::new())))
}
