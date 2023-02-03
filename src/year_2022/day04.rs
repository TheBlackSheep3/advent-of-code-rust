use std::ops::RangeInclusive;

pub fn get_contained_pair_count(input: &str) -> Option<i32> {
    let mut sum = 0;
    for line in input.lines() {
        let (first, second) = parse_ranges(line)?;
        if is_range_contained(first, second) {
            sum += 1;
        }
    }
    Some(sum)
}

pub fn get_overlapping_pair_count(input: &str) -> Option<i32> {
    let mut sum = 0;
    for line in input.lines() {
        let (first, second) = parse_ranges(line)?;
        if is_range_overlapping(first, second) {
            sum += 1;
        }
    }
    Some(sum)
}

fn parse_ranges(line: &str) -> Option<(RangeInclusive<i32>, RangeInclusive<i32>)> {
    let line: &str = line.trim();
    let mut ranges: Vec<RangeInclusive<i32>> = Vec::new();
    for range in line.split(',') {
        let mut tmp: Vec<i32> = Vec::new();
        for entry in range.split('-') {
            tmp.push((*entry).parse::<i32>().ok()?)
        }
        ranges.push(RangeInclusive::new(tmp[0], tmp[1]));
    }
    Some((ranges[0].clone(), ranges[1].clone()))
}

fn is_range_contained(first: RangeInclusive<i32>, second: RangeInclusive<i32>) -> bool {
    if first.contains(second.start()) && first.contains(second.end()) {
        true
    } else if second.contains(first.start()) && second.contains(first.end()) {
        true
    } else {
        false
    }
}

fn is_range_overlapping(first: RangeInclusive<i32>, second: RangeInclusive<i32>) -> bool {
    if first.contains(second.start()) || first.contains(second.end()) {
        true
    } else if second.contains(first.start()) || second.contains(first.end()) {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn contained_count() {
        assert_eq!(get_contained_pair_count(TEST_INPUT), Some(2));
    }

    #[test]
    fn overlapping_count() {
        assert_eq!(get_overlapping_pair_count(TEST_INPUT), Some(4));
    }

    #[test]
    fn overlapping() {
        assert!(is_range_overlapping(1..=6, 3..=6));
        assert!(is_range_overlapping(1..=5, 3..=6));
        assert!(!is_range_overlapping(1..=2, 3..=6));
        assert!(is_range_overlapping(3..=6, 1..=6));
    }

    #[test]
    fn contained() {
        assert!(is_range_contained(1..=6, 3..=6));
        assert!(!is_range_contained(1..=5, 3..=6));
        assert!(!is_range_contained(1..=2, 3..=6));
        assert!(is_range_contained(3..=6, 1..=6));
    }
}
