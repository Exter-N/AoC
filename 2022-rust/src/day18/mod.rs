use std::collections::{HashSet, VecDeque};
use std::error::Error;

use nom::character::complete::{char, i16};
use nom::combinator::map;
use nom::sequence::tuple;

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
    point::{Direction3, Point3},
};

#[derive(Default)]
struct Day18 {
    min: Option<Point3<i16>>,
    max: Option<Point3<i16>>,
    model: HashSet<Point3<i16>>,
}

impl Day18 {
    fn new() -> Self {
        Default::default()
    }
    fn total_surface_area(&self) -> u32 {
        let mut sa: u32 = 0;
        for pt in self.model.iter() {
            for dir in Direction3::all() {
                if !self.model.contains(&pt.next_towards(dir)) {
                    sa += 1;
                }
            }
        }

        sa
    }
    fn outer_surface_area(&self) -> u32 {
        let mut sa: u32 = 0;
        if let Some(mut min) = self.min {
            if let Some(mut max) = self.max {
                min -= Point3(1, 1, 1);
                max += Point3(1, 1, 1);
                let mut shell_model: HashSet<Point3<i16>> = HashSet::new();
                let mut queue: VecDeque<Point3<i16>> = VecDeque::new();
                queue.push_back(min);
                while let Some(pt) = queue.pop_front() {
                    if !self.model.contains(&pt) && !shell_model.contains(&pt) {
                        shell_model.insert(pt);
                        for dir in Direction3::all() {
                            let pt_n = pt.next_towards(dir);
                            if pt_n.0 >= min.0
                                && pt_n.0 <= max.0
                                && pt_n.1 >= min.1
                                && pt_n.1 <= max.1
                                && pt_n.2 >= min.2
                                && pt_n.2 <= max.2
                            {
                                queue.push_back(pt_n);
                            }
                        }
                    }
                }
                for pt in self.model.iter() {
                    for dir in Direction3::all() {
                        if shell_model.contains(&pt.next_towards(dir)) {
                            sa += 1;
                        }
                    }
                }
            }
        }

        sa
    }
}

impl LineStreamHandler for Day18 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let pt = parse_full_string(
            line,
            map(
                tuple((i16, char(','), i16, char(','), i16)),
                |(x, _, y, _, z)| Point3(x, y, z),
            ),
        )?;
        self.model.insert(pt);
        if let Some(min) = self.min {
            self.min = Some(min.componentwise_min(pt));
        } else {
            self.min = Some(pt);
        }
        if let Some(max) = self.max {
            self.max = Some(max.componentwise_max(pt));
        } else {
            self.max = Some(pt);
        }

        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Total surface area: {}",
            SILVER_ANSI,
            self.total_surface_area()
        );
        println!(
            "[{}] Outer surface area: {}",
            GOLD_ANSI,
            self.outer_surface_area()
        );

        Ok(())
    }
}

pub fn new() -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(18, "Boiling Boulders", Day18::new()))
}
