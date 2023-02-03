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
    match year_2022::day02::get_rock_paper_scissors_score(&input) {
        Ok(score) => println!("my score is {}", score.right),
        Err(e) => println!("{e}"),
    }
}
