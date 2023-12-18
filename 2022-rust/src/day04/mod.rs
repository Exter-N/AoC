use std::error::Error;

use nom::character::complete::{char, u32};
use nom::sequence::separated_pair;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};

#[derive(Default)]
struct Day4 {
    contain: u32,
    overlap: u32,
}

impl Day4 {
    fn new() -> Self {
        Default::default()
    }
}

impl LineStreamHandler for Day4 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let ((start1, end1), (start2, end2)) = parse_full_string(
            line,
            separated_pair(
                separated_pair(u32, char('-'), u32),
                char(','),
                separated_pair(u32, char('-'), u32),
            ),
        )?;
        // these two conditions are equivalent:
        // if start1 >= start2 && end1 <= end2 || start2 >= start1 && end2 <= end1 {
        if (start1.cmp(&start2) as i32) * (end1.cmp(&end2) as i32) <= 0 {
            self.contain += 1;
        }
        if start1 <= end2 && start2 <= end1 {
            self.overlap += 1;
        }

        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!("[{}] Contain: {}", SILVER_ANSI, self.contain);
        println!("[{}] Overlap: {}", GOLD_ANSI, self.overlap);

        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(4, "Camp Cleanup", Day4::new()))
}
