use std::{
    error::Error,
    iter::{Product, Sum},
};

use aoc_common_rs::{
    day::{Day, GOLD_ANSI, SILVER_ANSI},
    line_stream::{LineStreamHandler, parse_full_string},
};
use itertools::Itertools;
use nom::{
    Parser,
    character::complete::{one_of, space0, space1, u64},
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, terminated},
};

fn parse_line<'a, F>(
    line: &'a str,
    field: F,
) -> Result<Vec<<F as Parser<&'a str>>::Output>, nom::error::Error<usize>>
where
    F: Parser<&'a str, Error = nom::error::Error<&'a str>>,
{
    parse_full_string(
        line,
        preceded(space0, terminated(separated_list0(space1, field), space0)),
    )
}

#[derive(Debug)]
enum Aggregate {
    Sum,
    Product,
}

impl Aggregate {
    fn execute<S, I>(self, iter: I) -> S
    where
        I: Iterator,
        S: Sum<I::Item> + Product<I::Item>,
    {
        match self {
            Self::Sum => iter.sum(),
            Self::Product => iter.product(),
        }
    }
}

fn parse_aggregates(line: &str) -> Result<Vec<Aggregate>, nom::error::Error<usize>> {
    parse_line(
        line,
        map(one_of("+*"), |ch| match ch {
            '+' => Aggregate::Sum,
            '*' => Aggregate::Product,
            _ => unreachable!(),
        }),
    )
}

struct Day6Silver {
    columns: Vec<Vec<u64>>,
}

impl Day6Silver {
    fn new() -> Self {
        Self {
            columns: Vec::new(),
        }
    }
}

impl LineStreamHandler for Day6Silver {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if let Ok(aggregates) = parse_aggregates(line) {
            println!(
                "[{}] Sum of results: {}",
                SILVER_ANSI,
                self.columns
                    .iter()
                    .zip(aggregates.into_iter())
                    .map(|(column, op)| op.execute::<u64, _>(column.iter()))
                    .sum::<u64>()
            );
            return Ok(());
        }

        let nums = parse_line(line, u64)?;
        self.columns
            .reserve(nums.len().saturating_sub(self.columns.len()));
        for (num, i) in nums.iter().zip(0usize..) {
            if i >= self.columns.len() {
                self.columns.push(Vec::new());
            }
            self.columns[i].push(*num);
        }

        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

struct Day6Gold {
    columns: Vec<Vec<u8>>,
}

impl Day6Gold {
    fn new() -> Self {
        Self {
            columns: Vec::new(),
        }
    }
}

impl LineStreamHandler for Day6Gold {
    fn update(&mut self, line: &str) -> Result<(), Box<dyn Error>> {
        if let Ok(aggregates) = parse_aggregates(line) {
            let chunks = self
                .columns
                .iter()
                .chunk_by(|col| col.iter().all(|ch| ch.is_ascii_whitespace()));
            let columns = chunks
                .into_iter()
                .filter_map(|(empty, cols)| if empty { None } else { Some(cols) })
                .map(|item| {
                    item.map(|col| {
                        parse_full_string(
                            str::from_utf8(col).unwrap(),
                            preceded(space0, terminated(u64, space0)),
                        )
                        .unwrap()
                    })
                    .collect_vec()
                });
            println!(
                "[{}] Sum of results: {}",
                GOLD_ANSI,
                columns
                    .zip(aggregates.into_iter())
                    .map(|(column, op)| op.execute::<u64, _>(column.iter()))
                    .sum::<u64>()
            );
            return Ok(());
        }

        self.columns
            .reserve(line.len().saturating_sub(self.columns.len()));
        for (ch, i) in line.bytes().zip(0usize..) {
            if i >= self.columns.len() {
                self.columns.push(Vec::new());
            }
            self.columns[i].push(ch);
        }

        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

pub fn new(gold: bool) -> Result<Day, Box<dyn Error>> {
    Ok(if gold {
        Day::new(6, "Trash Compactor", Day6Gold::new())
    } else {
        Day::new(6, "Trash Compactor", Day6Silver::new())
    })
}
