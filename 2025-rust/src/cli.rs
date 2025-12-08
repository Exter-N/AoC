use clap::{Parser, Subcommand};

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
        gold: bool,
        #[arg(short, long)]
        verbose: bool,
    },
    Day02 {
        #[arg(short, long)]
        gold: bool,
    },
    Day03 {
        #[arg(short, long)]
        gold: bool,
    },
    Day04 {
        #[arg(short, long)]
        verbose: bool,
    },
    Day05,
    Day06 {
        #[arg(short, long)]
        gold: bool,
    },
    Day07,
    Day08 {
        #[arg(short, long)]
        gold: bool,
    },
    Day09,
    Day10,
    Day11,
    Day12,
}
