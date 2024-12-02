mod day01;

pub fn get_implemented() -> Vec<&'static str> {
    vec![
        "Day 1: Historian Hysteria",
    ]
}

pub fn solve(day: u8, input: &str) {
    match day {
        1 => {
            match day01::get_list_difference(input) {
                Ok(x) => println!("part 1: {}", x),
                Err(e) => println!("{}", e),
            }
            match day01::get_list_similarity_score(input) {
                Ok(x) => println!("part 2: {}", x),
                Err(e) => println!("{}", e),
            }
        }
        2..25 => {
            super::print_not_implemented(2024, day);
        }
        _ => println!("{} is not a valid day for challenges", day),
    }
}
