use std::error::Error;

use aoc_common_rs::{day::Day, line_stream::LineStreamHandler};

#[derive(Default)]
struct Day19 {}

impl Day19 {
    fn new() -> Self {
        Default::default()
    }
}

impl LineStreamHandler for Day19 {
    fn update(&mut self, _line: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(19, "Not Enough Minerals", Day19::new()))
}
