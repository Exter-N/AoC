use std::collections::HashSet;
use std::error::Error;

use crate::point::{Direction2, Point2};

use super::{LineStreamHandler, GOLD_ANSI, SILVER_ANSI};

#[derive(Debug)]
struct Blizzard {
    direction: Direction2,
    position: Point2<usize>,
}

impl Blizzard {
    fn new(direction: Direction2, position: Point2<usize>) -> Self {
        Self {
            direction,
            position,
        }
    }
}

#[derive(Debug, Default)]
struct Day24 {
    end: Point2<usize>,
    height: usize,
    blizzards: Vec<Blizzard>,
}

impl Day24 {
    fn new() -> Self {
        Default::default()
    }
    fn update_blizzards(&mut self) {
        for blizzard in self.blizzards.iter_mut() {
            blizzard.position = blizzard.position.next_towards(blizzard.direction);
            if blizzard.position.0 == 0 {
                blizzard.position.0 = self.end.0;
            } else if blizzard.position.0 == self.end.0 + 1 {
                blizzard.position.0 = 1;
            }
            if blizzard.position.1 == 0 {
                blizzard.position.1 = self.end.1 - 1;
            } else if blizzard.position.1 == self.end.1 {
                blizzard.position.1 = 1;
            }
        }
    }
    fn has_blizzard_at(&self, pos: Point2<usize>) -> bool {
        self.blizzards.iter().any(|bliz| bliz.position == pos)
    }
    fn neighbors<'a>(
        self: &'a Self,
        pt: Point2<usize>,
    ) -> impl Iterator<Item = Point2<usize>> + 'a {
        Direction2::all().filter_map(move |dir| {
            if match dir {
                Direction2::Right => pt.0 < self.end.0 && pt.1 > 0,
                Direction2::Down => {
                    pt.1 < self.end.1 && (pt.1 + 1 < self.end.1 || pt.0 == self.end.0)
                }
                Direction2::Left => pt.0 > 1 && pt.1 < self.end.1,
                Direction2::Up => pt.1 > 0 && (pt.1 > 1 || pt.0 == 1),
            } {
                Some(pt.next_towards(dir))
            } else {
                None
            }
        })
    }
    fn time_from_to(&mut self, from: Point2<usize>, to: Point2<usize>) -> Option<usize> {
        let mut next_positions: HashSet<Point2<usize>> = HashSet::new();
        next_positions.insert(from);
        for time in 0usize.. {
            if next_positions.contains(&to) {
                return Some(time);
            }
            let positions = next_positions.drain().collect::<Vec<_>>();
            if positions.is_empty() {
                return None;
            }
            self.update_blizzards();
            for pos in positions {
                if !self.has_blizzard_at(pos) {
                    next_positions.insert(pos);
                }
                for pos_n in self.neighbors(pos) {
                    if !self.has_blizzard_at(pos_n) {
                        next_positions.insert(pos_n);
                    }
                }
            }
        }

        None
    }
    fn time_forwards(&mut self) -> Option<usize> {
        self.time_from_to(Point2(1, 0), self.end)
    }
    fn time_backwards(&mut self) -> Option<usize> {
        self.time_from_to(self.end, Point2(1, 0))
    }
}

impl LineStreamHandler for Day24 {
    fn update(&mut self, line: &str) -> Result<Option<Box<dyn LineStreamHandler>>, Box<dyn Error>> {
        let mut is_end_line = false;
        for (ch, i) in line.chars().zip(0usize..) {
            let pos = Point2(i, self.height);
            match ch {
                '>' => {
                    self.blizzards.push(Blizzard::new(Direction2::Right, pos));
                }
                'v' => {
                    self.blizzards.push(Blizzard::new(Direction2::Down, pos));
                }
                '<' => {
                    self.blizzards.push(Blizzard::new(Direction2::Left, pos));
                }
                '^' => {
                    self.blizzards.push(Blizzard::new(Direction2::Up, pos));
                }
                '#' => {
                    if i == 1 {
                        is_end_line = true;
                    }
                }
                '.' => {
                    if is_end_line {
                        self.end = pos;
                    }
                }
                _ => {}
            }
        }
        self.height = self.height + 1;

        Ok(None)
    }

    fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(time) = self.time_forwards() {
            println!("[{}] Time to traverse: {}", SILVER_ANSI, time);
            if let Some(time2) = self.time_backwards() {
                if let Some(time3) = self.time_forwards() {
                    println!("[{}] Total time:       {}", GOLD_ANSI, time + time2 + time3);
                } else {
                    return Err(Box::from("cannot traverse again"));
                }
            } else {
                return Err(Box::from("cannot traverse back"));
            }
        } else {
            return Err(Box::from("cannot traverse"));
        }

        Ok(())
    }
}

pub fn new() -> Result<(u8, &'static str, Box<dyn LineStreamHandler>), Box<dyn Error>> {
    Ok((24, "Blizzard Basin", Box::new(Day24::new())))
}
