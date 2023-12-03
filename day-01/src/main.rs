use advent_utils::macros::solution;

fn main() {
    part_1();
    part_2();
}

#[solution(day = 1, part = 1)]
fn part_1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let iter = line.chars().filter(|c| c.is_numeric());
            let first_char = iter.clone().next().unwrap();
            let last_char = iter.last().unwrap();

            10 * first_char.to_digit(10).unwrap() + last_char.to_digit(10).unwrap()
        })
        .sum()
}

const STR_DIGITS: [(&str, &str); 9] = [
    ("one", "o1e"),
    ("two", "t2o"),
    ("three", "t3e"),
    ("four", "f4r"),
    ("five", "f5e"),
    ("six", "s6x"),
    ("seven", "s7n"),
    ("eight", "e8t"),
    ("nine", "n9e"),
];

#[solution(day = 1, part = 2)]
fn part_2(input: &str) -> u32 {
    let mut new_input: String = input.into();

    // pre-process input to remove all non-numeric string digits + replace with actual digits
    for (from, to) in STR_DIGITS {
        new_input = new_input.replace(from, to);
    }

    new_input
        .lines()
        .map(|line| {
            let iter = line.chars().filter(|c| c.is_numeric());
            let first_char = iter.clone().next().unwrap();
            let last_char = iter.last().unwrap();

            10 * first_char.to_digit(10).unwrap() + last_char.to_digit(10).unwrap()
        })
        .sum()
}
