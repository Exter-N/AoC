use clap::Parser;

use std::error::Error;

mod cli;
mod day01;
mod day02;
mod day03;

use aoc_common_rs::day::{Day, run};
use cli::{Cli, Commands};

impl TryFrom<Commands> for Day {
    type Error = Box<dyn Error>;

    fn try_from(value: Commands) -> Result<Self, Self::Error> {
        match value {
            Commands::Day01 { gold } => day01::new(gold),
            Commands::Day02 { gold } => day02::new(gold),
            Commands::Day03 { gold } => day03::new(gold),
            Commands::Day04 => unimplemented!(),
            Commands::Day05 => unimplemented!(),
            Commands::Day06 => unimplemented!(),
            Commands::Day07 => unimplemented!(),
            Commands::Day08 => unimplemented!(),
            Commands::Day09 => unimplemented!(),
            Commands::Day10 => unimplemented!(),
            Commands::Day11 => unimplemented!(),
            Commands::Day12 => unimplemented!(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    run(cli.command.try_into()?, cli.timed)
}
