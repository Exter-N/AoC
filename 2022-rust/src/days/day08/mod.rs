use std::cmp::max;
use std::error::Error;

use super::LineStreamHandler;

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
    fn update(&mut self, line: &str) -> Result<Option<Box<dyn LineStreamHandler>>, Box<dyn Error>> {
        let mut row = self.map.new_row();
        for ch in line.chars() {
            row.push(ch.try_into()?);
        }
        self.map.push_row(row);

        Ok(None)
    }

    fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        self.map.calculate_ns_visibilities();
        self.map.calculate_scenic_scores();
        println!("[S] Visible tree count: {}", self.map.visible_count);
        println!(
            "[G] Max scenic score:   {}",
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

pub fn new(
    verbosity: u8,
) -> Result<(u8, &'static str, Box<dyn LineStreamHandler>), Box<dyn Error>> {
    Ok((8, "Treetop Tree House", Box::new(Day8::new(verbosity))))
}
