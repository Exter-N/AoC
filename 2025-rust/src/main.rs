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

use aoc_common_rs::day::{Day, run};
use cli::{Cli, Commands};

impl TryFrom<Commands> for Day {
    type Error = Box<dyn Error>;

    fn try_from(value: Commands) -> Result<Self, Self::Error> {
        match value {
            Commands::Day01 { gold, verbose } => day01::new(gold, verbose),
            Commands::Day02 { gold } => day02::new(gold),
            Commands::Day03 { gold } => day03::new(gold),
            Commands::Day04 { verbose } => day04::new(verbose),
            Commands::Day05 => day05::new(),
            Commands::Day06 { gold } => day06::new(gold),
            Commands::Day07 => day07::new(),
            Commands::Day08 { gold } => day08::new(gold),
            Commands::Day09 { gold } => day09::new(gold),
            Commands::Day10 { gold } => day10::new(gold),
            Commands::Day11 => unimplemented!(),
            Commands::Day12 => unimplemented!(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    run(cli.command.try_into()?, cli.timed)
}
