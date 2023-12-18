use std::cmp::max;
use std::error::Error;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
};

mod map;

use map::Map;

#[derive(Default)]
struct Day8 {
    verbosity: u8,
    map: Map,
}

impl Day8 {
    fn new(verbosity: u8) -> Self {
        Self {
            verbosity,
            ..Default::default()
        }
    }
}

impl LineStreamHandler for Day8 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut row = self.map.new_row();
        for ch in line.chars() {
            row.push(ch.try_into()?);
        }
        self.map.push_row(row);

        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        self.map.calculate_ns_visibilities();
        self.map.calculate_scenic_scores();
        println!(
            "[{}] Visible tree count: {}",
            SILVER_ANSI, self.map.visible_count
        );
        println!(
            "[{}] Max scenic score:   {}",
            GOLD_ANSI,
            max(self.map.max_visible_score, self.map.max_hidden_score)
        );
        if self.verbosity > 0 {
            println!("[-] Max visible score:  {}", self.map.max_visible_score);
            println!("[-] Max hidden score:   {}", self.map.max_hidden_score);
        }
        if self.verbosity > 2 {
            self.map.dump_extended()
        } else if self.verbosity > 1 {
            self.map.dump();
        }

        Ok(())
    }
}

pub fn new(verbosity: u8) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(8, "Treetop Tree House", Day8::new(verbosity)))
}
