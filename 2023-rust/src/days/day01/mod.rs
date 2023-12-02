use super::{Day, LineStreamHandler};
use crate::days::{GOLD_ANSI, SILVER_ANSI};
use std::error::Error;

mod digit_matcher;

use digit_matcher::DigitMatcher;

struct Day1 {
    matcher: DigitMatcher,
    verbose: bool,
    calibration_sum: u32,
}

impl Day1 {
    fn new(gold: bool, verbose: bool) -> Self {
        Self {
            matcher: DigitMatcher::new(gold),
            verbose,
            calibration_sum: 0,
        }
    }
}

impl LineStreamHandler for Day1 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut first_digit = None;
        let mut last_digit = None;
        for ch in line.chars() {
            let digit = self.matcher.update(ch);
            if digit.is_some() {
                first_digit = first_digit.or(digit);
                last_digit = digit;
            }
        }
        let calibration_value = first_digit.unwrap() * 10 + last_digit.unwrap();
        if self.verbose {
            println!("[-] {} | {}", calibration_value, line);
        }
        self.calibration_sum += calibration_value as u32;
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn std::error::Error>> {
        println!(
            "[{}] Sum of calibration values: {}",
            if self.matcher.allow_spelled_out {
                GOLD_ANSI
            } else {
                SILVER_ANSI
            },
            self.calibration_sum
        );
        Ok(())
    }
}

pub fn new(gold: bool, verbose: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(1, "Trebuchet?!", Day1::new(gold, verbose)))
}
