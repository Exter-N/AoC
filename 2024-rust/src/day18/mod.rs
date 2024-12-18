use std::error::Error;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
    point::Point2,
    terrain::Terrain,
};
use nom::{
    character::complete::{char, u64},
    combinator::map,
    sequence::separated_pair,
};

#[derive(Debug)]
struct Cell {
    min_cost: usize,
    corrupted_after: usize,
}

impl Cell {
    fn new() -> Self {
        Self {
            min_cost: usize::MAX,
            corrupted_after: usize::MAX,
        }
    }
}

struct Day18 {
    terrain: Terrain<Cell>,
    corruptions: usize,
    gold: bool,
}

impl Day18 {
    fn new(size: usize, gold: bool) -> Self {
        Self {
            terrain: Terrain::new_with(size, size, |_| Cell::new()),
            corruptions: 0,
            gold,
        }
    }
}

impl LineStreamHandler for Day18 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let pt = parse_full_string(
            line,
            map(separated_pair(u64, char(','), u64), |(x, y)| {
                Point2(x as usize, y as usize)
            }),
        )?;
        self.terrain[pt].corrupted_after = self.corruptions;
        self.corruptions += 1;

        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        let exit = Point2(self.terrain.width() - 1, self.terrain.height() - 1);
        if self.gold {
            for corruptions in (0..self.corruptions).rev() {
                self.terrain.flood_fill_mut(
                    Point2(0, 0),
                    |terrain, _, _, _, pt| terrain[pt].corrupted_after >= corruptions,
                    |terrain, level, pt| terrain[pt].min_cost = level,
                );

                if self.terrain[exit].min_cost != usize::MAX {
                    println!("[-] Maximum time to reach the exit: {}", corruptions);
                    println!(
                        "[-] Minimum number of steps:        {}",
                        self.terrain[exit].min_cost
                    );
                    for pt in self.terrain.points() {
                        if self.terrain[pt].corrupted_after == corruptions {
                            println!(
                                "[{}] Coordinates of byte:            {},{}",
                                GOLD_ANSI, pt.0, pt.1
                            );
                        }
                    }
                    break;
                }
            }
        } else {
            self.terrain.flood_fill_mut(
                Point2(0, 0),
                |terrain, _, _, _, pt| {
                    terrain[pt].corrupted_after >= if terrain.width() > 20 { 1024 } else { 12 }
                },
                |terrain, level, pt| terrain[pt].min_cost = level,
            );

            println!(
                "[{}] Minimum number of steps: {}",
                SILVER_ANSI, self.terrain[exit].min_cost
            );
        }

        Ok(())
    }
}

pub fn new(sample: bool, gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(
        18,
        "RAM Run",
        Day18::new(if sample { 7 } else { 71 }, gold),
    ))
}
