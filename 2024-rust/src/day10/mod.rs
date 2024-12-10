use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
    point::Point2,
    terrain::Terrain,
};

#[derive(Debug)]
struct Day10 {
    height_map: Terrain<u8>,
    trailheads: HashSet<Point2<usize>>,
}

impl Day10 {
    fn new() -> Self {
        Self {
            height_map: Terrain::new(),
            trailheads: HashSet::new(),
        }
    }
}

impl LineStreamHandler for Day10 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut row = self.height_map.new_row();
        for (ch, i) in line.chars().zip(0usize..) {
            let height = if let Some(digit) = ch.to_digit(10) {
                digit as u8
            } else {
                u8::MAX
            };
            row.push(height);
            if height == 0 {
                self.trailheads.insert(Point2(i, self.height_map.height()));
            }
        }
        self.height_map.push_row(row);
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        let mut positions = HashMap::new();
        for trailhead in self.trailheads {
            positions
                .entry(trailhead)
                .or_insert_with(|| HashMap::new())
                .insert(trailhead, 1usize);
        }
        for height in 1..=9 {
            if positions.is_empty() {
                break;
            }
            let mut new_positions = HashMap::new();
            for (position, trailheads) in positions {
                for (_, neighbor) in self.height_map.neighbors(position) {
                    if self.height_map[neighbor] == height {
                        let new_trailheads = new_positions
                            .entry(neighbor)
                            .or_insert_with(|| HashMap::new());
                        for (trailhead, paths) in &trailheads {
                            *new_trailheads.entry(*trailhead).or_insert(0usize) += paths;
                        }
                    }
                }
            }
            positions = new_positions;
        }
        let mut score = 0usize;
        let mut rating = 0usize;
        for trailheads in positions.values() {
            score += trailheads.len();
            for paths in trailheads.values() {
                rating += *paths;
            }
        }
        println!("[{}] Sum of trailhead scores:  {}", SILVER_ANSI, score);
        println!("[{}] Sum of trailhead ratings: {}", GOLD_ANSI, rating);
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(10, "Hoof It", Day10::new()))
}
