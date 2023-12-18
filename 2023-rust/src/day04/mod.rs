use std::{
    collections::{HashSet, VecDeque},
    error::Error,
};

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, u32},
    multi::fold_many1,
    sequence::{preceded, separated_pair, terminated},
};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};

struct Day4 {
    score: u32,
    gold: bool,
    next_copies: VecDeque<usize>,
}

impl Day4 {
    fn new(gold: bool) -> Self {
        Self {
            score: 0,
            gold,
            next_copies: VecDeque::new(),
        }
    }
}

impl LineStreamHandler for Day4 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        let (_, (winning, ours)) = parse_full_string(
            line,
            separated_pair(
                preceded(tag("Card"), preceded(multispace0, u32)),
                terminated(tag(":"), multispace0),
                separated_pair(
                    fold_many1(
                        terminated(u32, multispace0),
                        || HashSet::new(),
                        |mut set, num| {
                            set.insert(num);
                            set
                        },
                    ),
                    terminated(tag("|"), multispace0),
                    fold_many1(
                        terminated(u32, multispace0),
                        || HashSet::new(),
                        |mut set, num| {
                            set.insert(num);
                            set
                        },
                    ),
                ),
            ),
        )?;
        let copies = 1 + self.next_copies.pop_front().unwrap_or(0);
        if self.gold {
            self.score += copies as u32;
        }
        let winning_count = ours.into_iter().filter(|num| winning.contains(num)).count();
        if winning_count > 0 {
            if self.gold {
                for i in 0..winning_count {
                    if let Some(next_copies) = self.next_copies.get_mut(i) {
                        *next_copies += copies;
                    } else {
                        self.next_copies.push_back(copies);
                    }
                }
            } else {
                self.score += 1 << (winning_count - 1);
            }
        }
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        if self.gold {
            println!(
                "[{}] Total number of scratchcards: {}",
                GOLD_ANSI, self.score
            );
        } else {
            println!("[{}] Total score: {}", SILVER_ANSI, self.score);
        }
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(4, "Scratchcards", Day4::new(gold)))
}
