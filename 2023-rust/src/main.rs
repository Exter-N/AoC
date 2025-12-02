use clap::Parser;

use std::error::Error;

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

use aoc_common_rs::day::{run, Day};
use cli::{Cli, Commands};

impl TryFrom<Commands> for Day {
    type Error = Box<dyn Error>;

    fn try_from(value: Commands) -> Result<Self, Self::Error> {
        match value {
            Commands::Day01 { gold, verbose } => day01::new(gold, verbose),
            Commands::Day02 => day02::new(),
            Commands::Day03 => day03::new(),
            Commands::Day04 { gold } => day04::new(gold),
            Commands::Day05 { gold } => day05::new(gold),
            Commands::Day06 { gold } => day06::new(gold),
            Commands::Day07 { gold } => day07::new(gold),
            Commands::Day08 { gold } => day08::new(gold),
            Commands::Day09 { gold } => day09::new(gold),
            Commands::Day10 => day10::new(),
            Commands::Day11 => day11::new(),
            Commands::Day12 { gold } => day12::new(gold),
            Commands::Day13 { gold } => day13::new(gold),
            Commands::Day14 { gold } => day14::new(gold),
            Commands::Day15 => day15::new(),
            Commands::Day16 { gold } => day16::new(gold),
            Commands::Day17 { gold } => day17::new(gold),
            Commands::Day18 { gold } => day18::new(gold),
            Commands::Day19 { gold } => day19::new(gold),
            Commands::Day20 { gold, export } => day20::new(gold, export),
            Commands::Day21 { gold, steps } => day21::new(gold, steps),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    run(cli.command.try_into()?, cli.timed)
}
