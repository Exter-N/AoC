use std::error::Error;

use super::{LineStreamHandler, GOLD_ANSI, SILVER_ANSI};

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
    fn update(&mut self, line: &str) -> Result<Option<Box<dyn LineStreamHandler>>, Box<dyn Error>> {
        let mut row = self.map.new_row();
        for (ch, j) in line.chars().zip(0usize..) {
            let cell = TerrainCell::try_from(ch)?;
            self.map.update(&cell, j);
            row.push(cell);
        }
        self.map.push_row(row);

        Ok(None)
    }

    fn finish(&mut self) -> Result<(), Box<dyn Error>> {
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

pub fn new(
    gold: bool,
    verbose: bool,
) -> Result<(u8, &'static str, Box<dyn LineStreamHandler>), Box<dyn Error>> {
    Ok((
        12,
        "Hill Climbing Algorithm",
        Box::new(Day12::new(gold, verbose)),
    ))
}
