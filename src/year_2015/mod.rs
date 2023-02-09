pub fn get_implemented() -> Vec<&'static str> {
    vec![]
}

pub fn solve(day: u8, _input: &str) {
    match day {
        1..=25 => {
            super::print_not_implemented(2015, day);
        }
        _ => println!("{} is not a valid day for challenges", day),
    }
}
