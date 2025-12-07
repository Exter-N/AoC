use std::{collections::HashMap, error::Error, usize};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
};

#[derive(Debug)]
struct Day7 {
    beams: HashMap<usize, usize>,
    splits: usize,
}

impl Day7 {
    fn new() -> Self {
        Self {
            beams: HashMap::new(),
            splits: 0,
        }
    }
}

impl LineStreamHandler for Day7 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        for (ch, i) in line.chars().zip(0usize..) {
            match ch {
                'S' => {
                    self.beams.insert(i, 1);
                }
                '^' => {
                    if let Some(paths) = self.beams.remove(&i) {
                        self.splits += 1;
                        self.beams
                            .entry(i - 1)
                            .and_modify(|existing| *existing += paths)
                            .or_insert(paths);
                        self.beams
                            .entry(i + 1)
                            .and_modify(|existing| *existing += paths)
                            .or_insert(paths);
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
            self.beams.into_values().sum::<usize>()
        );
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(7, "Laboratories", Day7::new()))
}
