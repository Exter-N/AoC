use std::{collections::BTreeMap, error::Error};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{LineStreamHandler, parse_full_string},
    multi_range::MultiRangeInclusive,
    point::{Direction2, Point2},
};
use itertools::Itertools;
use nom::{
    character::complete::{char, usize},
    sequence::separated_pair,
};

fn insert_vertical_bounds(
    vertical_bounds: &mut Vec<BTreeMap<usize, bool>>,
    x: usize,
    y_start: usize,
    y_end: usize,
    down: bool,
) {
    for y in y_start..=y_end {
        if vertical_bounds[y].insert(x, down).is_some() {
            unreachable!();
        }
    }
}

struct Day9 {
    tiles: Vec<Point2<usize>>,
    gold: bool,
}

impl Day9 {
    fn new(gold: bool) -> Self {
        Self {
            tiles: Vec::new(),
            gold,
        }
    }

    fn height(&self) -> usize {
        self.tiles.iter().map(|tile| tile.1 + 1).max().unwrap_or(0)
    }

    fn interior(&self) -> Vec<MultiRangeInclusive<usize>> {
        if self.tiles.len() < 4 {
            unreachable!();
        }

        let mut interior: Vec<MultiRangeInclusive<usize>> =
            vec![MultiRangeInclusive::new(); self.height()];
        let mut vertical_bounds: Vec<BTreeMap<usize, bool>> = vec![BTreeMap::new(); interior.len()];
        let mut previous_point = self.tiles.last().unwrap();
        for point in self.tiles.iter() {
            match previous_point.direction_towards(point) {
                Some(direction) => match direction {
                    Direction2::Right => {
                        interior[point.1].insert(previous_point.0..=point.0);
                    }
                    Direction2::Down => {
                        insert_vertical_bounds(
                            &mut vertical_bounds,
                            point.0,
                            previous_point.1,
                            point.1,
                            true,
                        );
                    }
                    Direction2::Left => {
                        interior[point.1].insert(point.0..=previous_point.0);
                    }
                    Direction2::Up => {
                        insert_vertical_bounds(
                            &mut vertical_bounds,
                            point.0,
                            point.1,
                            previous_point.1,
                            false,
                        );
                    }
                },
                None => unimplemented!(),
            }
            previous_point = point;
        }
        for (interior_row, vbound_row) in interior.iter_mut().zip(vertical_bounds.iter()) {
            if vbound_row.is_empty() {
                continue;
            }
            let mut previous_entry = vbound_row.first_key_value().unwrap();
            let ccw_winding = *previous_entry.1;
            for entry in vbound_row.iter().skip(1) {
                if *previous_entry.1 == ccw_winding && *entry.1 != ccw_winding {
                    interior_row.insert(*previous_entry.0..=*entry.0);
                }
                previous_entry = entry;
            }
        }
        interior
    }
}

impl LineStreamHandler for Day9 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let (x, y) = parse_full_string(line, separated_pair(usize, char(','), usize))?;
        self.tiles.push(Point2(x, y));
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        if self.gold {
            let interior = self.interior();
            println!(
                "[{}] Maximum area: {}",
                GOLD_ANSI,
                self.tiles
                    .iter()
                    .cartesian_product(self.tiles.iter())
                    .filter_map(move |(tile1, tile2)| {
                        if tile1.1 > tile2.1 {
                            return None;
                        }
                        let x_range = if tile1.0 > tile2.0 {
                            tile2.0..=tile1.0
                        } else {
                            tile1.0..=tile2.0
                        };
                        for y in tile1.1..=tile2.1 {
                            if !interior[y].contains_all(&x_range) {
                                return None;
                            }
                        }
                        Some((tile1.0.abs_diff(tile2.0) + 1) * (tile1.1.abs_diff(tile2.1) + 1))
                    })
                    .max()
                    .unwrap_or(0)
            );
        } else {
            println!(
                "[{}] Maximum area: {}",
                SILVER_ANSI,
                self.tiles
                    .iter()
                    .cartesian_product(self.tiles.iter())
                    .filter_map(|(tile1, tile2)| if tile1.1 > tile2.1 {
                        None
                    } else {
                        Some((tile1.0.abs_diff(tile2.0) + 1) * (tile1.1.abs_diff(tile2.1) + 1))
                    })
                    .max()
                    .unwrap_or(0)
            );
        }

        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(9, "Movie Theater", Day9::new(gold)))
}
