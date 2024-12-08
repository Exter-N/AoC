use clap::Parser;

use std::io;
use std::{error::Error, io::BufRead};

mod cli;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

use aoc_common_rs::day::Day;
use cli::{Cli, Commands};

impl TryFrom<Commands> for Day {
    type Error = Box<dyn Error>;

    fn try_from(value: Commands) -> Result<Self, Self::Error> {
        match value {
            Commands::Day01 { gold } => day01::new(gold),
            Commands::Day02 { gold } => day02::new(gold),
            Commands::Day03 { gold } => day03::new(gold),
            Commands::Day04 => day04::new(),
            Commands::Day05 => day05::new(),
            Commands::Day06 => day06::new(),
            Commands::Day07 { gold } => day07::new(gold),
            Commands::Day08 { gold } => day08::new(gold),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let day: Day = cli.command.try_into()?;

    if day.display_banner {
        eprintln!("--- Day {}: {} ---", day.number, day.title);
    }

    let stdin = io::BufReader::new(io::stdin());
    for line in stdin.lines() {
        day.update(line?.as_str())?;
    }

    day.finish()
}
