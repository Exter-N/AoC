use std::{collections::HashSet, error::Error};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
    point::{Direction2, Point2},
    terrain::Terrain,
};

#[derive(Debug)]
struct Day6 {
    terrain: Terrain<bool>,
    start: Option<Point2<usize>>,
}

impl Day6 {
    fn new() -> Self {
        Self {
            terrain: Terrain::new(),
            start: None,
        }
    }
    fn walk(&self) -> (HashSet<Point2<usize>>, bool) {
        let mut visited = HashSet::new();
        let mut oriented_visited = HashSet::new();
        let mut position = self.start.unwrap();
        visited.insert(position);
        let mut direction = Direction2::Up;
        loop {
            if !oriented_visited.insert((position, direction)) {
                return (visited, true);
            }
            let next_position = position.try_next_towards(direction);
            if next_position.is_none() {
                return (visited, false);
            }
            let next_position = next_position.unwrap();
            if next_position.0 >= self.terrain.width() || next_position.1 >= self.terrain.height() {
                return (visited, false);
            }
            if self.terrain[next_position] {
                direction = direction.clockwise();
            } else {
                visited.insert(next_position);
                position = next_position;
            }
        }
    }
    fn count_possible_obstructions(&mut self) -> usize {
        let mut possible_obstructions = 0;
        let (points, already_looping) = self.walk();
        if already_looping {
            panic!("the guard is already looping");
        }
        for point in points {
            self.terrain[point] = true;
            if self.walk().1 {
                possible_obstructions += 1;
            }
            self.terrain[point] = false;
        }
        possible_obstructions
    }
}

impl LineStreamHandler for Day6 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut row = self.terrain.new_row();
        for (ch, i) in line.chars().zip(0usize..) {
            row.push(ch == '#');
            if ch == '^' {
                self.start = Some(Point2(i, self.terrain.len()));
            }
        }
        self.terrain.push_row(row);
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Visited positions:     {}",
            SILVER_ANSI,
            self.walk().0.len()
        );
        println!(
            "[{}] Possible obstructions: {}",
            GOLD_ANSI,
            self.count_possible_obstructions()
        );
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(6, "Guard Gallivant", Day6::new()))
}
