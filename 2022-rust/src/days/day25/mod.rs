use std::error::Error;

use aoc_common_rs::{
    day::{Day, SILVER_ANSI},
    line_stream::LineStreamHandler,
};

mod snafu;

use snafu::Snafu;

#[derive(Default)]
struct Day25 {
    sum: u64,
}

impl Day25 {
    fn new() -> Self {
        Default::default()
    }
}

impl LineStreamHandler for Day25 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let snafu = Snafu::try_from(line)?;
        self.sum += snafu.0;

        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Sum of fuel requirements: {}",
            SILVER_ANSI,
            Snafu(self.sum)
        );

        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(25, "Full of Hot Air", Day25::new()))
}
