use std::error::Error;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, multispace1, u64},
    combinator::value,
    multi::{many0_count, many1},
    sequence::{pair, preceded, separated_pair},
};

use crate::{
    days::GOLD_ANSI,
    line_stream::{parse_full_string, LineStreamHandler},
};

use super::{Day, SILVER_ANSI};

struct Day6 {
    gold: bool,
    times: Vec<(usize, u64)>,
    distances: Vec<(usize, u64)>,
}

impl Day6 {
    fn new(gold: bool) -> Self {
        Self {
            gold,
            times: Vec::new(),
            distances: Vec::new(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Field {
    Time,
    Distance,
}

impl LineStreamHandler for Day6 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (field, mut value) = parse_full_string(
            line,
            separated_pair(
                alt((
                    value(Field::Time, tag("Time")),
                    value(Field::Distance, tag("Distance")),
                )),
                char(':'),
                many1(preceded(multispace1, pair(many0_count(char('0')), u64))),
            ),
        )?;
        if self.gold {
            let mut concat = 0u64;
            for (leading_zeros, num) in value {
                let magnitude = (num as f64).log10().floor() as u32 + 1 + (leading_zeros as u32);
                concat = (concat * 10u64.pow(magnitude)) + num;
            };
            value = vec![(0, concat)];
        }
        match field {
            Field::Time => {
                self.times = value;
            }
            Field::Distance => {
                self.distances = value;
            }
        }
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn std::error::Error>> {
        let mut product = 1u64;
        for ((_, time), (_, distance)) in self.times.into_iter().zip(self.distances.into_iter()) {
            println!("[-] Time: {} - Distance: {}", time, distance);
            let f_time = time as f64;
            let f_distance = distance as f64;
            let f_min = (f_time - (f_time * f_time - 4.0 * f_distance).sqrt()) / 2.0;
            let min = f_min.floor() as u64 + 1;
            let ways_to_beat = (time / 2 - min) * 2 + time % 2 + 1;
            product *= ways_to_beat;
        }
        println!(
            "[{}] Ways to beat records: {}",
            if self.gold { GOLD_ANSI } else { SILVER_ANSI },
            product
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(6, "Wait For It", Day6::new(gold)))
}
