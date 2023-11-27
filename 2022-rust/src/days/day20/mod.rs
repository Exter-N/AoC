use std::collections::HashMap;
use std::error::Error;

use nom::character::complete::i64;

use super::{parse_full_string, LineStreamHandler, GOLD_ANSI, SILVER_ANSI};

#[derive(Debug, Default)]
struct Day20 {
    key: i64,
    rounds: u16,
    nums: Vec<i64>,
    next: HashMap<usize, usize>,
    prev: HashMap<usize, usize>,
    first: usize,
    zero: usize,
}

impl Day20 {
    fn new(key: i64, rounds: u16) -> Self {
        Self {
            key,
            rounds,
            ..Default::default()
        }
    }
    fn nth_after(&self, pos: usize, nth: usize) -> usize {
        let mut p = pos;
        for _ in 0..(nth % (self.nums.len() - 1)) {
            p = self.next[&p];
        }

        p
    }
    fn nth_before(&self, pos: usize, nth: usize) -> usize {
        let mut p = pos;
        for _ in 0..(nth % (self.nums.len() - 1)) {
            p = self.prev[&p];
        }

        p
    }
    fn partially_remove(&mut self, pos: usize) {
        self.next.insert(self.prev[&pos], self.next[&pos]);
        self.prev.insert(self.next[&pos], self.prev[&pos]);
    }
    fn insert_after(&mut self, pos: usize, after: usize) {
        let before = self.next[&after];
        self.next.insert(after, pos);
        self.next.insert(pos, before);
        self.prev.insert(before, pos);
        self.prev.insert(pos, after);
    }
    fn mix(&mut self) {
        for pos in 0..self.nums.len() {
            if self.first == pos {
                self.first = self.next[&pos];
            }
            let num = self.nums[pos];
            let mut after = self.prev[&pos];
            self.partially_remove(pos);
            if num > 0 {
                after = self.nth_after(after, num as usize);
            } else if num < 0 {
                after = self.nth_before(after, (-num) as usize);
            }
            self.insert_after(pos, after);
        }
    }
}

impl LineStreamHandler for Day20 {
    fn update(&mut self, line: &str) -> Result<Option<Box<dyn LineStreamHandler>>, Box<dyn Error>> {
        let num = parse_full_string(line, i64)?;
        let pos = self.nums.len();
        self.nums.push(num * self.key);
        if pos > 0 {
            self.next.insert(pos - 1, pos);
            self.prev.insert(pos, pos - 1);
        }

        Ok(None)
    }

    fn finish(&mut self) -> Result<(), Box<dyn Error>> {
        if !self.nums.is_empty() {
            let last = self.nums.len() - 1;
            self.next.insert(last, 0);
            self.prev.insert(0, last);
        }
        if let Some(zero) = self.nums.iter().position(|num| *num == 0) {
            self.zero = zero;
        } else {
            return Err(Box::from("0 not found in list"));
        }
        for _ in 0..self.rounds {
            self.mix();
        }
        let x = self.nth_after(self.zero, 1000);
        let y = self.nth_after(x, 1000);
        let z = self.nth_after(y, 1000);
        println!(
            "[{}] Sum of coordinates: {}",
            if self.key == 1 {
                SILVER_ANSI
            } else {
                GOLD_ANSI
            },
            self.nums[x] + self.nums[y] + self.nums[z]
        );
        println!(
            "[-] X = {}, Y = {}, Z = {}",
            self.nums[x], self.nums[y], self.nums[z]
        );

        Ok(())
    }
}

pub fn new(gold: bool) -> Result<(u8, &'static str, Box<dyn LineStreamHandler>), Box<dyn Error>> {
    Ok((
        20,
        "Grove Positioning System",
        Box::new(Day20::new(
            if gold { 811_589_153 } else { 1 },
            if gold { 10 } else { 1 },
        )),
    ))
}
