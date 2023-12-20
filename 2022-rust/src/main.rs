use aoc_common_rs::day::Day;
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
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod day24;
mod day25;

use cli::{Cli, Commands};

impl TryFrom<Commands> for Day {
    type Error = Box<dyn Error>;

    fn try_from(value: Commands) -> Result<Self, Self::Error> {
        match value {
            Commands::Day01 { verbose } => day01::new(verbose),
            Commands::Day02 { gold } => day02::new(gold),
            Commands::Day03 => day03::new(),
            Commands::Day04 => day04::new(),
            Commands::Day05 { gold, verbose } => day05::new(gold, verbose),
            Commands::Day06 { gold } => day06::new(gold),
            Commands::Day07 { verbose } => day07::new(verbose),
            Commands::Day08 { verbosity } => day08::new(verbosity),
            Commands::Day09 { gold, verbosity } => day09::new(gold, verbosity),
            Commands::Day10 { verbose } => day10::new(verbose),
            Commands::Day11 { gold, verbosity } => day11::new(gold, verbosity),
            Commands::Day12 { gold, verbose } => day12::new(gold, verbose),
            Commands::Day13 => day13::new(),
            Commands::Day14 { gold, verbose } => day14::new(gold, verbose),
            Commands::Day15 { sample, verbose } => day15::new(sample, verbose),
            Commands::Day16 => day16::new(),
            Commands::Day17 { gold, verbose } => day17::new(gold, verbose),
            Commands::Day18 => day18::new(),
            Commands::Day19 => day19::new(),
            Commands::Day20 { gold } => day20::new(gold),
            Commands::Day21 { gold } => day21::new(gold),
            Commands::Day22 { gold } => day22::new(gold),
            Commands::Day23 => day23::new(),
            Commands::Day24 => day24::new(),
            Commands::Day25 => day25::new(),
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
