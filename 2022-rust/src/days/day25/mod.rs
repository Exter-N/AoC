use std::error::Error;

use super::{LineStreamHandler, SILVER_ANSI};

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
    fn update(&mut self, line: &str) -> Result<Option<Box<dyn LineStreamHandler>>, Box<dyn Error>> {
        let snafu = Snafu::try_from(line)?;
        self.sum += snafu.0;

        Ok(None)
    }

    fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Sum of fuel requirements: {}",
            SILVER_ANSI,
            Snafu(self.sum)
        );

        Ok(())
    }
}

pub fn new() -> Result<(u8, &'static str, Box<dyn LineStreamHandler>), Box<dyn Error>> {
    Ok((25, "Full of Hot Air", Box::new(Day25::new())))
}
