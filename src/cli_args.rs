use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct CliArgs {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// solves a challenge for a given year, day and input
    Solve(Challenge),
    /// lists all challenges implemented
    List,
}

#[derive(Args)]
pub struct Challenge {
    /// the year of the challenge
    pub year: u16,
    /// the day of the challenge
    pub day: u8,
    /// input file provided by advent of code
    pub input_file: PathBuf,
}
