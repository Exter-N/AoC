use std::error::Error;

use nom::character::complete::u32;
use nom::combinator::opt;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
    ord::Top,
};

#[derive(Default)]
struct Day1 {
    verbose: bool,
    top: Top<u32, 3>,
    running: u32,
}

impl Day1 {
    fn new(verbose: bool) -> Self {
        Self {
            verbose,
            ..Default::default()
        }
    }
    fn add(&mut self, calories: u32) {
        self.running += calories;
    }
    fn end_group(&mut self) {
        self.top.insert(self.running);
        self.running = 0;
    }
}

impl LineStreamHandler for Day1 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if let Some(calories) = parse_full_string(line, opt(u32))? {
            self.add(calories);
        } else {
            self.end_group();
        }

        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        self.end_group();

        let [top3, top2, top1] = *self.top;
        println!("[{}] Top Elf:      {}", SILVER_ANSI, top1);
        println!("[{}] Sum of top 3: {}", GOLD_ANSI, top1 + top2 + top3);
        if self.verbose {
            println!("[-] Top 3 Elves:  {} + {} + {}", top1, top2, top3);
        }

        Ok(())
    }
}

pub fn new(verbose: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(1, "Calorie Counting", Day1::new(verbose)))
}
