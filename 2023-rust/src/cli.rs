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
    Day02,
    Day03,
    Day04 {
        #[arg(short, long)]
        gold: bool,
    },
    Day05 {
        #[arg(short, long)]
        gold: bool,
    },
    Day06 {
        #[arg(short, long)]
        gold: bool,
    },
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
    Day12 {
        #[arg(short, long)]
        gold: bool,
    },
    Day13 {
        #[arg(short, long)]
        gold: bool,
    },
    Day14 {
        #[arg(short, long)]
        gold: bool,
    },
    Day15,
    Day16 {
        #[arg(short, long)]
        gold: bool,
    },
    Day17 {
        #[arg(short, long)]
        gold: bool,
    },
    Day18 {
        #[arg(short, long)]
        gold: bool,
    },
    Day19 {
        #[arg(short, long)]
        gold: bool,
    },
    Day20 {
        #[arg(short, long)]
        gold: bool,
        #[arg(short = 'x', long)]
        export: bool,
    },
    Day21 {
        #[arg(short, long)]
        gold: bool,
        #[arg(short, long)]
        steps: usize,
    },
}
