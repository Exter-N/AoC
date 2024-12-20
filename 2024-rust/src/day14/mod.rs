use std::{
    cmp::Ordering,
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult, Write},
};

use aoc_common_rs::{
    day::{Day, SILVER_ANSI}, line_stream::{parse_full_string, LineStreamHandler}, math::lcm, point::Point2, terrain::Terrain
};

use nom::{
    bytes::complete::tag,
    character::complete::{char, i64, u64},
    combinator::map,
    sequence::{preceded, separated_pair},
};

#[derive(Clone, Copy, Debug)]
struct Robot {
    position: (u64, u64),
    velocity: (i64, i64),
}

impl Robot {
    fn step(&mut self, mut steps: i64, width: u64, height: u64) {
        steps = steps.rem_euclid(lcm(width, height) as i64);
        if steps == 0 {
            return;
        }
        let (vxi, vyi) = self.velocity;
        let vx = vxi.rem_euclid(width as i64) as u64;
        let vy = vyi.rem_euclid(height as i64) as u64;
        self.position = (
            (self.position.0 + vx * (steps as u64)) % width,
            (self.position.1 + vy * (steps as u64)) % height,
        );
    }

    fn position_usize(&self) -> Point2<usize> {
        Point2(self.position.0 as usize, self.position.1 as usize)
    }
}

#[derive(Clone, Copy, Default)]
struct HistogramCell(usize);

impl Display for HistogramCell {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        f.write_char(match self.0 {
            0 => '.',
            1 => '1',
            2 => '2',
            3 => '3',
            4 => '4',
            5 => '5',
            6 => '6',
            7 => '7',
            8 => '8',
            9 => '9',
            _ => '+',
        })
    }
}

#[derive(Clone, Debug)]
struct Day14 {
    robots: Vec<Robot>,
    display_frame: Option<i64>,
}

impl Day14 {
    fn new(display_frame: Option<i64>) -> Self {
        Self {
            robots: Vec::new(),
            display_frame,
        }
    }

    fn terrain_size(&self) -> (u64, u64) {
        if self.robots.len() > 20 {
            (101, 103)
        } else {
            (11, 7)
        }
    }

    fn step_all(&mut self, steps: i64) {
        let (width, height) = self.terrain_size();
        for robot in self.robots.iter_mut() {
            robot.step(steps, width, height);
        }
    }

    fn compute_histogram(&self) -> Terrain<HistogramCell> {
        let size = self.terrain_size();
        let mut terrain: Terrain<HistogramCell> =
            Terrain::new_with(size.0 as usize, size.1 as usize, |_| Default::default());
        for robot in &self.robots {
            terrain[robot.position_usize()].0 += 1;
        }
        terrain
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
        if let Some(display_frame) = self.display_frame {
            let size = self.terrain_size();
            println!("[-] Displaying frame {} of {}", display_frame, lcm(size.0, size.1));
            let mut display_clone = (*self).clone();
            display_clone.step_all(display_frame);
            println!("{}", display_clone.compute_histogram());
        }
        self.step_all(100);
        let (width, height) = self.terrain_size();
        let (pivot_x, pivot_y) = (width >> 1, height >> 1);
        let mut count_tl = 0usize;
        let mut count_tr = 0usize;
        let mut count_bl = 0usize;
        let mut count_br = 0usize;
        for robot in self.robots.iter() {
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

pub fn new(display_frame: Option<i64>) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(14, "Restroom Redoubt", Day14::new(display_frame)))
}
