use clap::Parser;

use std::io;
use std::{error::Error, io::BufRead};

mod cli;
mod days;

use aoc_common_rs::day::Day;
use cli::{Cli, Commands};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let day: Day = match cli.command {
        Commands::Day01 { gold, verbose } => days::day01::new(gold, verbose),
        Commands::Day02 => days::day02::new(),
        Commands::Day03 => days::day03::new(),
        Commands::Day04 { gold } => days::day04::new(gold),
        Commands::Day05 { gold } => days::day05::new(gold),
        Commands::Day06 { gold } => days::day06::new(gold),
        Commands::Day07 { gold } => days::day07::new(gold),
        Commands::Day08 { gold } => days::day08::new(gold),
        Commands::Day09 { gold } => days::day09::new(gold),
        Commands::Day10 => days::day10::new(),
        Commands::Day11 => days::day11::new(),
        Commands::Day12 { gold } => days::day12::new(gold),
        Commands::Day13 { gold } => days::day13::new(gold),
        Commands::Day14 { gold } => days::day14::new(gold),
        Commands::Day15 => days::day15::new(),
    }?;

    println!("--- Day {}: {} ---", day.number, day.title);

    let stdin = io::BufReader::new(io::stdin());
    for line in stdin.lines() {
        day.update(line?.as_str())?;
    }

    day.finish()
}
