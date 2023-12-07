use clap::Parser;

use std::io;
use std::{error::Error, io::BufRead};

mod cli;
mod days;
mod line_stream;

use cli::{Cli, Commands};
use days::Day;

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
    }?;

    println!("--- Day {}: {} ---", day.number, day.title);

    let stdin = io::BufReader::new(io::stdin());
    for line in stdin.lines() {
        day.update(line?.as_str())?;
    }

    day.finish()
}
