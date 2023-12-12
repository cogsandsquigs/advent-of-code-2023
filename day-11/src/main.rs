use advent_utils::{grid::Grid, macros::solution, point::Point};
use itertools::Itertools;

fn main() {
    part_1();
    part_2();
}

fn get_galaxies(grid: &Grid<char>) -> Vec<Point<usize>> {
    grid.into_iter()
        .filter(|(_, c)| **c == '#')
        .map(|(p, _)| p)
        .collect()
}

// Rows, cols
fn get_expanded_rows_cols(
    grid: &Grid<char>,
    galaxies: &[Point<usize>],
) -> (Vec<usize>, Vec<usize>) {
    let mut row_counts = vec![0; grid.height];
    let mut col_counts = vec![0; grid.width];

    for p in galaxies {
        row_counts[p.y] += 1;
        col_counts[p.x] += 1;
    }

    (
        row_counts
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == 0)
            .map(|(i, _)| i)
            .collect(),
        col_counts
            .iter()
            .enumerate()
            .filter(|(_, c)| **c == 0)
            .map(|(i, _)| i)
            .collect(),
    )
}

#[solution(part = 1)]
fn part_1(input: &str) -> usize {
    let grid = Grid::from_str(input);
    let galaxies = get_galaxies(&grid);
    let (expanded_rows, expanded_cols) = get_expanded_rows_cols(&grid, &galaxies);

    galaxies
        .iter()
        .map(|p| {
            let mut p = *p;

            p.x += expanded_cols.iter().filter(|c| **c < p.x).count();
            p.y += expanded_rows.iter().filter(|r| **r < p.y).count();

            p
        })
        .tuple_combinations()
        .fold(0, |acc, (a, b)| acc + a.manhattan_distance(&b))
}

#[solution(part = 2)]
fn part_2(input: &str) -> usize {
    let expansion_factor = 1000000;
    let grid = Grid::from_str(input);
    let galaxies = get_galaxies(&grid);
    let (expanded_rows, expanded_cols) = get_expanded_rows_cols(&grid, &galaxies);

    galaxies
        .iter()
        .map(|p| {
            let mut p = *p;

            p.x += expanded_cols.iter().filter(|c| **c < p.x).count() * (expansion_factor - 1);
            p.y += expanded_rows.iter().filter(|r| **r < p.y).count() * (expansion_factor - 1);

            p
        })
        .tuple_combinations()
        .fold(0, |acc, (a, b)| acc + a.manhattan_distance(&b))
}
