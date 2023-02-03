mod year_2022;

use std::env::args;
use std::fs;
use std::path::Path;

fn main() {
    let path_str: String = match args().nth(1) {
        Some(string) => string,
        None => {
            println!("missing path argument");
            return;
        }
    };
    let path: &Path = Path::new(&path_str);
    let input = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(e) => {
            println!("Error reading {}: {e}", path.display());
            return;
        }
    };
    match year_2022::day01::get_highest_calories(&input) {
        Ok(cal) => println!("highest count of calories is {cal}"),
        Err(e) => println!("{e}"),
    }
    match year_2022::day01::get_top_n_calorie_sum(&input, 3) {
        Ok(cal) => println!("the sum of the top 3 calories is {cal}"),
        Err(e) => println!("{e}"),
    }
}
