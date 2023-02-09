mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

pub fn get_implemented() -> Vec<&'static str> {
    vec![
        "Day 1: Calorie Counting",
        "Day 2: Rock Paper Scissors",
        "Day 3: Rucksack Reorganization",
        "Day 4: Camp Cleanup",
    ]
}

pub fn solve(day: u8, input: &str) {
    match day {
        1 => {
            match day01::get_highest_calories(input) {
                Ok(x) => println!("part 1: {}", x),
                Err(e) => println!("{}", e),
            }
            match day01::get_top_n_calorie_sum(input, 3) {
                Ok(x) => println!("part 2: {}", x),
                Err(e) => println!("{}", e),
            }
        }
        2 => {
            match day02::get_rock_paper_scissors_score1(input) {
                Ok(x) => println!("part 1: {}", x.right),
                Err(e) => println!("{}", e),
            }
            match day02::get_rock_paper_scissors_score2(input) {
                Ok(x) => println!("part 1: {}", x.right),
                Err(e) => println!("{}", e),
            }
        }
        3 => {
            match day03::get_priority_sum(input) {
                Some(x) => println!("part 1: {}", x),
                None => println!("error"),
            }
            match day03::get_badge_sum(input) {
                Some(x) => println!("part 2: {}", x),
                None => println!("error"),
            }
        }
        4 => {
            match day04::get_contained_pair_count(input) {
                Some(x) => println!("part 1: {}", x),
                None => println!("error"),
            }
            match day04::get_overlapping_pair_count(input) {
                Some(x) => println!("part 2: {}", x),
                None => println!("error"),
            }
        }
        5 => {
            match day05::get_top_crates_one_at_a_time(input) {
                Ok(x) => println!("part 1: {}", x),
                Err(e) => println!("{}", e),
            }
            match day05::get_top_crates_multiple_at_a_time(input) {
                Ok(x) => println!("part 1: {}", x),
                Err(e) => println!("{}", e),
            }
        }
        6..=25 => {
            super::print_not_implemented(2022, day);
        }
        _ => println!("{} is not a valid day for challenges", day),
    }
}
