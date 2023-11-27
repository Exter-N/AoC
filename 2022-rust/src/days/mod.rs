use std::error::Error;

use nom::combinator::eof;
use nom::sequence::terminated;
use nom::{Finish, Parser};

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;
pub mod day17;
pub mod day18;
pub mod day19;
pub mod day20;
pub mod day21;
pub mod day22;
pub mod day23;
pub mod day24;
pub mod day25;

pub const GOLD_ANSI: &str = "\x1B[38;2;255;215;0mG\x1B[m";
pub const SILVER_ANSI: &str = "\x1B[38;2;192;192;192mS\x1B[m";

pub fn parse_full_string<'a, O, F>(s: &'a str, f: F) -> Result<O, nom::error::Error<usize>>
where
    F: Parser<&'a str, O, nom::error::Error<&'a str>>,
{
    match terminated(f, eof)(s).finish() {
        Ok((_, result)) => Ok(result),
        Err(error) => Err(nom::error::Error::new(
            (error.input.as_ptr() as usize) - (s.as_ptr() as usize),
            error.code,
        )),
    }
}

pub trait LineStreamHandler {
    fn update(&mut self, line: &str) -> Result<Option<Box<dyn LineStreamHandler>>, Box<dyn Error>>;
    fn finish(&mut self) -> Result<(), Box<dyn Error>>;
}
