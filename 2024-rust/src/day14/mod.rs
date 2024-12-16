use std::{cmp::Ordering, error::Error};

use aoc_common_rs::{
    day::{Day, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};

use nom::{
    bytes::complete::tag,
    character::complete::{char, i64, u64},
    combinator::map,
    sequence::{preceded, separated_pair},
};

#[derive(Debug)]
struct Robot {
    position: (u64, u64),
    velocity: (i64, i64),
}

impl Robot {
    fn step(&mut self, steps: i64, width: u64, height: u64) {
        if steps == 0 {
            return;
        }
        let (vxi, vyi) = self.velocity;
        let vx = (vxi * steps.signum()).rem_euclid(width as i64) as u64;
        let vy = (vyi * steps.signum()).rem_euclid(height as i64) as u64;
        self.position = (
            (self.position.0 + vx * (steps.abs() as u64)) % width,
            (self.position.1 + vy * (steps.abs() as u64)) % height,
        );
    }
}

#[derive(Debug)]
struct Day14 {
    robots: Vec<Robot>,
}

impl Day14 {
    fn new() -> Self {
        Self { robots: Vec::new() }
    }
}

impl LineStreamHandler for Day14 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        self.robots.push(parse_full_string(
            line,
            map(
                preceded(
                    tag("p="),
                    separated_pair(
                        separated_pair(u64, char(','), u64),
                        tag(" v="),
                        separated_pair(i64, char(','), i64),
                    ),
                ),
                |(position, velocity)| Robot { position, velocity },
            ),
        )?);
        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        let steps = 100;
        let (width, height) = if self.robots.len() > 20 {
            (101, 103)
        } else {
            (11, 7)
        };
        let (pivot_x, pivot_y) = (width >> 1, height >> 1);
        let mut count_tl = 0usize;
        let mut count_tr = 0usize;
        let mut count_bl = 0usize;
        let mut count_br = 0usize;
        for robot in self.robots.iter_mut() {
            robot.step(steps, width, height);
            match (
                robot.position.0.cmp(&pivot_x),
                robot.position.1.cmp(&pivot_y),
            ) {
                (Ordering::Less, Ordering::Less) => {
                    count_tl += 1;
                }
                (Ordering::Greater, Ordering::Less) => {
                    count_tr += 1;
                }
                (Ordering::Less, Ordering::Greater) => {
                    count_bl += 1;
                }
                (Ordering::Greater, Ordering::Greater) => {
                    count_br += 1;
                }
                (_, _) => {}
            }
        }
        println!(
            "[{}] Safety factor: {}",
            SILVER_ANSI,
            count_tl * count_tr * count_bl * count_br
        );
        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(14, "Restroom Redoubt", Day14::new()))
}
