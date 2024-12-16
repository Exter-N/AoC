use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Day01 {
        #[arg(short, long)]
        gold: bool,
    },
    Day02 {
        #[arg(short, long)]
        gold: bool,
    },
    Day03 {
        #[arg(short, long)]
        gold: bool,
    },
    Day04,
    Day05,
    Day06,
    Day07 {
        #[arg(short, long)]
        gold: bool,
    },
    Day08 {
        #[arg(short, long)]
        gold: bool,
    },
    Day09 {
        #[arg(short, long)]
        gold: bool,
    },
    Day10,
    Day11,
    Day12,
    Day13 {
        #[arg(short, long)]
        gold: bool,
    },
    Day14,
    Day15 {
        #[arg(short, long)]
        gold: bool,
        #[arg(short, long)]
        verbose: bool,
    },
}
