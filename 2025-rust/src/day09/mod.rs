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

#[derive(Clone, Debug)]
struct RunVec<T>(Vec<(usize, T)>);

impl<T> RunVec<T> {
    fn new(end: usize, item: T) -> Self {
        Self(vec![(end, item)])
    }

    fn find_run_index(&self, position: usize) -> usize {
        match self.0.binary_search_by(|(until, _)| until.cmp(&position)) {
            Ok(index) => index + 1,
            Err(index) => index,
        }
    }
}

impl<T> RunVec<T>
where
    T: Clone,
{
    fn split_run_before(&mut self, index: &mut usize, position: usize) {
        let last_end = if *index > 0 { self.0[*index - 1].0 } else { 0 };
        if last_end != position {
            self.0.insert(*index, (position, self.0[*index].1.clone()));
            *index += 1;
        }
    }

    fn split_run_after(&mut self, index: usize, position: usize) {
        if self.0[index].0 != position + 1 {
            self.0
                .insert(index, (position + 1, self.0[index].1.clone()));
        }
    }

    fn get_mut(&mut self, position: usize) -> &mut T {
        let mut index = self.find_run_index(position);
        self.split_run_before(&mut index, position);
        self.split_run_after(index, position);
        &mut self.0[index].1
    }
}

fn insert_vertical_bounds(
    vertical_bounds: &mut RunVec<(MultiRangeInclusive<usize>, BTreeMap<usize, bool>)>,
    x: usize,
    y_start: usize,
    y_end: usize,
    down: bool,
) {
    let mut index_start = vertical_bounds.find_run_index(y_start);
    vertical_bounds.split_run_before(&mut index_start, y_start);
    let index_end = vertical_bounds.find_run_index(y_end);
    vertical_bounds.split_run_after(index_end, y_end);
    for index in index_start..=index_end {
        if vertical_bounds.0[index].1.1.insert(x, down).is_some() {
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

    fn interior(&self) -> RunVec<MultiRangeInclusive<usize>> {
        if self.tiles.len() < 4 {
            unimplemented!();
        }

        let height = self.height();
        let mut interior_and_vbounds: RunVec<(MultiRangeInclusive<usize>, BTreeMap<usize, bool>)> =
            RunVec::new(height, (MultiRangeInclusive::new(), BTreeMap::new()));
        let mut previous_point = self.tiles.last().unwrap();
        for point in self.tiles.iter() {
            match previous_point.direction_towards(point) {
                Some(direction) => match direction {
                    Direction2::Right => {
                        interior_and_vbounds
                            .get_mut(point.1)
                            .0
                            .insert(previous_point.0..=point.0);
                    }
                    Direction2::Down => {
                        insert_vertical_bounds(
                            &mut interior_and_vbounds,
                            point.0,
                            previous_point.1,
                            point.1,
                            true,
                        );
                    }
                    Direction2::Left => {
                        interior_and_vbounds
                            .get_mut(point.1)
                            .0
                            .insert(point.0..=previous_point.0);
                    }
                    Direction2::Up => {
                        insert_vertical_bounds(
                            &mut interior_and_vbounds,
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
        for (_, (interior_row, vbound_row)) in interior_and_vbounds.0.iter_mut() {
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
        let mut interior = RunVec(
            interior_and_vbounds
                .0
                .into_iter()
                .map(|(end, (interior_row, _))| (end, interior_row))
                .collect_vec(),
        );
        for i in (1..interior.0.len()).rev() {
            if interior.0[i].1.iter().eq(interior.0[i - 1].1.iter()) {
                interior.0.remove(i - 1);
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
                        let y_start = interior.find_run_index(tile1.1);
                        let y_end = interior.find_run_index(tile2.1);
                        for y in y_start..=y_end {
                            if !interior.0[y].1.contains_all(&x_range) {
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
