use std::error::Error;

use super::LineStreamHandler;

#[derive(Default)]
struct Day19 {}

impl Day19 {
    fn new() -> Self {
        Default::default()
    }
}

impl LineStreamHandler for Day19 {
    fn update(
        &mut self,
        _line: &str,
    ) -> Result<Option<Box<dyn LineStreamHandler>>, Box<dyn Error>> {
        Ok(None)
    }

    fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub fn new() -> Result<(u8, &'static str, Box<dyn LineStreamHandler>), Box<dyn Error>> {
    Ok((19, "Not Enough Minerals", Box::new(Day19::new())))
}
