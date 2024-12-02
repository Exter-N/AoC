use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
    unwrap_either,
};
use nom::error::Error as NomError;
use nom::{
    character::complete::{multispace0, u32},
    sequence::separated_pair,
};
use std::{collections::HashMap, error::Error};

struct Day1Silver {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Day1Silver {
    fn new() -> Self {
        Self {
            left: Vec::new(),
            right: Vec::new(),
        }
    }
}

fn parse_line(line: &str) -> Result<(u32, u32), NomError<usize>> {
    parse_full_string(line, separated_pair(u32, multispace0, u32))
}

impl LineStreamHandler for Day1Silver {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let (left, right) = parse_line(line)?;
        let left_pos = unwrap_either(self.left.binary_search(&left));
        self.left.insert(left_pos, left);
        let right_pos = unwrap_either(self.right.binary_search(&right));
        self.right.insert(right_pos, right);
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        let mut distance = 0u32;
        for (left, right) in self.left.iter().zip(self.right.iter()) {
            distance += left.abs_diff(*right);
        }
        println!(
            "[{}] Total distance between lists: {}",
            SILVER_ANSI, distance
        );
        Ok(())
    }
}

struct Day1Gold {
    left: HashMap<u32, usize>,
    right: HashMap<u32, usize>,
}

impl Day1Gold {
    fn new() -> Self {
        Self {
            left: HashMap::new(),
            right: HashMap::new(),
        }
    }
}

impl LineStreamHandler for Day1Gold {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let (left, right) = parse_line(line)?;
        self.left
            .entry(left)
            .and_modify(|count| {
                *count += 1;
            })
            .or_insert(1);
        self.right
            .entry(right)
            .and_modify(|count| {
                *count += 1;
            })
            .or_insert(1);
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        let mut similarity = 0usize;
        for (id, left_count) in self.left.iter() {
            if let Some(right_count) = self.right.get(id) {
                similarity += (*id as usize) * left_count * right_count;
            }
        }
        println!("[{}] Similarity score: {}", GOLD_ANSI, similarity);
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(if gold {
        Day::new(1, "Historian Hysteria", Day1Gold::new())
    } else {
        Day::new(1, "Historian Hysteria", Day1Silver::new())
    })
}
