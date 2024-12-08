use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
    point::Point2,
};

#[derive(Debug)]
struct Day8 {
    width: isize,
    height: isize,
    antennas: HashMap<char, HashSet<Point2<isize>>>,
    with_harmonics: bool,
}

impl Day8 {
    fn new(with_harmonics: bool) -> Self {
        Self {
            width: 0,
            height: 0,
            antennas: HashMap::new(),
            with_harmonics,
        }
    }

    fn is_in_bounds(&self, point: Point2<isize>) -> bool {
        point.0 >= 0 && point.0 < self.width && point.1 >= 0 && point.1 < self.height
    }

    fn count_antinodes(&self) -> usize {
        let mut antinodes = HashSet::new();
        for antennas in self.antennas.values() {
            if antennas.len() <= 1 {
                continue;
            }
            for antenna1 in antennas {
                for antenna2 in antennas {
                    if antenna1 == antenna2 {
                        continue;
                    }
                    let antinode = antenna1.point_reflect(*antenna2);
                    if self.is_in_bounds(antinode) {
                        antinodes.insert(antinode);
                    }
                }
            }
        }
        antinodes.len()
    }

    fn count_antinodes_with_harmonics(&self) -> usize {
        let mut antinodes = HashSet::new();
        for antennas in self.antennas.values() {
            if antennas.len() <= 1 {
                continue;
            }
            for antenna1 in antennas {
                antinodes.insert(*antenna1);
                for antenna2 in antennas {
                    if antenna1 == antenna2 {
                        continue;
                    }
                    for multiplier in 1isize.. {
                        let antinode = *antenna2 * (multiplier + 1) - *antenna1 * multiplier;
                        if !self.is_in_bounds(antinode) {
                            break;
                        }
                        antinodes.insert(antinode);
                    }
                }
            }
        }
        antinodes.len()
    }
}

impl LineStreamHandler for Day8 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        for (ch, i) in line.chars().zip(0isize..) {
            if ch != '.' {
                self.antennas
                    .entry(ch)
                    .or_insert_with(|| HashSet::new())
                    .insert(Point2(i, self.height));
            }
        }
        self.width = self.width.max(line.len() as isize);
        self.height += 1;
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        if self.with_harmonics {
            println!(
                "[{}] Unique antinode locations: {}",
                GOLD_ANSI,
                self.count_antinodes_with_harmonics()
            );
        } else {
            println!(
                "[{}] Unique antinode locations: {}",
                SILVER_ANSI,
                self.count_antinodes()
            );
        }
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(8, "Resonant Collinearity", Day8::new(gold)))
}
