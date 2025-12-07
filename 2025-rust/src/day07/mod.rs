use std::{error::Error, usize};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
};

#[derive(Debug)]
struct Day7 {
    beams: Vec<usize>,
    splits: usize,
}

impl Day7 {
    fn new() -> Self {
        Self {
            beams: Vec::new(),
            splits: 0,
        }
    }
}

impl LineStreamHandler for Day7 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if self.beams.len() < line.len() {
            self.beams.resize(line.len(), 0);
        }
        for (ch, i) in line.chars().zip(0usize..) {
            match ch {
                'S' => {
                    self.beams[i] = 1;
                }
                '^' => {
                    let paths = self.beams[i];
                    if paths > 0 {
                        self.splits += 1;
                        self.beams[i] = 0;
                        self.beams[i - 1] += paths;
                        self.beams[i + 1] += paths;
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!("[{}] Splits: {}", SILVER_ANSI, self.splits);
        println!(
            "[{}] Paths:  {}",
            GOLD_ANSI,
            self.beams.into_iter().sum::<usize>()
        );
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(7, "Laboratories", Day7::new()))
}
