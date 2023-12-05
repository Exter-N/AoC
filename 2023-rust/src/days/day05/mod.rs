use std::{error::Error, mem::take, ops::Range};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, u64},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, terminated},
};

use crate::{
    days::{GOLD_ANSI, SILVER_ANSI},
    line_stream::{parse_full_string, LineStreamHandler},
};

use super::Day;

#[derive(Clone, Debug)]
struct Id {
    value: Range<u64>,
    category: u32,
}

enum IdMapResult {
    Single(Id),
    Double(Id, Id),
    Triple(Id, Id, Id),
}

impl IdMapResult {
    fn split_first(self) -> (Id, Option<Self>) {
        match self {
            Self::Single(first) => (first, None),
            Self::Double(first, second) => (first, Some(Self::Single(second))),
            Self::Triple(first, second, third) => (first, Some(Self::Double(second, third))),
        }
    }
}

impl IntoIterator for IdMapResult {
    type Item = Id;

    type IntoIter = IdMapIter;

    fn into_iter(self) -> Self::IntoIter {
        IdMapIter { next: Some(self) }
    }
}

struct IdMapIter {
    next: Option<IdMapResult>,
}

impl Iterator for IdMapIter {
    type Item = Id;

    fn next(&mut self) -> Option<Self::Item> {
        match take(&mut self.next) {
            Some(next) => {
                let (result, rest) = next.split_first();
                self.next = rest;
                Some(result)
            }
            None => None,
        }
    }
}

impl Id {
    fn new(value: Range<u64>) -> Self {
        Self { value, category: 0 }
    }
    fn split_at(&self, at: u64) -> Option<(Self, Self)> {
        if self.value.contains(&at) {
            Some((
                Self {
                    value: self.value.start..at,
                    category: self.category,
                },
                Self {
                    value: at..self.value.end,
                    category: self.category,
                },
            ))
        } else {
            None
        }
    }
    fn map_single(&self, source: &Range<u64>, destination: u64, category: u32) -> Option<Self> {
        if self.category == category
            || self.value.end <= source.start
            || self.value.start >= source.end
        {
            Some(self.clone())
        } else if self.value.start >= source.start && self.value.end <= source.end {
            Some(Self {
                value: (self.value.start - source.start + destination)
                    ..(self.value.end - source.start + destination),
                category,
            })
        } else {
            None
        }
    }
    fn map(&self, source: &Range<u64>, destination: u64, category: u32) -> IdMapResult {
        if let Some(single) = self.map_single(source, destination, category) {
            return IdMapResult::Single(single);
        }

        if self.value.end <= source.end {
            let (before, within) = self.split_at(source.start).unwrap();
            IdMapResult::Double(
                before.map_single(source, destination, category).unwrap(),
                within.map_single(source, destination, category).unwrap(),
            )
        } else if self.value.start >= source.start {
            let (within, after) = self.split_at(source.end).unwrap();
            IdMapResult::Double(
                within.map_single(source, destination, category).unwrap(),
                after.map_single(source, destination, category).unwrap(),
            )
        } else {
            let (before, rest) = self.split_at(source.start).unwrap();
            let (within, after) = rest.split_at(source.end).unwrap();
            IdMapResult::Triple(
                before.map_single(source, destination, category).unwrap(),
                within.map_single(source, destination, category).unwrap(),
                after.map_single(source, destination, category).unwrap(),
            )
        }
    }
}

struct Day5 {
    gold: bool,
    ids: Vec<Id>,
    category: u32,
}

impl Day5 {
    fn new(gold: bool) -> Self {
        Self {
            gold,
            ids: Vec::new(),
            category: 0,
        }
    }
}

enum Line {
    Seeds(Vec<u64>),
    MapHeader,
    Mapping(Range<u64>, u64),
}

impl LineStreamHandler for Day5 {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if line.is_empty() {
            return Ok(());
        }

        match parse_full_string(
            line,
            alt((
                map(
                    preceded(tag("seeds: "), separated_list1(tag(" "), u64)),
                    |vec| Line::Seeds(vec),
                ),
                map(
                    terminated(separated_pair(alpha1, tag("-to-"), alpha1), tag(" map:")),
                    |_| Line::MapHeader,
                ),
                map(
                    separated_pair(
                        u64,
                        tag(" "),
                        map(
                            separated_pair(u64, tag(" "), u64),
                            |(source_start, length)| source_start..(source_start + length),
                        ),
                    ),
                    |(destination, source)| Line::Mapping(source, destination),
                ),
            )),
        )? {
            Line::Seeds(vec) => {
                if self.gold {
                    for chunk in vec.chunks(2) {
                        self.ids.push(Id::new(chunk[0]..(chunk[0] + chunk[1])));
                    }
                } else {
                    for id in vec {
                        self.ids.push(Id::new(id..(id + 1)));
                    }
                }
            }
            Line::MapHeader => {
                self.category += 1;
            }
            Line::Mapping(source, destination) => {
                let new_ids = take(&mut self.ids)
                    .into_iter()
                    .flat_map(|id| id.map(&source, destination, self.category));
                self.ids.extend(new_ids);
            }
        };

        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        let mut lowest_id = u64::MAX;
        for id in self.ids {
            lowest_id = lowest_id.min(id.value.start);
        }
        println!(
            "[{}] Lowest location: {}",
            if self.gold { GOLD_ANSI } else { SILVER_ANSI },
            lowest_id
        );
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(Day::new(
        5,
        "If You Give A Seed A Fertilizer",
        Day5::new(gold),
    ))
}
