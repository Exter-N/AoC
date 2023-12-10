use std::error::Error;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
};

mod map;

use map::{Map, TerrainCell};

#[derive(Default)]
struct Day12 {
    from_any: bool,
    verbose: bool,
    map: Map,
}

impl Day12 {
    fn new(from_any: bool, verbose: bool) -> Self {
        Self {
            from_any,
            verbose,
            ..Default::default()
        }
    }
}

impl LineStreamHandler for Day12 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut row = self.map.new_row();
        for (ch, j) in line.chars().zip(0usize..) {
            let cell = TerrainCell::try_from(ch)?;
            self.map.update(&cell, j);
            row.push(cell);
        }
        self.map.push_row(row);

        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        self.map.calculate_distances(self.from_any);
        if let Some(distance) = self.map.climbing_distance() {
            println!(
                "[{}] Climbing distance: {}",
                if self.from_any {
                    GOLD_ANSI
                } else {
                    SILVER_ANSI
                },
                distance
            );
        } else {
            println!(
                "[{}] Cannot calculate climbing distance",
                if self.from_any {
                    GOLD_ANSI
                } else {
                    SILVER_ANSI
                }
            );
        }
        if self.verbose {
            self.map.calculate_path();
            self.map.dump();
        }

        Ok(())
    }
}

pub fn new(gold: bool, verbose: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(
        12,
        "Hill Climbing Algorithm",
        Day12::new(gold, verbose),
    ))
}
