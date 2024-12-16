use std::{
    collections::{HashMap, HashSet},
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult, Write},
    mem::replace,
};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandler,
    point::{Direction2, Point2},
    terrain::Terrain,
};

const MOVE_COST: usize = 1;
const TURN_COST: usize = 1000;

#[derive(Debug)]
struct Cell {
    wall: bool,
    optimal: std::cell::Cell<bool>,
    min_cost_left: Option<usize>,
    min_cost_up: Option<usize>,
    min_cost_right: Option<usize>,
    min_cost_down: Option<usize>,
}

impl Cell {
    fn new(wall: bool) -> Self {
        Self {
            wall,
            optimal: std::cell::Cell::new(false),
            min_cost_left: None,
            min_cost_up: None,
            min_cost_right: None,
            min_cost_down: None,
        }
    }

    fn min_cost_facing(&self, facing: Direction2) -> &Option<usize> {
        match facing {
            Direction2::Right => &self.min_cost_right,
            Direction2::Down => &self.min_cost_down,
            Direction2::Left => &self.min_cost_left,
            Direction2::Up => &self.min_cost_up,
        }
    }

    fn min_cost_facing_mut(&mut self, facing: Direction2) -> &mut Option<usize> {
        match facing {
            Direction2::Right => &mut self.min_cost_right,
            Direction2::Down => &mut self.min_cost_down,
            Direction2::Left => &mut self.min_cost_left,
            Direction2::Up => &mut self.min_cost_up,
        }
    }

    fn min_cost(&self) -> Option<(usize, Direction2)> {
        let mut best = None;
        for dir in Direction2::all() {
            best = match (best, self.min_cost_facing(dir)) {
                (None, None) => None,
                (None, Some(cost)) => Some((*cost, dir)),
                (Some(previous), None) => Some(previous),
                (Some(previous), Some(cost)) => Some(if *cost < previous.0 {
                    (*cost, dir)
                } else {
                    previous
                }),
            };
        }
        best
    }
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let ch = if self.wall {
            '#'
        } else {
            match self.min_cost() {
                Some((_, facing)) => match facing {
                    Direction2::Right => '>',
                    Direction2::Down => 'v',
                    Direction2::Left => '<',
                    Direction2::Up => '^',
                },
                None => '.',
            }
        };
        if self.optimal.get() {
            write!(f, "\x1B[38;2;0;204;0m{}\x1B[m", ch)
        } else {
            f.write_char(ch)
        }
    }
}

struct Day16 {
    terrain: Terrain<Cell>,
    start: Option<Point2<usize>>,
    end: Option<Point2<usize>>,
    verbose: bool,
}

fn update_next(
    next: &mut HashMap<(Point2<usize>, Direction2), usize>,
    pt: Point2<usize>,
    facing: Direction2,
    cost: usize,
) {
    next.entry((pt.next_towards(facing), facing))
        .and_modify(|best| *best = (*best).min(cost))
        .or_insert(cost);
}

impl Day16 {
    fn new(verbose: bool) -> Self {
        Self {
            terrain: Terrain::new(),
            start: None,
            end: None,
            verbose,
        }
    }

    fn calculate_costs(&mut self) {
        let mut next = HashMap::new();
        next.insert((self.start.unwrap(), Direction2::Right), 0usize);
        while !next.is_empty() {
            for ((pt, facing), cost) in replace(&mut next, HashMap::new()) {
                if self.terrain[pt].wall {
                    continue;
                }

                let min_cost = self.terrain[pt].min_cost_facing_mut(facing);
                let better = match min_cost {
                    Some(min) => cost < *min,
                    None => true,
                };

                if !better {
                    continue;
                }

                *min_cost = Some(cost);

                update_next(&mut next, pt, facing, cost + MOVE_COST);
                update_next(
                    &mut next,
                    pt,
                    facing.clockwise(),
                    cost + MOVE_COST + TURN_COST,
                );
                update_next(
                    &mut next,
                    pt,
                    facing.counterclockwise(),
                    cost + MOVE_COST + TURN_COST,
                );
            }
        }
    }

    fn end_cost(&self) -> (usize, Direction2) {
        self.terrain[self.end.unwrap()].min_cost().unwrap()
    }

    fn collect_optimal(
        &self,
        pt: Point2<usize>,
        backtracked_cost: (usize, Direction2),
        dest: &mut HashSet<Point2<usize>>,
    ) {
        if !dest.insert(pt) {
            return;
        }
        let cell = &self.terrain[pt];
        cell.optimal.set(true);
        if cell.min_cost().unwrap().0 < MOVE_COST {
            return;
        }
        for dir in Direction2::all() {
            let min = match *cell.min_cost_facing(dir) {
                Some(cost) => cost,
                None => {
                    continue;
                }
            };
            let turn_cost = if dir == backtracked_cost.1 {
                0
            } else {
                TURN_COST
            };
            let min_aligned = min + turn_cost;
            if min_aligned == backtracked_cost.0 {
                self.collect_optimal(
                    pt.next_towards(-dir),
                    (backtracked_cost.0 - turn_cost - MOVE_COST, dir),
                    dest,
                );
            }
        }
    }

    fn count_optimal_cells(&mut self) -> usize {
        let mut optimal_cells = HashSet::new();
        self.collect_optimal(self.end.unwrap(), self.end_cost(), &mut optimal_cells);
        optimal_cells.len()
    }
}

impl LineStreamHandler for Day16 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut row = self.terrain.new_row();
        for (ch, i) in line.chars().zip(0usize..) {
            row.push(Cell::new(match ch {
                '#' => true,
                '.' => false,
                'E' => {
                    self.end = Some(Point2(i, self.terrain.height()));
                    false
                }
                'S' => {
                    self.start = Some(Point2(i, self.terrain.height()));
                    false
                }
                _ => panic!(),
            }));
        }
        self.terrain.push_row(row);
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        self.calculate_costs();
        let optimal_cells = self.count_optimal_cells();
        if self.verbose {
            println!("{}", self.terrain);
        }
        println!(
            "[{}] Optimal score:          {}",
            SILVER_ANSI,
            self.end_cost().0
        );
        println!("[{}] Cells on optimal paths: {}", GOLD_ANSI, optimal_cells);
        Ok(())
    }
}

pub fn new(verbose: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(16, "Reindeer Maze", Day16::new(verbose)))
}
