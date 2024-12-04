mod day01;
mod day02;
mod day03;

pub fn get_implemented() -> Vec<&'static str> {
    vec![
        "Day 1: Historian Hysteria",
        "Day 2: Red-Nosed Reports",
        "Day 3: Mull It Over",
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
        2 => {
            match day02::count_safe_reports(input) {
                Ok(x) => println!("part 1: {}", x),
                Err(e) => println!("{}", e),
            }
            match day02::count_safe_reports_dampened(input) {
                Ok(x) => println!("part 2: {}", x),
                Err(e) => println!("{}", e),
            }
        }
        3 => {
            match day03::parse_and_execute_multiplication(input) {
                Some(x) => println!("part 1: {}", x),
                None => println!("unabale to parse string and execute multiplications"),
            }
            match day03::parse_and_execute_multiplication_with_conditionals(input) {
                Some(x) => println!("part 2: {}", x),
                None => println!("unabale to parse string and execute multiplications"),
            }
        }
        4..25 => {
            super::print_not_implemented(2024, day);
        }
        _ => println!("{} is not a valid day for challenges", day),
    }
}
