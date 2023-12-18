use std::error::Error;

use aoc_common_rs::{day::Day, line_stream::LineStreamHandler};

#[derive(Default)]
struct Day23 {}

impl Day23 {
    fn new() -> Self {
        Default::default()
    }
}

impl LineStreamHandler for Day23 {
    fn update(&mut self, _line: &str) -> Result<(), Box<dyn Error>> {
        todo!()
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(23, "Unstable Diffusion", Day23::new()))
}
