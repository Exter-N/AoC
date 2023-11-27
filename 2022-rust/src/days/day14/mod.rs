use std::collections::HashSet;
use std::error::Error;

use nom::bytes::complete::tag;
use nom::character::complete::{char, u16};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;

use crate::point::{Direction2, Point2};

use super::{parse_full_string, LineStreamHandler, GOLD_ANSI, SILVER_ANSI};

const SAND_START: Point2<u16> = Point2(500, 0);

#[derive(Default)]
struct Day14 {
    with_floor: bool,
    verbose: bool,
    min: Point2<u16>,
    max: Point2<u16>,
    solid: HashSet<Point2<u16>>,
    sand: HashSet<Point2<u16>>,
}

impl Day14 {
    fn new(with_floor: bool, verbose: bool) -> Self {
        Self {
            with_floor,
            verbose,
            min: SAND_START,
            max: SAND_START,
            ..Default::default()
        }
    }
    fn extend(&mut self, pt: Point2<u16>) {
        self.min = self.min.componentwise_min(pt);
        self.max = self.max.componentwise_max(pt);
    }
    fn add_solid_line(&mut self, from: Point2<u16>, to: Point2<u16>) -> Result<(), Box<dyn Error>> {
        self.extend(from);
        self.extend(to);
        if let Some(direction) = from.direction_towards(&to) {
            for i in 0..=from.manhattan_distance(to) {
                self.solid.insert(from.towards(direction, i));
            }

            Ok(())
        } else {
            Err(Box::from("non-axis-aligned line"))
        }
    }
    fn add_floor(&mut self) -> Result<(), Box<dyn Error>> {
        let distance = self.max.1 - SAND_START.1 + 2;
        let center = SAND_START.towards(Direction2::Down, distance);
        self.add_solid_line(
            center.towards(Direction2::Left, distance),
            center.towards(Direction2::Right, distance),
        )?;

        Ok(())
    }
    fn add_all_sand(&mut self) {
        while let Some(pt) = self.next_sand() {
            self.solid.insert(pt);
            self.sand.insert(pt);
        }
    }
    fn next_sand(&self) -> Option<Point2<u16>> {
        let mut sand = SAND_START;
        if self.solid.contains(&sand) {
            return None;
        }

        loop {
            if sand.0 < self.min.0 || sand.0 > self.max.0 || sand.1 >= self.max.1 {
                return None;
            }
            if let Some(next) = self.sand_fall(sand) {
                sand = next;
            } else {
                return Some(sand);
            }
        }
    }
    fn sand_fall(&self, sand: Point2<u16>) -> Option<Point2<u16>> {
        let down = sand.next_towards(Direction2::Down);
        if !self.solid.contains(&down) {
            return Some(down);
        }
        let down_left = down.next_towards(Direction2::Left);
        if !self.solid.contains(&down_left) {
            return Some(down_left);
        }
        let down_right = down.next_towards(Direction2::Right);
        if !self.solid.contains(&down_right) {
            return Some(down_right);
        }

        None
    }
    fn dump(&self) {
        for y in self.min.1..=self.max.1 {
            for x in self.min.0..=self.max.0 {
                let pt = Point2(x, y);
                print!(
                    "{}",
                    if pt == SAND_START {
                        '+'
                    } else if self.sand.contains(&pt) {
                        'o'
                    } else if self.solid.contains(&pt) {
                        '#'
                    } else {
                        '.'
                    }
                )
            }
            println!()
        }
    }
}

impl LineStreamHandler for Day14 {
    fn update(&mut self, line: &str) -> Result<Option<Box<dyn LineStreamHandler>>, Box<dyn Error>> {
        let path = parse_full_string(
            line,
            separated_list1(
                tag(" -> "),
                map(separated_pair(u16, char(','), u16), |(x, y)| Point2(x, y)),
            ),
        )?;
        for i in 1usize..path.len() {
            self.add_solid_line(path[i - 1], path[i])?;
        }

        Ok(None)
    }

    fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        if self.with_floor {
            self.add_floor()?;
        }
        self.add_all_sand();

        println!(
            "[{}] Units of sand: {}",
            if self.with_floor {
                GOLD_ANSI
            } else {
                SILVER_ANSI
            },
            self.sand.len()
        );

        if self.verbose {
            self.dump();
        }

        Ok(())
    }
}

pub fn new(
    gold: bool,
    verbose: bool,
) -> Result<(u8, &'static str, Box<dyn LineStreamHandler>), Box<dyn Error>> {
    Ok((
        14,
        "Regolith Reservoir",
        Box::new(Day14::new(gold, verbose)),
    ))
}
