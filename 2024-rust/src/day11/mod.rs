use std::{collections::HashMap, error::Error, mem::replace};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};
use nom::{
    character::complete::{multispace1, u64},
    multi::separated_list1,
};

#[derive(Debug)]
struct Day11 {
    frequencies: HashMap<u64, usize>,
}

impl Day11 {
    fn new() -> Self {
        Self {
            frequencies: HashMap::new(),
        }
    }

    fn step(&mut self) {
        for (stone, freq) in replace(&mut self.frequencies, HashMap::new()) {
            let (new_stone1, maybe_new_stone2) = step(stone);
            *self.frequencies.entry(new_stone1).or_insert(0) += freq;
            if let Some(new_stone2) = maybe_new_stone2 {
                *self.frequencies.entry(new_stone2).or_insert(0) += freq;
            }
        }
    }

    fn count(&self) -> usize {
        let mut result = 0usize;
        for freq in self.frequencies.values() {
            result += *freq;
        }
        result
    }
}

fn step(stone: u64) -> (u64, Option<u64>) {
    if stone == 0 {
        (1, None)
    } else if (stone.ilog10() & 1) == 1 {
        let mid = 10u64.pow((stone.ilog10() + 1) >> 1);

        (stone / mid, Some(stone % mid))
    } else {
        (stone * 2024, None)
    }
}

impl LineStreamHandler for Day11 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let stones = parse_full_string(line, separated_list1(multispace1, u64))?;
        for stone in stones {
            *self.frequencies.entry(stone).or_insert(0) += 1;
        }
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        for _ in 0usize..25 {
            self.step();
        }
        println!(
            "[{}] Total stone count after 25 blinks: {}",
            SILVER_ANSI,
            self.count()
        );
        for _ in 0usize..50 {
            self.step();
        }
        println!(
            "[{}] Total stone count after 75 blinks: {}",
            GOLD_ANSI,
            self.count()
        );
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(11, "Plutonian Pebbles", Day11::new()))
}
