use itertools::Itertools;

advent_of_code::solution!(3);

fn max_voltage(line: &str, n_batteries: usize) -> u64 {
    let battery_bank = line.chars().collect_vec();
    let mut search_slice = 0..battery_bank.len() - n_batteries + 1;
    (1..n_batteries + 1)
        .map(|battery_idx| {
            let (idx, max_value) = battery_bank
                .iter()
                .enumerate()
                .filter(|(i, _)| search_slice.contains(i))
                .max_by(|(i, x), (j, y)| x.cmp(y).then(i.cmp(j).reverse()))
                .unwrap();

            search_slice = idx + 1..battery_bank.len() - (n_batteries - battery_idx) + 1;
            max_value
        })
        .collect::<String>()
        .parse::<u64>()
        .unwrap()
}

pub fn part_one(input: &str) -> Option<u64> {
    let result = input.lines().map(|line| max_voltage(line, 2)).sum::<u64>();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let result = input.lines().map(|line| max_voltage(line, 12)).sum::<u64>();
    Some(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("987654321111111", 2, 98)]
    #[case("811111111111119", 2, 89)]
    #[case("234234234234278", 2, 78)]
    #[case("818181911112111", 2, 92)]
    #[case("987654321111111", 12, 987654321111)]
    #[case("811111111111119", 12, 811111111119)]
    #[case("234234234234278", 12, 434234234278)]
    #[case("818181911112111", 12, 888911112111)]
    fn test_max_voltage(#[case] line: &str, #[case] n_batteries: usize, #[case] expected: u64) {
        assert_eq!(max_voltage(line, n_batteries), expected);
    }

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}
