use std::error::Error;

use aoc_common_rs::day::{GOLD_ANSI, SILVER_ANSI};
use aoc_common_rs::line_stream::parse_full_string;
use aoc_common_rs::{day::Day, line_stream::LineStreamHandler};
use nom::character::complete::u32;
use nom::character::one_of;
use nom::combinator::map;
use nom::error::Error as NomError;
use nom::sequence::pair;

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
}

struct Day1 {
    position: u32,
    zeros: u32,
    count_all_zeros: bool,
}

impl Day1 {
    fn new(gold: bool) -> Day1 {
        Day1 {
            position: 50,
            zeros: 0,
            count_all_zeros: gold,
        }
    }

    fn rotate(&mut self, direction: Direction, mut amount: u32) {
        if amount == 0 {
            return;
        }

        if self.count_all_zeros {
            self.zeros += amount / 100;
        }
        amount %= 100;

        let mut new_position = match direction {
            Direction::Left => self.position + 100 - amount,
            Direction::Right => self.position + amount,
        };
        if new_position >= 100 {
            new_position -= 100;
        }

        if self.count_all_zeros {
            match direction {
                Direction::Left => {
                    if new_position > self.position && self.position != 0 || new_position == 0 {
                        self.zeros += 1;
                    }
                }
                Direction::Right => {
                    if new_position < self.position {
                        self.zeros += 1;
                    }
                }
            }
        } else {
            if new_position == 0 {
                self.zeros += 1;
            }
        }
        self.position = new_position;
    }
}

fn parse_line(line: &str) -> Result<(Direction, u32), NomError<usize>> {
    parse_full_string(
        line,
        pair(
            map(one_of("LR"), |ch| match ch {
                'L' => Direction::Left,
                'R' => Direction::Right,
                _ => unreachable!(),
            }),
            u32,
        ),
    )
}

impl LineStreamHandler for Day1 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let (direction, amount) = parse_line(line)?;
        self.rotate(direction, amount);
        println!(
            "{:?}{} {:>3} -> {:>2} ({:>5})",
            direction,
            if matches!(direction, Direction::Left) {
                " "
            } else {
                ""
            },
            amount,
            self.position,
            self.zeros
        );
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!("[-] Final position: {}", self.position);
        println!(
            "[{}] Password:       {}",
            if self.count_all_zeros {
                GOLD_ANSI
            } else {
                SILVER_ANSI
            },
            self.zeros
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(1, "Secret Entrance", Day1::new(gold)))
}
