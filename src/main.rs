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
    match solutions::year_2022::day03::get_badge_sum(&input) {
        Some(s) => println!("the sum is {}", s),
        None => println!("failed to calculate the sum"),
    }
}
