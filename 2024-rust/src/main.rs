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
mod day22;
mod day23;
mod day24;
mod day25;

use aoc_common_rs::day::{run, Day};
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
            Commands::Day09 { gold } => day09::new(gold),
            Commands::Day10 => day10::new(),
            Commands::Day11 => day11::new(),
            Commands::Day12 => day12::new(),
            Commands::Day13 { gold } => day13::new(gold),
            Commands::Day14 { frame } => day14::new(frame),
            Commands::Day15 { gold, verbose } => day15::new(gold, verbose),
            Commands::Day16 { verbose } => day16::new(verbose),
            Commands::Day17 { gold } => day17::new(gold),
            Commands::Day18 { sample, gold } => day18::new(sample, gold),
            Commands::Day19 => day19::new(),
            Commands::Day20 { gold } => day20::new(gold),
            Commands::Day21 { gold } => day21::new(gold),
            Commands::Day22 => day22::new(),
            Commands::Day23 => day23::new(),
            Commands::Day24 => day24::new(),
            Commands::Day25 => day25::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    run(cli.command.try_into()?, cli.timed)
}
