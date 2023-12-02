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
    }?;

    println!("--- Day {}: {} ---", day.number, day.title);

    let stdin = io::BufReader::new(io::stdin());
    for line in stdin.lines() {
        day.update(line?.as_str())?;
    }

    day.finish()
}
