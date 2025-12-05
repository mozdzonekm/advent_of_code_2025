advent_of_code::solution!(5);

use std::ops::RangeInclusive;

use itertools::{Itertools, any};
use nom::{
    IResult, Parser,
    bytes::complete::tag,
    character::complete::{newline, u64},
    multi::{many0, separated_list0},
    sequence::separated_pair,
};

fn parse_input(input: &str) -> IResult<&str, (Vec<RangeInclusive<u64>>, Vec<u64>)> {
    let (input, ranages) =
        separated_list0(newline, separated_pair(u64, tag("-"), u64)).parse(input)?;
    let (input, _) = many0(newline).parse(input)?;
    let (input, ingredients) = separated_list0(newline, u64).parse(input)?;

    Ok((
        input,
        (
            ranages.iter().copied().map(|(x, y)| x..=y).collect_vec(),
            ingredients,
        ),
    ))
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum End {
    Low = 0,
    High = 1,
}

fn reduce(ranges: &[RangeInclusive<u64>]) -> Vec<RangeInclusive<u64>> {
    let sorted_ends = ranges
        .iter()
        .flat_map(|range| vec![(range.start(), End::Low), (range.end(), End::High)])
        .sorted_by(|(id_1, end_1), (id_2, end_2)| {
            id_1.cmp(id_2).then(end_1.cmp(end_2)) // sort by id, on equals End::High first
        })
        .collect_vec();
    let mut nested_lvl = 0;
    let mut start = 0;
    sorted_ends
        .iter()
        .filter_map(|(id, end)| match end {
            End::Low => {
                if nested_lvl == 0 {
                    start = **id
                };
                nested_lvl += 1;
                None
            }
            End::High => {
                nested_lvl -= 1;
                if nested_lvl == 0 {
                    Some(start..=**id)
                } else {
                    None
                }
            }
        })
        .collect_vec()
}

pub fn part_one(input: &str) -> Option<u64> {
    let (_, (ranges, ingredients)) = parse_input(input).unwrap();
    let reduced_ranges = reduce(&ranges);
    let result = ingredients
        .iter()
        .filter(|i| any(&reduced_ranges, |r| r.contains(i)))
        .count() as u64;
    Some(result)
}
pub fn part_two(input: &str) -> Option<u64> {
    let (_, (ranges, _)) = parse_input(input).unwrap();
    let result = reduce(&ranges)
        .iter()
        .map(|r| r.end() - r.start() + 1)
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
        assert_eq!(result, Some(14));
    }
}
