use std::{collections::HashSet, error::Error, mem::take};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
    point::{Direction2, Point2},
    terrain::Terrain,
};

#[derive(Debug, Clone, Copy)]
enum Tile {
    Garden(usize, usize),
    Rock,
}

#[derive(Debug)]
struct Day21 {
    gold: bool,
    steps: usize,
    terrain: Terrain<Tile>,
    starting_position: Option<Point2<usize>>,
}

impl Day21 {
    fn new(gold: bool, steps: usize) -> Self {
        Self {
            gold,
            steps,
            terrain: Terrain::new(),
            starting_position: None,
        }
    }
    fn calculate_distances(&mut self) -> Result<(), Box<dyn Error>> {
        let starting_position = match self.starting_position {
            Some(pos) => pos,
            None => {
                return Err("no starting position".into());
            }
        };
        let terrain_size = self.terrain.size();
        let mut next = HashSet::new();
        next.insert(starting_position);
        for distance in 0usize.. {
            if next.is_empty() {
                break;
            }
            for pt in take(&mut next) {
                match &mut self.terrain[pt] {
                    Tile::Garden(tile_distance, _) => {
                        *tile_distance = distance;
                    }
                    _ => unimplemented!(),
                }
                for dir in Direction2::all() {
                    if let Some(next_pt) = pt.try_next_towards(dir) {
                        if next_pt < terrain_size
                            && matches!(self.terrain[next_pt], Tile::Garden(usize::MAX, _))
                        {
                            next.insert(next_pt);
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

impl LineStreamHandler for Day21 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut row = self.terrain.new_row();
        for (ch, i) in line.chars().zip(0usize..) {
            if ch == 'S' {
                self.starting_position = Some(Point2(i, self.terrain.len()));
            }
            row.push(match ch {
                'S' => Tile::Garden(0, 0),
                '.' => Tile::Garden(usize::MAX, 0),
                '#' => Tile::Rock,
                _ => unimplemented!(),
            });
        }
        self.terrain.push_row(row);
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        self.calculate_distances()?;
        let mut reachable = 0usize;
        for pt in self.terrain.points() {
            if let Tile::Garden(distance_from_start, _) = self.terrain[pt] {
                if distance_from_start <= self.steps
                    && (distance_from_start % 2) == (self.steps % 2)
                {
                    reachable += 1;
                }
            }
        }
        println!("{:?}", self.terrain);
        println!(
            "[{}] Reachable plots in {} steps: {}",
            if self.gold { GOLD_ANSI } else { SILVER_ANSI },
            self.steps,
            reachable
        );
        Ok(())
    }
}

pub fn new(gold: bool, steps: usize) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(21, "Step Counter", Day21::new(gold, steps)))
}
