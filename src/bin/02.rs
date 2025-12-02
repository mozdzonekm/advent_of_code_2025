advent_of_code::solution!(2);

use std::collections::HashSet;

use itertools::Itertools;

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    input
        .trim()
        .split(",")
        .map(|s| {
            let (low, high) = s.split_once("-").unwrap();
            (low.parse::<u64>().unwrap(), high.parse::<u64>().unwrap())
        })
        .collect_vec()
}

fn take_n_digits(num: &u64, n: u32) -> u64 {
    let len = digits(num);
    num / (10_u64).pow(len - n)
}

fn digits(num: &u64) -> u32 {
    num.ilog10() + 1
}

fn repeated(pattern: u64, n: u32) -> u64 {
    let exp = (10_u64).pow(digits(&pattern));
    let mut num = pattern;
    for _ in 2..=n {
        num = num * exp + pattern;
    }
    num
}

fn invalid_ids(low: &u64, high: &u64, repetition: u32) -> Vec<u64> {
    let pattern_min = take_n_digits(low, digits(low) / repetition);
    let pattern_max = take_n_digits(high, digits(high).div_ceil(repetition));
    (pattern_min..=pattern_max)
        .filter(|pattern| *pattern > 0)
        .map(|pattern| repeated(pattern, repetition))
        .filter(|id| (low..=high).contains(&id))
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let ranges = parse_input(input);
    let result = ranges
        .iter()
        .flat_map(|(low, high)| invalid_ids(low, high, 2))
        .sum::<u64>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let ranges = parse_input(input);
    let result = ranges
        .iter()
        .flat_map(|(low, high)| {
            (2..=digits(high)).flat_map(|repetition| invalid_ids(low, high, repetition))
        })
        .collect::<HashSet<u64>>()
        .iter()
        .sum::<u64>();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeated() {
        assert_eq!(repeated(10, 2), 1010);
        assert_eq!(repeated(12, 2), 1212);
        assert_eq!(repeated(1202, 3), 120212021202);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}
