use std::{collections::HashSet, error::Error};

use aoc_common_rs::{
    day::{Day, SILVER_ANSI},
    line_stream::{LineStreamHandler, parse_full_string},
};
use nom::{
    character::complete::{char, u64},
    multi::separated_list1,
    sequence::separated_pair,
};

fn upper(num: u64, parts: u32) -> (u64, bool) {
    if num == 0 {
        return (0, true);
    }

    let log = num.ilog10();
    if ((log + 1) % parts) != 0 {
        return (10u64.pow(log / parts), false);
    }

    let divisor = 10u64.pow((log + 2) / parts * (parts - 1));
    let upper = num / divisor;
    let lower = num % divisor;
    let lower_ref = repeat(upper, parts - 1);
    if lower == lower_ref {
        (upper, true)
    } else if lower < lower_ref {
        (upper, false)
    } else {
        (upper + 1, false)
    }
}

fn repeat(num: u64, times: u32) -> u64 {
    if num == 0 {
        return 0;
    }

    let mut accumulator = num;
    for _ in 1..times {
        accumulator = accumulator * 10u64.pow(num.ilog10() + 1) + num;
    }

    accumulator
}

struct Day2 {
    gold: bool,
    invalids: HashSet<u64>,
}

impl Day2 {
    fn new(gold: bool) -> Self {
        Self {
            gold,
            invalids: HashSet::new(),
        }
    }

    fn collect_invalid(&mut self, lower_bound: u64, upper_bound: u64, parts: u32) {
        let (lb_upper, _) = upper(lower_bound, parts);
        let (mut ub_upper, ub_exact) = upper(upper_bound, parts);
        if ub_exact {
            ub_upper += 1;
        }

        for upper in lb_upper..ub_upper {
            self.invalids.insert(repeat(upper, parts));
        }
    }
}

impl LineStreamHandler for Day2 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let ranges = parse_full_string(
            line,
            separated_list1(char(','), separated_pair(u64, char('-'), u64)),
        )?;
        for (lower_bound, upper_bound) in ranges {
            if self.gold {
                for parts in 2..=(upper_bound.checked_ilog10().unwrap_or(0) + 1) {
                    self.collect_invalid(lower_bound, upper_bound, parts);
                }
            } else {
                self.collect_invalid(lower_bound, upper_bound, 2);
            }
        }
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!("[-] Number of invalid IDs: {}", self.invalids.len());
        println!(
            "[{}] Sum of invalid IDs:    {}",
            SILVER_ANSI,
            self.invalids.into_iter().sum::<u64>()
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(2, "Gift Shop", Day2::new(gold)))
}
