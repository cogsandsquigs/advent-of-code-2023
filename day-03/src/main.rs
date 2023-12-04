use advent_utils::{grid::Grid, macros::solution, point::Point};
use std::collections::HashSet;

fn main() {
    part_1();
    part_2();
}

fn is_symbol(c: char) -> bool {
    !c.is_alphanumeric() && c != '.'
}

#[allow(clippy::type_complexity)]
fn get_nums_and_symbol_edges(
    input: &str,
) -> (Vec<(u32, Vec<Point<usize>>)>, HashSet<Point<usize>>) {
    let grid = Grid::from_str(input);
    let mut nums: Vec<(u32, Vec<Point<usize>>)> = vec![];
    let mut symbol_edges: HashSet<Point<usize>> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        let mut current_num = 0;
        let mut current_num_points = vec![];
        let mut was_num = false;

        for (x, c) in line.chars().enumerate() {
            // Just in case there's no symbol before the end of the line
            if was_num && !c.is_numeric() {
                nums.push((current_num, current_num_points.clone()));
                current_num = 0;
                current_num_points = vec![];
                was_num = false;
            }

            if is_symbol(c) {
                grid.neighbors(Point::new(x, y)).iter().for_each(|p| {
                    symbol_edges.insert(*p);
                });
            } else if c.is_numeric() {
                was_num = true;
                current_num_points.push(Point::new(x, y));
                current_num = current_num * 10 + c.to_digit(10).unwrap();
            }
        }

        // Just in case the number is at the end of the line
        if was_num {
            nums.push((current_num, current_num_points.clone()));
        }
    }

    (nums, symbol_edges)
}

#[solution(part = 1)]
fn part_1(input: &str) -> u32 {
    let (nums, symbol_edges) = get_nums_and_symbol_edges(input);

    nums.iter()
        .filter(|(_, ps)| ps.iter().any(|p| symbol_edges.contains(p)))
        .map(|(n, _)| n)
        .sum()
}

fn get_gears(input: &str) -> Vec<Point<usize>> {
    let mut gears: Vec<Point<usize>> = vec![];

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if is_symbol(c) {
                gears.push(Point::new(x, y));
            }
        }
    }

    gears
}

#[solution(part = 2)]
fn part_2(input: &str) -> u32 {
    let (nums, _) = get_nums_and_symbol_edges(input);
    let gears = get_gears(input);

    let mut gear_ratio_sum = 0;

    for gear in gears {
        let neighbors = gear.neighbors();

        let adjacent_nums = nums
            .iter()
            .filter(|(_, ps)| ps.iter().any(|p| neighbors.contains(p)))
            .map(|(n, _)| n)
            .collect::<Vec<_>>();

        if adjacent_nums.len() == 2 {
            gear_ratio_sum += adjacent_nums[0] * adjacent_nums[1];
        }
    }

    gear_ratio_sum
}
