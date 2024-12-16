use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult, Write},
};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::LineStreamHandlerOnce,
    point::{Direction2, Point2},
    terrain::Terrain,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Cell {
    Free,
    Wall,
    Box,
    BoxLeft,
    BoxRight,
    Robot,
}

impl Display for Cell {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_char(match self {
            Cell::Free => '.',
            Cell::Wall => '#',
            Cell::Box => 'O',
            Cell::BoxLeft => '[',
            Cell::BoxRight => ']',
            Cell::Robot => '@',
        })
    }
}

#[derive(Debug)]
struct Day15 {
    terrain: Terrain<Cell>,
    robot_position: Option<Point2<usize>>,
    upscale: bool,
    verbose: bool,
}

impl Day15 {
    fn new(upscale: bool, verbose: bool) -> Self {
        Self {
            terrain: Terrain::new(),
            robot_position: None,
            upscale,
            verbose,
        }
    }

    fn try_push(&mut self, pt: Point2<usize>, towards: Direction2) -> bool {
        if !self.can_push(pt, towards, false) {
            return false;
        }

        self.push(pt, towards, false);

        return true;
    }

    fn can_push(&self, pt: Point2<usize>, towards: Direction2, second_half: bool) -> bool {
        let cell = self.terrain[pt];
        match cell {
            Cell::Free => {
                return true;
            }
            Cell::Wall => {
                return false;
            }
            _ => {}
        }

        let destination = match self.terrain.neighbor(pt, towards) {
            Some(pt) => pt,
            None => {
                return false;
            }
        };

        if !self.can_push(destination, towards, false) {
            return false;
        }

        if !second_half && matches!(towards, Direction2::Up | Direction2::Down) {
            match cell {
                Cell::BoxLeft => {
                    if !self.can_push(pt.next_right(), towards, true) {
                        return false;
                    }
                }
                Cell::BoxRight => {
                    if !self.can_push(pt.next_left(), towards, true) {
                        return false;
                    }
                }
                _ => {}
            }
        }

        return true;
    }

    fn push(&mut self, pt: Point2<usize>, towards: Direction2, second_half: bool) {
        let cell = self.terrain[pt];
        if matches!(cell, Cell::Free | Cell::Wall) {
            return;
        }

        let destination = self.terrain.neighbor(pt, towards).unwrap();

        self.push(destination, towards, false);

        if !second_half && matches!(towards, Direction2::Up | Direction2::Down) {
            match cell {
                Cell::BoxLeft => {
                    self.push(pt.next_right(), towards, true);
                }
                Cell::BoxRight => {
                    self.push(pt.next_left(), towards, true);
                }
                _ => {}
            }
        }

        if !matches!(self.terrain[destination], Cell::Free) {
            panic!();
        }
        self.terrain[destination] = cell;
        self.terrain[pt] = Cell::Free;
    }

    fn box_coord_sum(&self) -> usize {
        let mut sum = 0;
        for pt in self.terrain.points() {
            if matches!(self.terrain[pt], Cell::Box | Cell::BoxLeft) {
                sum += pt.1 * 100 + pt.0;
            }
        }
        sum
    }
}

struct Day15Terrain(Day15);

impl LineStreamHandlerOnce for Day15Terrain {
    fn update(
        mut self: Box<Self>,
        line: &str,
    ) -> Result<Box<dyn LineStreamHandlerOnce>, Box<dyn Error>> {
        if line.is_empty() {
            return Ok(Box::new(Day15Moves(self.0)));
        }

        let mut row = self.0.terrain.new_row();
        for ch in line.chars() {
            let cell = match ch {
                '#' => Cell::Wall,
                '.' => Cell::Free,
                'O' => Cell::Box,
                '@' => {
                    self.0.robot_position = Some(Point2(row.len(), self.0.terrain.height()));
                    Cell::Robot
                }
                _ => panic!(),
            };
            if self.0.upscale {
                row.push(match cell {
                    Cell::Box => Cell::BoxLeft,
                    _ => cell,
                });
                row.push(match cell {
                    Cell::Box => Cell::BoxRight,
                    Cell::Robot => Cell::Free,
                    _ => cell,
                });
            } else {
                row.push(cell);
            }
        }
        self.0.terrain.push_row(row);

        Ok(self)
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        unreachable!()
    }
}

struct Day15Moves(Day15);

impl LineStreamHandlerOnce for Day15Moves {
    fn update(
        mut self: Box<Self>,
        line: &str,
    ) -> Result<Box<dyn LineStreamHandlerOnce>, Box<dyn Error>> {
        for ch in line.chars() {
            let dir = match ch {
                '<' => Direction2::Left,
                '^' => Direction2::Up,
                '>' => Direction2::Right,
                'v' => Direction2::Down,
                _ => panic!(),
            };

            let robot = self.0.robot_position.unwrap();
            if self.0.try_push(robot, dir) {
                self.0.robot_position = Some(robot.next_towards(dir));
            }
        }

        Ok(self)
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        if self.0.verbose {
            println!("{}", self.0.terrain);
        }

        println!(
            "[{}] Sum of boxes' GPS coordinates: {}",
            if self.0.upscale {
                GOLD_ANSI
            } else {
                SILVER_ANSI
            },
            self.0.box_coord_sum()
        );

        Ok(())
    }
}

pub fn new(gold: bool, verbose: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new_once(
        15,
        "Warehouse Woes",
        Day15Terrain(Day15::new(gold, verbose)),
    ))
}
