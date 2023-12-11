use advent_utils::{grid::Grid, macros::solution, point::Point};
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    part_1();
    part_2();
}

fn get_neighbors(point: Point<usize>, segment: char) -> Vec<Point<usize>> {
    let mut neighbors = Vec::new();
    match segment {
        '|' => {
            neighbors.push(Point::new(point.x, point.y.saturating_sub(1)));
            neighbors.push(Point::new(point.x, point.y + 1));
        }
        '-' => {
            neighbors.push(Point::new(point.x.saturating_sub(1), point.y));
            neighbors.push(Point::new(point.x + 1, point.y));
        }
        '7' => {
            neighbors.push(Point::new(point.x.saturating_sub(1), point.y));
            neighbors.push(Point::new(point.x, point.y + 1));
        }
        'L' => {
            neighbors.push(Point::new(point.x + 1, point.y));
            neighbors.push(Point::new(point.x, point.y.saturating_sub(1)));
        }
        'J' => {
            neighbors.push(Point::new(point.x.saturating_sub(1), point.y));
            neighbors.push(Point::new(point.x, point.y.saturating_sub(1)));
        }
        'F' => {
            neighbors.push(Point::new(point.x + 1, point.y));
            neighbors.push(Point::new(point.x, point.y + 1));
        }
        'S' => {
            neighbors.push(Point::new(point.x.saturating_sub(1), point.y));
            neighbors.push(Point::new(point.x + 1, point.y));
            neighbors.push(Point::new(point.x, point.y.saturating_sub(1)));
            neighbors.push(Point::new(point.x, point.y + 1));
        }
        _ => {}
    }

    neighbors
}

// Checks if the two points are connected by a segment starting at segment_point
fn check_is_connected(point: Point<usize>, segment_point: Point<usize>, segment: char) -> bool {
    let neighbors = get_neighbors(segment_point, segment);

    if segment == '.' {
        return false;
    }

    for neighbor in neighbors {
        if neighbor == point {
            return true;
        }
    }

    false
}

#[solution(part = 1)]
fn part_1(input: &str) -> u64 {
    let grid = Grid::from_str(input);
    let start = grid.clone().into_iter().find(|(_, c)| *c == 'S').unwrap().0;

    let mut previous = start;
    let mut current = start;
    let mut steps = 0;

    loop {
        let current_segment = grid.get(current);

        // First condition prevents loop from exiting right away
        if previous != start && *current_segment == 'S' {
            break;
        }

        for neighbor in get_neighbors(current, *current_segment) {
            let neighbor_segment = grid.get(neighbor);

            if neighbor != previous && check_is_connected(current, neighbor, *neighbor_segment) {
                previous = current;
                current = neighbor;
                steps += 1;
                break;
            }
        }
    }

    (steps as f64 / 2.0).ceil() as u64
}

#[solution(part = 2)]
fn part_2(input: &str) -> u64 {
    let grid = Grid::from_str(input);
    let start = grid.clone().into_iter().find(|(_, c)| *c == 'S').unwrap().0;

    let mut previous = start;
    let mut current = start;
    let mut loop_points = HashSet::new();

    loop_points.insert(start);

    loop {
        let current_segment = grid.get(current);

        // First condition prevents loop from exiting right away
        if previous != start && *current_segment == 'S' {
            break;
        }

        for neighbor in get_neighbors(current, *current_segment) {
            let neighbor_segment = grid.get(neighbor);

            if neighbor != previous && check_is_connected(current, neighbor, *neighbor_segment) {
                previous = current;
                current = neighbor;
                loop_points.insert(current);
                break;
            }
        }
    }

    let mut area = 0;

    for (y, line) in input.lines().enumerate() {
        let mut parity = false; // False is even/outside loop, true is odd/inside loop

        for (x_previous, (_, c, _)) in line.chars().tuple_windows().enumerate() {
            let previous = Point::new(x_previous, y);
            let current = Point::new(x_previous + 1, y);
            let next = Point::new(x_previous + 2, y);

            if !loop_points.contains(&previous)
                && loop_points.contains(&current)
                && !loop_points.contains(&next)
                || loop_points.contains(&previous)
                    && loop_points.contains(&current)
                    && !loop_points.contains(&next)
                || !loop_points.contains(&previous)
                    && loop_points.contains(&current)
                    && loop_points.contains(&next)
            {
                parity = !parity;
            }

            if parity && !loop_points.contains(&current) {
                area += 1;
                print!("#");
            } else if loop_points.contains(&current) {
                print!("*");
            } else {
                print!("{}", c);
            }
        }

        println!();
    }

    area
}
