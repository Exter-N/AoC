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
}
