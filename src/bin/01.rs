use itertools::Itertools;

advent_of_code::solution!(1);

const DIAL_SIZE: i32 = 100;
const START_POS: i32 = 50;

fn parse_input(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| match line.chars().next().unwrap() {
            'L' => -line[1..].parse::<i32>().unwrap(),
            'R' => line[1..].parse::<i32>().unwrap(),
            _ => panic!("Bad first letter in line {line}. Expected 'L' or 'R'."),
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let rotations = parse_input(input);
    let mut position = START_POS;
    let result = rotations
        .iter()
        .map(|rot| {
            let clipped_rot = rot % DIAL_SIZE;
            position = (position + clipped_rot + DIAL_SIZE) % DIAL_SIZE;
            position
        })
        .filter(|&n| n == 0)
        .count() as u64;
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let rotations = parse_input(input);
    let mut position = START_POS;
    let result = rotations
        .iter()
        .map(|rot| {
            let clipped_rot = rot % DIAL_SIZE;
            let new_pos_raw = position + clipped_rot;
            let next_position = (new_pos_raw + DIAL_SIZE) % DIAL_SIZE;

            let rot_pass_zero = (rot.abs() / DIAL_SIZE) as u64;
            let ends_in_zero = if next_position == 0 { 1 } else { 0 };
            let last_pass_zero = if position != 0 && !(0..=DIAL_SIZE).contains(&new_pos_raw) {
                1
            } else {
                0
            };
            position = next_position;
            rot_pass_zero + last_pass_zero + ends_in_zero
        })
        .sum::<u64>();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
