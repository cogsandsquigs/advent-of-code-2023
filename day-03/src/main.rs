use advent_utils::{grid::Grid, macros::solution, point::Point};
use std::{collections::HashSet, iter::FromIterator};

fn main() {
    part_1();
    part_2();
}

fn is_symbol(c: char) -> bool {
    !c.is_alphanumeric() && c != '.'
}

fn get_numbers_from_pointset(num_grid: &Grid<char>, pointset: &HashSet<&Point<usize>>) -> Vec<u32> {
    let mut nums = Vec::new();
    let mut avoids = HashSet::new();

    for point in pointset {
        let c = num_grid.get(**point);
        let mut numstr = c.to_string();

        // We know it's a number, so we have to check left + right to make sure it's not a double/triple digit number or something

        // First to the right, so add digits to the end of the string
        let mut i = 1;
        while num_grid.get(Point::new(point.x + i, point.y)) != &'.'
            && !is_symbol(*num_grid.get(Point::new(point.x + i, point.y)))
        {
            let p = Point::new(point.x + i, point.y);
            if avoids.contains(&p) {
                break;
            }

            let c = num_grid.get(p);

            // remove p from pointset to avoid double counting
            avoids.insert(p);

            numstr.push(*c);

            i += 1;
        }

        // Then to the left, so add digits to the beginning of the string
        let mut i = 1;
        while i <= point.x
            && (num_grid.get(Point::new(point.x - i, point.y)) != &'.'
                && !is_symbol(*num_grid.get(Point::new(point.x - i, point.y))))
        {
            let p = Point::new(point.x - i, point.y);
            if avoids.contains(&p) {
                break;
            }
            let c = num_grid.get(p);

            // remove p from pointset to avoid double counting
            avoids.insert(p);

            numstr.insert(0, *c);

            i += 1;
        }

        // Now we have the full number, so we can parse it
        let num = numstr.parse::<u32>().unwrap();

        // Add the number to the vector
        nums.push(num);

        avoids.insert(**point);
    }

    nums
}

#[solution(day = "03", part = "1")]
fn part_1(input: &str) -> u32 {
    let grid = Grid::from_str(input);

    let numbers_points: HashSet<_> =
        HashSet::from_iter(Grid::from_str_precomp(input, char::is_numeric).true_points());

    let symbols_neighbor_points: HashSet<_> = HashSet::from_iter(
        Grid::from_str_precomp(input, is_symbol)
            .true_points()
            .flat_map(|p| p.neighbors()),
    );

    // AND the sets together to get the intersection
    let intersections: HashSet<_> = numbers_points
        .intersection(&symbols_neighbor_points)
        .collect();

    let nums = get_numbers_from_pointset(&grid, &intersections);

    println!("{:?}", nums);

    nums.iter().sum()
}

#[solution(day = "03", part = "2")]
fn part_2(input: &str) -> u32 {
    0
}
