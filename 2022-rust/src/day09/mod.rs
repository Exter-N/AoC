use std::error::Error;

use nom::character::complete::{anychar, char, u16};
use nom::sequence::separated_pair;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
    point::{Direction2, Point2},
};

mod rope;

use rope::{pull_towards, TracedPoint};

#[derive(Default)]
struct Day9 {
    verbosity: u8,
    head: Point2<i16>,
    intermediate: Vec<Point2<i16>>,
    tail: TracedPoint<i16>,
    min: Point2<i16>,
    max: Point2<i16>,
}

impl Day9 {
    fn new(intermediate_knots: usize, verbosity: u8) -> Self {
        Self {
            verbosity,
            intermediate: vec![Default::default(); intermediate_knots],
            ..Default::default()
        }
    }
    fn move_head(&mut self, direction: Direction2, distance: u16) {
        if self.verbosity > 2 {
            println!("[-] Moving head by {} towards {:?}", distance, direction);
        }
        for i in 0..distance {
            let prev_head = self.head;
            self.head = self.head.next_towards(direction);
            self.min = self.min.componentwise_min(self.head);
            self.max = self.max.componentwise_max(self.head);
            if self.intermediate.is_empty() {
                self.tail.pull_towards(self.head, prev_head);
            } else {
                let (mut intermediate, mut prev_intermediate) =
                    pull_towards(&mut self.intermediate[0], self.head, prev_head);
                for j in 1..self.intermediate.len() {
                    (intermediate, prev_intermediate) =
                        pull_towards(&mut self.intermediate[j], intermediate, prev_intermediate);
                }
                self.tail.pull_towards(intermediate, prev_intermediate);
            }
            if self.verbosity > 3 {
                if i > 0 {
                    println!()
                }
                self.dump_current()
            }
        }
        if self.verbosity == 3 {
            self.dump_current()
        }
    }
    fn dump_current(&self) {
        for y in self.min.1..=self.max.1 {
            for x in self.min.0..=self.max.0 {
                let pt = Point2(x, y);
                print!(
                    "{}",
                    if pt == self.head {
                        'H'
                    } else if let Some(index) = self.intermediate.iter().position(|p| *p == pt) {
                        (index as u8 + '1' as u8) as char
                    } else if pt == self.tail.current {
                        'T'
                    } else if pt == Default::default() {
                        's'
                    } else if self.tail.trace.contains(&pt) {
                        '#'
                    } else {
                        '.'
                    }
                );
            }
            println!()
        }
    }
    fn dump_trace(&self) {
        for y in self.min.1..=self.max.1 {
            for x in self.min.0..=self.max.0 {
                let pt = Point2(x, y);
                print!(
                    "{}",
                    if pt == Default::default() {
                        's'
                    } else if self.tail.trace.contains(&pt) {
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

impl LineStreamHandler for Day9 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let (direction, distance) =
            parse_full_string(line, separated_pair(anychar, char(' '), u16))?;
        self.move_head(Direction2::try_from(direction)?, distance);

        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Points in tail trace: {}",
            if self.intermediate.is_empty() {
                SILVER_ANSI
            } else {
                GOLD_ANSI
            },
            self.tail.trace.len()
        );
        if self.verbosity > 1 {
            self.dump_current()
        } else if self.verbosity > 0 {
            self.dump_trace()
        }

        Ok(())
    }
}

pub fn new(gold: bool, verbosity: u8) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(
        9,
        "Rope Bridge",
        Day9::new(if gold { 8 } else { 0 }, verbosity),
    ))
}
