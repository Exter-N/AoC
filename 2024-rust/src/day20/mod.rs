use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult, Write},
};

use aoc_common_rs::{
    day::{Day, SILVER_ANSI},
    line_stream::LineStreamHandler,
    point::{Direction2, Point2},
    terrain::Terrain,
};

#[derive(Debug)]
struct Track {
    wall: bool,
    cost: usize,
}

impl Display for Track {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_char(if self.wall { '#' } else { '.' })
    }
}

impl Track {
    fn new(wall: bool) -> Self {
        Self {
            wall,
            cost: usize::MAX,
        }
    }
}

struct CheatReport {
    interesting: usize,
    best_benefit: usize,
    best_from: Point2<usize>,
    best_to: Point2<usize>,
}

impl CheatReport {
    fn new_empty(pt: Point2<usize>) -> Self {
        Self {
            interesting: 0,
            best_benefit: 0,
            best_from: pt,
            best_to: pt,
        }
    }

    fn merge(self, other: Self) -> Self {
        Self {
            interesting: self.interesting + other.interesting,
            ..if other.best_benefit > self.best_benefit {
                other
            } else {
                self
            }
        }
    }
}

#[derive(Debug)]
struct Day20 {
    terrain: Terrain<Track>,
    start: Option<Point2<usize>>,
    end: Option<Point2<usize>>,
    max_cheat_distance: usize,
}

impl Day20 {
    fn new(gold: bool) -> Self {
        Self {
            terrain: Terrain::new(),
            start: None,
            end: None,
            max_cheat_distance: if gold { 20 } else { 2 },
        }
    }

    fn calculate_costs(&mut self) {
        self.terrain.flood_fill_mut(
            self.start.unwrap(),
            |terrain, _, _, _, pt| !terrain[pt].wall,
            |terrain, level, pt| terrain[pt].cost = level,
        );
    }

    fn next(&self, pt: Point2<usize>) -> Option<(Direction2, Point2<usize>)> {
        if self.terrain[pt].wall {
            return None;
        }
        let cost = self.terrain[pt].cost;
        self.terrain
            .neighbors(pt)
            .find(|(_, next)| !self.terrain[*next].wall && self.terrain[*next].cost == cost + 1)
    }

    fn find_cheats_from(&self, pt: Point2<usize>, max_distance: usize) -> CheatReport {
        let mut report = CheatReport::new_empty(pt);
        if self.terrain[pt].wall {
            return report;
        }
        let cost = self.terrain[pt].cost;
        self.terrain.flood_fill(
            pt,
            |_, level, _, _, _| level < max_distance,
            |terrain, level, pt2| {
                if terrain[pt2].wall {
                    return;
                }
                let delta = match terrain[pt2].cost.checked_sub(cost) {
                    Some(delta) => delta,
                    None => {
                        return;
                    }
                };
                let benefit = delta - level;
                if benefit >= 100 {
                    report.interesting += 1;
                }
                if benefit > report.best_benefit {
                    report.best_benefit = benefit;
                    report.best_from = pt;
                    report.best_to = pt2;
                }
            },
        );
        report
    }
}

impl LineStreamHandler for Day20 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let mut row = self.terrain.new_row();
        for (ch, i) in line.chars().zip(0usize..) {
            row.push(Track::new(match ch {
                '#' => true,
                '.' => false,
                'S' => {
                    self.start = Some(Point2(i, self.terrain.height()));
                    false
                }
                'E' => {
                    self.end = Some(Point2(i, self.terrain.height()));
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
        let mut report = CheatReport::new_empty(self.start.unwrap());
        let mut maybe_pt = self.start;
        while let Some(pt) = maybe_pt {
            report = report.merge(self.find_cheats_from(pt, self.max_cheat_distance));
            maybe_pt = self.next(pt).map(|(_, pt)| pt);
        }
        println!(
            "[{}] Interesting cheats: {}",
            SILVER_ANSI, report.interesting
        );
        println!(
            "[-] Best cheat:         {} -> {}, saves {} ps",
            report.best_from, report.best_to, report.best_benefit,
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(20, "Race Condition", Day20::new(gold)))
}
