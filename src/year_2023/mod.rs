mod day01;

pub fn get_implemented() -> Vec<&'static str> {
    vec![
        "Day 1: Trebuchet?!",
    ]
}

pub fn solve(day: u8, input: &str) {
    match day {
        1 => {
            match day01::get_calibration_values_sum(input) {
                Ok(x) => println!("part 1: {}", x),
                Err(e) => println!("{}", e),
            }
        }
        2..=25 => {
            super::print_not_implemented(2022, day);
        }
        _ => println!("{} is not a valid day for challenges", day),
    }
}
