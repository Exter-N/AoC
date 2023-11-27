use std::error::Error;

use nom::character::complete::{char, u32};
use nom::sequence::separated_pair;

use super::{parse_full_string, LineStreamHandler, GOLD_ANSI, SILVER_ANSI};

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
    fn update(&mut self, line: &str) -> Result<Option<Box<dyn LineStreamHandler>>, Box<dyn Error>> {
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

        Ok(None)
    }

    fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        println!("[{}] Contain: {}", SILVER_ANSI, self.contain);
        println!("[{}] Overlap: {}", GOLD_ANSI, self.overlap);

        Ok(())
    }
}

pub fn new() -> Result<(u8, &'static str, Box<dyn LineStreamHandler>), Box<dyn Error>> {
    Ok((4, "Camp Cleanup", Box::new(Day4::new())))
}
