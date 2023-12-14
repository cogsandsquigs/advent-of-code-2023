use std::cmp;

use advent_utils::{grid::Grid, macros::solution};

fn main() {
    part_1();
    part_2();
}

fn parse_input(input: &str) -> Vec<Grid<char>> {
    input.split("\n\n").map(Grid::from_str).collect()
}

fn check_row_symmetry(grid: &Grid<char>, row_idx_before_symmetric_split: usize) -> bool {
    for i in 0..cmp::min(
        row_idx_before_symmetric_split + 1,
        grid.height - row_idx_before_symmetric_split - 1,
    ) {
        let row_before = grid.get_row(row_idx_before_symmetric_split - i);
        let row_after = grid.get_row(row_idx_before_symmetric_split + i + 1);

        if row_before != row_after {
            return false;
        }
    }

    true
}

fn check_col_symmetry(grid: &Grid<char>, col_idx_before_symmetric_split: usize) -> bool {
    for i in 0..cmp::min(
        col_idx_before_symmetric_split + 1,
        grid.width - col_idx_before_symmetric_split - 1,
    ) {
        let col_before = grid.get_col(col_idx_before_symmetric_split - i);
        let col_after = grid.get_col(col_idx_before_symmetric_split + i + 1);

        if col_before != col_after {
            return false;
        }
    }

    true
}

fn get_summary(grid: &Grid<char>, skip_row: Option<usize>, skip_col: Option<usize>) -> usize {
    let mut summary = 0;

    for col_idx in 0..grid.width - 1 {
        if skip_col.map(|i| i == col_idx).unwrap_or(false) {
            continue;
        }

        if check_col_symmetry(grid, col_idx) {
            summary += col_idx + 1;
        }
    }

    for row_idx in 0..grid.height - 1 {
        if skip_row.map(|i| i == row_idx).unwrap_or(false) {
            continue;
        }

        if check_row_symmetry(grid, row_idx) {
            summary += (row_idx + 1) * 100;
        }
    }

    summary
}

#[solution(part = 1)]
fn part_1(input: &str) -> usize {
    parse_input(input)
        .iter()
        .map(|g| get_summary(g, None, None))
        .sum()
}

/// row, col
fn get_summary_row_col(grid: &Grid<char>) -> (Option<usize>, Option<usize>) {
    let mut row = None;
    let mut col = None;

    for col_idx in 0..grid.width - 1 {
        if check_col_symmetry(grid, col_idx) {
            col = Some(col_idx);
            break;
        }
    }

    for row_idx in 0..grid.height - 1 {
        if check_row_symmetry(grid, row_idx) {
            row = Some(row_idx);
            break;
        }
    }

    (row, col)
}

#[solution(part = 2)]
fn part_2(input: &str) -> usize {
    let mut summary_sum = 0;

    for grid in parse_input(input) {
        let (orig_row, orig_col) = get_summary_row_col(&grid);

        for (point, val) in (&grid).into_iter() {
            let new_val = if *val == '#' { '.' } else { '#' };

            // println!("{}", point);

            let mut new_grid = grid.clone();
            new_grid[point] = new_val;

            let summary = get_summary(&new_grid, orig_row, orig_col);

            if summary != 0 {
                summary_sum += summary;
                break;
            }
        }
    }

    summary_sum
}
