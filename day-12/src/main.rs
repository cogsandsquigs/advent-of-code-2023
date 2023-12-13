use advent_utils::macros::solution;
use rayon::prelude::*;

fn main() {
    part_1();
    part_2();
}

struct Springs {
    /// The bitflags where current working springs are. Note this is reversed from the input, as the indices
    /// are from left to right.
    springs: u128,

    /// The bitflags where we can insert working springs. Note this is reversed from the input, as the indices
    /// are from left to right.
    open: u128,

    /// The chunk sizes we have to work with.
    sizes: Vec<u128>,

    /// The number of springs we need to insert.
    needed_springs: u128,
}

fn parse_input_day1(input: &str) -> impl Iterator<Item = Springs> + '_ {
    input.lines().map(|line| {
        let mut iter = line.split_whitespace();
        let spring_str = iter.next().unwrap();
        let size_str = iter.next().unwrap();

        let mut springs = 0;
        let mut open = 0;

        for (i, c) in spring_str.chars().enumerate() {
            if c == '#' {
                springs |= 1 << i;
            } else if c == '?' {
                open |= 1 << i;
            }
        }

        let mut springs = Springs {
            springs,
            open,

            sizes: size_str.split(',').map(|s| s.parse().unwrap()).collect(),
            needed_springs: 0,
        };

        springs.needed_springs = springs.sizes.iter().sum::<u128>()
            - spring_str.chars().filter(|c| *c == '#').count() as u128;

        springs
    })
}

// See if it works
fn test_possible_layout(springs: u128, sizes: &[u128]) -> bool {
    let mut current_size_idx = 0;
    let mut current_run = 0;

    // Rough estimate of the number of bits we need to check.
    for i in 0..128 {
        if springs & (1 << i) != 0 {
            current_run += 1;
        } else if current_run != 0 {
            if sizes[current_size_idx] != current_run {
                return false;
            }

            current_run = 0;
            current_size_idx += 1;
        }
    }

    true
}

fn get_all_ways(
    Springs {
        springs,
        open,
        sizes,
        needed_springs,
    }: Springs,
) -> u128 {
    let mut ways = 0;
    let mut i: u128 = 0;

    // Only iterate over the number of possible combinations, no more, no less.
    for _ in 0..1_u128 << open.count_ones() {
        // Update the next pattern to try.
        // Stolen from https://stackoverflow.com/questions/52830817/how-is-it-possible-to-iterate-over-all-subsets-of-a-set-represented-by-bits
        // Safe to do this first because we need to update i before we exit the loop anyway. Also, the empty set will
        // occur on the last iteration.
        i = ((i | !open).wrapping_add(1)) & open;

        if i.count_ones() != needed_springs as u32 {
            continue;
        }

        if test_possible_layout(i | springs, &sizes) {
            ways += 1;
        }
    }

    ways
}

#[solution(part = 1)]
fn part_1(input: &str) -> u128 {
    parse_input_day1(input).par_bridge().map(get_all_ways).sum()
}

fn parse_input_day2(input: &str) -> impl Iterator<Item = Springs> + '_ {
    input.lines().map(|line| {
        let mut iter = line.split_whitespace();
        let spring_str = [iter.next().unwrap()].repeat(5).join("?");
        let size_str = [iter.next().unwrap()].repeat(5).join(",");

        let mut springs = 0;
        let mut open = 0;

        for (i, c) in spring_str.chars().enumerate() {
            if c == '#' {
                springs |= 1 << i;
            } else if c == '?' {
                open |= 1 << i;
            }
        }

        let mut springs = Springs {
            springs,
            open,

            sizes: size_str
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect(),
            needed_springs: 0,
        };

        springs.needed_springs = springs.sizes.iter().sum::<u128>()
            - spring_str.chars().filter(|c| *c == '#').count() as u128;

        springs
    })
}

#[solution(part = 2)]
fn part_2(input: &str) -> u128 {
    parse_input_day2(input).par_bridge().map(get_all_ways).sum()
}
