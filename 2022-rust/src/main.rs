use aoc_common_rs::day::Day;
use clap::Parser;

use std::io;
use std::{error::Error, io::BufRead};

mod cli;
mod days;

use cli::{Cli, Commands};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let day: Day = match cli.command {
        Commands::Day01 { verbose } => days::day01::new(verbose),
        Commands::Day02 { gold } => days::day02::new(gold),
        Commands::Day03 => days::day03::new(),
        Commands::Day04 => days::day04::new(),
        Commands::Day05 { gold, verbose } => days::day05::new(gold, verbose),
        Commands::Day06 { gold } => days::day06::new(gold),
        Commands::Day07 { verbose } => days::day07::new(verbose),
        Commands::Day08 { verbosity } => days::day08::new(verbosity),
        Commands::Day09 { gold, verbosity } => days::day09::new(gold, verbosity),
        Commands::Day10 { verbose } => days::day10::new(verbose),
        Commands::Day11 { gold, verbosity } => days::day11::new(gold, verbosity),
        Commands::Day12 { gold, verbose } => days::day12::new(gold, verbose),
        Commands::Day13 => days::day13::new(),
        Commands::Day14 { gold, verbose } => days::day14::new(gold, verbose),
        Commands::Day15 { sample, verbose } => days::day15::new(sample, verbose),
        Commands::Day16 => days::day16::new(),
        Commands::Day17 { gold, verbose } => days::day17::new(gold, verbose),
        Commands::Day18 => days::day18::new(),
        Commands::Day19 => days::day19::new(),
        Commands::Day20 { gold } => days::day20::new(gold),
        Commands::Day21 { gold } => days::day21::new(gold),
        Commands::Day22 { gold } => days::day22::new(gold),
        Commands::Day23 => days::day23::new(),
        Commands::Day24 => days::day24::new(),
        Commands::Day25 => days::day25::new(),
    }?;

    println!("--- Day {}: {} ---", day.number, day.title);

    let stdin = io::BufReader::new(io::stdin());
    for line in stdin.lines() {
        day.update(line?.as_str())?;
    }

    day.finish()
}
