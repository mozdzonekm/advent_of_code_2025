use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect_vec()).collect_vec()
}

fn padded_grid(grid: &[Vec<char>]) -> Vec<Vec<char>> {
    (0..grid.len() + 2)
        .map(|y| {
            (0..grid[0].len() + 2)
                .map(|x| {
                    if y > 0
                        && x > 0
                        && y < grid.len() + 1
                        && x < grid[0].len() + 1
                        && grid[y - 1][x - 1] == '@'
                    {
                        '@'
                    } else {
                        '.'
                    }
                })
                .collect_vec()
        })
        .collect_vec()
}

fn padded_safe_neighborhood(y: usize, x: usize) -> Vec<(usize, usize)> {
    let transformations = vec![
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    transformations
        .into_iter()
        .map(|(dy, dx)| ((y as i32 + dy) as usize, (x as i32 + dx) as usize))
        .collect_vec()
}

fn anneling(
    grid: &mut [Vec<char>],
    max_neighbours: usize,
    removed_last_time: &[(usize, usize)],
) -> Vec<(usize, usize)> {
    let positions_to_check: HashSet<(usize, usize)> = if !removed_last_time.is_empty() {
        removed_last_time
            .iter()
            .flat_map(|(y, x)| {
                padded_safe_neighborhood(*y, *x)
                    .iter()
                    .filter(|(ny, nx)| grid[*ny][*nx] == '@')
                    .copied()
                    .collect_vec()
            })
            .collect()
    } else {
        (0..grid.len())
            .cartesian_product(0..grid[0].len())
            .filter(|(y, x)| grid[*y][*x] == '@')
            .collect()
    };
    let removed_positions = positions_to_check
        .iter()
        .filter(|(y, x)| {
            padded_safe_neighborhood(*y, *x)
                .iter()
                .filter(|(ny, nx)| grid[*ny][*nx] == '@')
                .count()
                <= max_neighbours
        })
        .copied()
        .collect_vec();
    for (y, x) in removed_positions.iter() {
        grid[*y][*x] = '.';
    }
    removed_positions
}

fn count_active(grid: &[Vec<char>]) -> u64 {
    grid.iter().flatten().filter(|c| **c == '@').count() as u64
}

pub fn part_one(input: &str) -> Option<u64> {
    let max_neighbours = 3;
    let mut grid = padded_grid(&parse_input(input));
    let before = count_active(&grid);
    let removed_last_time: Vec<(usize, usize)> = vec![];
    anneling(&mut grid, max_neighbours, &removed_last_time);
    let after = count_active(&grid);
    Some((before - after) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let max_neighbours = 3;
    let mut grid = padded_grid(&parse_input(input));
    let before = count_active(&grid);
    let mut removed_last_time: Vec<(usize, usize)> = vec![];
    removed_last_time = anneling(&mut grid, max_neighbours, &removed_last_time);
    while !removed_last_time.is_empty() {
        removed_last_time = anneling(&mut grid, max_neighbours, &removed_last_time);
    }
    let after = count_active(&grid);
    Some((before - after) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
