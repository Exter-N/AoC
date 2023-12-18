use std::{error::Error, mem::replace};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
};

mod schematic;

use schematic::Line;

struct Day3 {
    sum_of_parts: u32,
    sum_of_ratios: u32,
    previous_line: Line,
}

impl Day3 {
    fn new() -> Self {
        Self {
            sum_of_parts: 0,
            sum_of_ratios: 0,
            previous_line: Default::default(),
        }
    }
}

impl LineStreamHandler for Day3 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut parsed_line = Line::parse(line);
        parsed_line.update_gears(&self.previous_line);
        self.previous_line.update_gears(&parsed_line);
        self.sum_of_parts +=
            parsed_line.inner_sum_of_parts() + parsed_line.outer_sum_of_parts(&self.previous_line);
        let previous_line = replace(&mut self.previous_line, parsed_line);
        self.sum_of_ratios += previous_line.into_gear_ratios();
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        self.sum_of_ratios += self.previous_line.into_gear_ratios();
        println!(
            "[{}] Sum of part numbers: {}",
            SILVER_ANSI, self.sum_of_parts
        );
        println!(
            "[{}] Sum of gear ratios:  {}",
            GOLD_ANSI, self.sum_of_ratios
        );
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(3, "Gear Ratios", Day3::new()))
}
