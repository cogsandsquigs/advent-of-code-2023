use advent_utils::macros::solution;
use itertools::Itertools;

fn main() {
    part_1();
    part_2();
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect()
}

/// The first array is the pattern for the 0th derivative (plain function), the second array is the pattern
/// for the 1st derivative, etc. Essentially this is taking the discrete taylor series of the patterns.
fn compute_taylor_patterns(line: Vec<i64>) -> Vec<Vec<i64>> {
    let mut patterns = vec![line.clone()];

    while !patterns[patterns.len() - 1].iter().all(|&x| x == 0) {
        let prev_pattern = &patterns[patterns.len() - 1];

        // Compute the differences of each adjacent pair
        let differences = prev_pattern
            .iter()
            .tuple_windows()
            .map(|(a, b)| b - a)
            .collect_vec();

        patterns.push(differences);
    }

    patterns
}

/// Get the next value in the pattern for each pattern, according to the taylor series. Returns list in order of derivatives.
fn compute_next_values(patterns: &[Vec<i64>]) -> Vec<i64> {
    let mut next_values = vec![];
    let mut prev_val = 0;

    for last_val in patterns.iter().rev().map(|x| x.last().unwrap()) {
        next_values.push(prev_val + last_val);
        prev_val += last_val;
    }

    next_values.reverse();

    next_values
}

#[solution(part = 1)]
fn part_1(input: &str) -> i64 {
    let lines = parse_input(input);

    let patterns = lines
        .iter()
        .map(|line| compute_taylor_patterns(line.clone()))
        .collect_vec();

    patterns.iter().map(|x| compute_next_values(x)[0]).sum()
}

/// Get the previous value before the first value in the pattern for each pattern, according to the taylor series.
/// Returns list in order of derivatives.    
fn compute_prev_values(patterns: &[Vec<i64>]) -> Vec<i64> {
    let mut prev_values: Vec<i64> = vec![];
    let mut prev_val = 0;

    for first_val in patterns.iter().rev().map(|x| x.first().unwrap()) {
        println!("{}, {}", prev_val, first_val);

        prev_values.push(first_val - prev_val);
        prev_val = first_val - prev_val;
    }

    prev_values.reverse();

    prev_values
}

#[solution(part = 2)]
fn part_2(input: &str) -> i64 {
    let lines = parse_input(input);

    let patterns = lines
        .iter()
        .map(|line| compute_taylor_patterns(line.clone()))
        .collect_vec();

    patterns.iter().map(|x| compute_prev_values(x)[0]).sum()
}
