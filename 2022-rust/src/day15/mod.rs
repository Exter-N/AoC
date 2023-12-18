use std::cmp::{max, min};
use std::collections::HashSet;
use std::error::Error;

use nom::bytes::complete::tag;
use nom::character::complete::i32;
use nom::combinator::map;
use nom::sequence::{preceded, separated_pair};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
    math::abs_diff,
    multi_range::MultiRangeInclusive,
    point::Point2,
};


const SAMPLE_MAP_SIZE: usize = 20;
const MAP_SIZE: usize = 4_000_000;

#[derive(Clone, Debug, Default)]
struct Row {
    beacons: HashSet<i32>,
    impossibles: MultiRangeInclusive<i32>,
}

impl Row {
    fn count_impossibles(&self) -> u32 {
        let mut count: u32 = self.impossibles.count() as u32;
        for beacon in &self.beacons {
            if self.impossibles.contains(beacon) {
                count -= 1;
            }
        }

        count
    }
}

#[derive(Default)]
struct Day15 {
    verbose: bool,
    map_size: usize,
    rows: Vec<Row>,
}

impl Day15 {
    fn new(verbose: bool, map_size: usize) -> Self {
        Self {
            verbose,
            map_size,
            rows: vec![Default::default(); (map_size + 1) as usize],
            ..Default::default()
        }
    }
    fn clip(&mut self) {
        for row in self.rows.iter_mut() {
            row.impossibles.retain(0..=(self.map_size as i32));
        }
    }
    fn distress_tuning_frequency(&self) -> Option<i64> {
        for (row, y) in self.rows.iter().zip(0i64..) {
            if row.impossibles.count() < (self.map_size + 1) as i32 {
                let mut position: MultiRangeInclusive<i32> = MultiRangeInclusive::new();
                position.insert(0..=(self.map_size as i32));
                for range in row.impossibles.iter() {
                    position.remove(range.to_owned());
                }
                return Some((*position[0].start() as i64) * (MAP_SIZE as i64) + y);
            }
        }

        None
    }
}

impl LineStreamHandler for Day15 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let point = move || {
            map(
                preceded(tag("x="), separated_pair(i32, tag(", y="), i32)),
                |(x, y)| Point2(x, y),
            )
        };
        let (sensor, beacon) = parse_full_string(
            line,
            preceded(
                tag("Sensor at "),
                separated_pair(point(), tag(": closest beacon is at "), point()),
            ),
        )?;
        let sensor_y = usize::try_from(sensor.1)?;
        let distance = sensor.manhattan_distance(beacon) as usize;
        for y in max(sensor_y.saturating_sub(distance), 0)
            ..=min(sensor_y.saturating_add(distance), self.map_size)
        {
            let dist_to_y = abs_diff(sensor_y, y);
            if distance >= dist_to_y {
                let remaining = (distance - dist_to_y) as i32;
                self.rows[y as usize]
                    .impossibles
                    .insert((sensor.0 - remaining)..=(sensor.0 + remaining));
            }
        }
        if beacon.1 >= 0 && beacon.1 <= self.map_size as i32 {
            self.rows[beacon.1 as usize].beacons.insert(beacon.0);
        }
        eprint!("{}", '.');

        Ok(())
    }

    fn finish(mut self: Box<Self>) -> Result<(), Box<dyn Error>> {
        eprintln!();
        let mid_y = self.map_size / 2;
        println!(
            "[{}] # of impossible positions at mid y: {:?}",
            SILVER_ANSI,
            self.rows[mid_y].count_impossibles()
        );
        if self.verbose {
            println!(
                "[-] Beacon positions at mid y:          {:?}",
                self.rows[mid_y].beacons
            );
            println!(
                "[-] Impossible positions at mid y:      {:?}",
                self.rows[mid_y].impossibles
            );
        }
        self.clip();
        println!(
            "[{}] Distress tuning frequency:          {:?}",
            GOLD_ANSI,
            self.distress_tuning_frequency()
        );

        Ok(())
    }
}

pub fn new(sample: bool, verbose: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(
        15,
        "Beacon Exclusion Zone",
        Day15::new(verbose, if sample { SAMPLE_MAP_SIZE } else { MAP_SIZE }),
    ))
}
