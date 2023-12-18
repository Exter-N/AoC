use std::{error::Error, mem::take};

use aoc_common_rs::{
    day::{Day, SILVER_ANSI},
    line_stream::LineStreamHandler,
    point::{Direction2, Directional2, Point2},
    terrain::Terrain,
};

const STRAIGHT_CAPACITY: usize = 10 + 1 - 4;

#[derive(Debug)]
struct Tile {
    cost: u8,
    best_path_cost: Directional2<[usize; STRAIGHT_CAPACITY]>,
}

impl Tile {
    fn new(cost: u8) -> Self {
        Tile {
            cost,
            best_path_cost: [usize::MAX; STRAIGHT_CAPACITY].into(),
        }
    }
    fn best_path_cost(&self) -> (Direction2, usize) {
        let mut best_dir = Direction2::Right;
        let mut best = usize::MAX;
        for dir in Direction2::all() {
            let cost = self.best_path_cost[dir][STRAIGHT_CAPACITY - 1];
            if cost < best {
                best_dir = dir;
                best = cost;
            }
        }
        (best_dir, best)
    }
}

struct Day17 {
    min_straight: usize,
    max_straight: usize,
    terrain: Terrain<Tile>,
}

impl Day17 {
    fn new(gold: bool) -> Self {
        Self {
            min_straight: if gold { 4 } else { 1 },
            max_straight: if gold { 10 } else { 3 },
            terrain: Terrain::new(),
        }
    }
    fn cost(&self, from: Point2<usize>, towards: Direction2, distance: usize) -> usize {
        let mut cost = 0;
        for i in 0..distance {
            cost += self.terrain[from.towards(towards, i)].cost as usize;
        }
        cost
    }
}

impl LineStreamHandler for Day17 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut row = self.terrain.new_row();
        for ch in line.chars() {
            if let Some(digit) = ch.to_digit(10) {
                row.push(Tile::new(digit as u8));
            }
        }
        self.terrain.push_row(row);
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        let size = self.terrain.size();
        let mut next: Vec<(Point2<usize>, Direction2, usize, usize)> = Vec::new();
        next.push((
            Point2(self.min_straight, 0),
            Direction2::Right,
            self.cost(Point2(1, 0), Direction2::Right, self.min_straight - 1),
            self.min_straight,
        ));
        next.push((
            Point2(0, self.min_straight),
            Direction2::Down,
            self.cost(Point2(0, 1), Direction2::Down, self.min_straight - 1),
            self.min_straight,
        ));
        while next.len() > 0 {
            for (pt, dir, path_cost, straight) in take(&mut next) {
                let tile = &mut self.terrain[pt];
                if path_cost >= tile.best_path_cost[dir][straight - self.min_straight] {
                    continue;
                }
                for i in (straight - self.min_straight)..STRAIGHT_CAPACITY {
                    tile.best_path_cost[dir][i] = tile.best_path_cost[dir][i].min(path_cost);
                }
                if straight < self.max_straight {
                    if let Some(next_pt) = pt.try_next_towards(dir) {
                        if next_pt < size {
                            next.push((next_pt, dir, path_cost + tile.cost as usize, straight + 1));
                        }
                    }
                }
                let cw_dir = dir.clockwise();
                if let Some(next_pt) = pt.try_towards(cw_dir, self.min_straight) {
                    if next_pt < size {
                        next.push((
                            next_pt,
                            cw_dir,
                            path_cost + self.cost(pt, cw_dir, self.min_straight),
                            self.min_straight,
                        ));
                    }
                }
                let ccw_dir = dir.counterclockwise();
                if let Some(next_pt) = pt.try_towards(ccw_dir, self.min_straight) {
                    if next_pt < size {
                        next.push((
                            next_pt,
                            ccw_dir,
                            path_cost + self.cost(pt, ccw_dir, self.min_straight),
                            self.min_straight,
                        ));
                    }
                }
            }
        }
        let end_corner = &self.terrain[self.terrain.size() - Point2(1, 1)];
        println!(
            "[{}] Best path cost: {}",
            SILVER_ANSI,
            end_corner.best_path_cost().1 + end_corner.cost as usize
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(17, "Clumsy Crucible", Day17::new(gold)))
}
