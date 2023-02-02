mod year_2022;

use std::env::args;
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
    match year_2022::day01::get_highest_calories_from_input_file(path) {
        Ok(cal) => println!("highest count of calories is {cal}"),
        Err(e) => println!("{e}"),
    }
}
