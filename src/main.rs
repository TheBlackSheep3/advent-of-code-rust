use std::fs;

mod cli_args;

use clap::Parser;
use cli_args::{CliArgs, Commands};

fn main() {
    let args: CliArgs = CliArgs::parse();
    match args.command {
        Commands::List => solutions::print_implemented(),
        Commands::Solve(challenge) => {
            let path = challenge.input_file.as_path();
            let input = match fs::read_to_string(path) {
                Ok(text) => text,
                Err(e) => {
                    println!("Error reading {}: {e}", path.display());
                    return;
                }
            };
            match challenge.year {
                2015 => solutions::year_2015::solve(challenge.day, &input),
                2016 => solutions::year_2016::solve(challenge.day, &input),
                2017 => solutions::year_2017::solve(challenge.day, &input),
                2018 => solutions::year_2018::solve(challenge.day, &input),
                2019 => solutions::year_2019::solve(challenge.day, &input),
                2020 => solutions::year_2020::solve(challenge.day, &input),
                2021 => solutions::year_2021::solve(challenge.day, &input),
                2022 => solutions::year_2022::solve(challenge.day, &input),
                _ => println!("no challenge available for {}", challenge.year),
            }
        }
    }
    //let path: &Path = args.input_file.as_path();
    //match solutions::year_2022::day04::get_overlapping_pair_count(&input) {
    //    Some(s) => println!("the sum is {}", s),
    //    None => println!("failed to calculate the sum"),
    //}
}
