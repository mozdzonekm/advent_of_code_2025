use itertools::Itertools;

advent_of_code::solution!(4);

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect_vec()).collect_vec()
}

fn neighborhood(grid: &[Vec<char>], y: usize, x: usize) -> Vec<(usize, usize)> {
    let size_y = grid.len();
    let size_x = grid[0].len();
    (-1..=1)
        .cartesian_product(-1..=1)
        .filter(|(dy, dx)| !(*dy == 0 && *dx == 0))
        .map(|(dy, dx)| (y as i32 + dy, x as i32 + dx))
        .filter(|(ny, nx)| *ny >= 0 && *ny < size_y as i32 && *nx >= 0 && *nx < size_x as i32)
        .map(|(ny, nx)| (ny as usize, nx as usize))
        .collect_vec()
}

fn build_active_neighbours(grid: &[Vec<char>]) -> Vec<Vec<Vec<(usize, usize)>>> {
    grid.iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, _)| {
                    neighborhood(grid, y, x)
                        .iter()
                        .filter(|(ny, nx)| grid[*ny][*nx] == '@')
                        .copied()
                        .collect_vec()
                })
                .collect_vec()
        })
        .collect_vec()
}

fn build_active_positions(grid: &[Vec<char>]) -> Vec<(usize, usize)> {
    grid.iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, c)| **c == '@')
                .map(move |(x, _)| (y, x))
        })
        .collect()
}

fn anneling(
    grid: &mut [Vec<char>],
    positions: Vec<(usize, usize)>,
    neighbours: &[Vec<Vec<(usize, usize)>>],
    max_neighbours: usize,
) -> Vec<(usize, usize)> {
    let mut positions_to_clear: Vec<(usize, usize)> = vec![];
    let new_active_positions = positions
        .iter()
        .filter(|(y, x)| {
            let to_remove = neighbours[*y][*x]
                .iter()
                .filter(|(ny, nx)| grid[*ny][*nx] == '@')
                .count()
                <= max_neighbours;
            if to_remove {
                positions_to_clear.push((*y, *x));
            }
            !to_remove
        })
        .copied()
        .collect_vec();
    for (y, x) in positions_to_clear {
        grid[y][x] = '.';
    }
    new_active_positions
}

pub fn part_one(input: &str) -> Option<u64> {
    let max_neighbours = 3;
    let mut grid = parse_input(input);
    let mut active_positions = build_active_positions(&grid);
    let neighbours = build_active_neighbours(&grid);
    let before = active_positions.len();
    active_positions = anneling(&mut grid, active_positions, &neighbours, max_neighbours);
    let after = active_positions.len();
    Some((before - after) as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let max_neighbours = 3;
    let mut grid = parse_input(input);
    let mut active_positions = build_active_positions(&grid);
    let neighbours = build_active_neighbours(&grid);
    let before = active_positions.len();
    let mut last = before;
    active_positions = anneling(&mut grid, active_positions, &neighbours, max_neighbours);
    while active_positions.len() < last {
        last = active_positions.len();
        active_positions = anneling(&mut grid, active_positions, &neighbours, max_neighbours);
    }
    Some((before - active_positions.len()) as u64)
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
