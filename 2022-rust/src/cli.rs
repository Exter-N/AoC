use clap::{ArgAction, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub timed: bool,
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Day01 {
        #[arg(short, long)]
        verbose: bool,
    },
    Day02 {
        #[arg(short, long)]
        gold: bool,
    },
    Day03,
    Day04,
    Day05 {
        #[arg(short, long)]
        gold: bool,
        #[arg(short, long)]
        verbose: bool,
    },
    Day06 {
        #[arg(short, long)]
        gold: bool,
    },
    Day07 {
        #[arg(short, long)]
        verbose: bool,
    },
    Day08 {
        #[arg(short, long = "verbose", action = ArgAction::Count)]
        verbosity: u8,
    },
    Day09 {
        #[arg(short, long)]
        gold: bool,
        #[arg(short, long = "verbose", action = ArgAction::Count)]
        verbosity: u8,
    },
    Day10 {
        #[arg(short, long)]
        verbose: bool,
    },
    Day11 {
        #[arg(short, long)]
        gold: bool,
        #[arg(short, long = "verbose", action = ArgAction::Count)]
        verbosity: u8,
    },
    Day12 {
        #[arg(short, long)]
        gold: bool,
        #[arg(short, long)]
        verbose: bool,
    },
    Day13,
    Day14 {
        #[arg(short, long)]
        gold: bool,
        #[arg(short, long)]
        verbose: bool,
    },
    Day15 {
        #[arg(short, long)]
        sample: bool,
        #[arg(short, long)]
        verbose: bool,
    },
    Day16,
    Day17 {
        #[arg(short, long)]
        gold: bool,
        #[arg(short, long)]
        verbose: bool,
    },
    Day18,
    Day19,
    Day20 {
        #[arg(short, long)]
        gold: bool,
    },
    Day21 {
        #[arg(short, long)]
        gold: bool,
    },
    Day22 {
        #[arg(short, long)]
        gold: bool,
    },
    Day23,
    Day24,
    Day25,
}
