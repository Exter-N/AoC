use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};
use itertools::chain;
use nom::{
    character::complete::{i32, multispace1},
    multi::separated_list1,
};
use std::error::Error;

struct Day2 {
    gold: bool,
    safe_reports: usize,
}

impl Day2 {
    fn new(gold: bool) -> Self {
        Self {
            gold,
            safe_reports: 0,
        }
    }
}

fn is_safe_parts(report1: &[i32], report2: &[i32]) -> bool {
    let mut iter = chain!(report1, report2);
    let mut last = *iter.next().unwrap();
    let mut min_diff = i32::MAX;
    let mut max_diff = i32::MIN;
    for level in iter {
        let diff = level - last;
        min_diff = min_diff.min(diff);
        max_diff = max_diff.max(diff);
        if (min_diff < -3 || max_diff > -1) && (min_diff < 1 || max_diff > 3) {
            return false;
        }
        last = *level;
    }
    true
}

fn is_safe(report: &[i32], almost: bool) -> bool {
    if is_safe_parts(report, &[]) {
        true
    } else {
        if almost {
            for i in 0usize..report.len() {
                if is_safe_parts(&report[..i], &report[(i + 1)..]) {
                    return true;
                }
            }
        }
        false
    }
}

impl LineStreamHandler for Day2 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let report = parse_full_string(line, separated_list1(multispace1, i32))?;
        if is_safe(&report, self.gold) {
            self.safe_reports += 1;
        }
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        println!(
            "[{}] Safe report count: {}",
            if self.gold { GOLD_ANSI } else { SILVER_ANSI },
            self.safe_reports
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(2, "Red-Nosed Reports", Day2::new(gold)))
}
