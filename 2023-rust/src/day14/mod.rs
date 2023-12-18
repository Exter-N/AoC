use std::{collections::HashMap, error::Error};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
    point::{Direction2, Point2},
    terrain::Terrain,
};
use nom::{branch::alt, character::complete::char, combinator::value, multi::many1};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Rock {
    None,
    Rounded,
    Cube,
}

struct Day14 {
    gold: bool,
    terrain: Terrain<Rock>,
}

impl Day14 {
    fn new(gold: bool) -> Self {
        Self {
            gold,
            terrain: Terrain::new(),
        }
    }
    fn tilt(&mut self, towards: Direction2) {
        match towards {
            Direction2::Up | Direction2::Left => {
                for source in self.terrain.points() {
                    self.tilt_single(source, towards);
                }
            }
            Direction2::Down | Direction2::Right => {
                for source in self.terrain.points_rev() {
                    self.tilt_single(source, towards);
                }
            }
        }
    }
    fn tilt_single(&mut self, source: Point2<usize>, towards: Direction2) {
        if matches!(self.terrain[source], Rock::Rounded) {
            let target = self.terrain.walk_while(source, towards, |target| {
                matches!(self.terrain[target], Rock::None)
            });
            if target != source {
                self.terrain[target] = Rock::Rounded;
                self.terrain[source] = Rock::None;
            }
        }
    }
    fn tilt_cycle(&mut self) {
        self.tilt(Direction2::Up);
        self.tilt(Direction2::Left);
        self.tilt(Direction2::Down);
        self.tilt(Direction2::Right);
    }
    fn total_north_load(&self) -> usize {
        let mut load = 0usize;
        for pt in self.terrain.points() {
            if matches!(self.terrain[pt], Rock::Rounded) {
                load += self.terrain.height() - pt.1;
            }
        }
        load
    }
}

impl LineStreamHandler for Day14 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        self.terrain.push_row(parse_full_string(
            line,
            many1(alt((
                value(Rock::None, char('.')),
                value(Rock::Rounded, char('O')),
                value(Rock::Cube, char('#')),
            ))),
        )?);
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        if self.gold {
            let mut states: HashMap<Terrain<Rock>, usize> = HashMap::new();
            for i in 0usize..1_000_000_000 {
                let first_seen = states.entry(self.terrain.clone()).or_insert(i);
                if *first_seen != i && (1_000_000_000usize - *first_seen) % (i - *first_seen) == 0 {
                    break;
                }
                self.tilt_cycle();
            }
        } else {
            self.tilt(Direction2::Up);
        }
        println!(
            "[{}] Total North Load: {}",
            if self.gold { GOLD_ANSI } else { SILVER_ANSI },
            self.total_north_load()
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(14, "Parabolic Reflector Dish", Day14::new(gold)))
}
