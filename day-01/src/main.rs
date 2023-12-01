use advent_utils::macros::solution;
use regex::Regex;
use std::collections::HashMap;

fn main() {
    // part_1();
    part_2();
}

#[solution(day = "01", part = "1")]
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

const STR_DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[solution(day = "01", part = "2")]
fn part_2(input: &str) -> u32 {
    let str_digits_map = STR_DIGITS
        .iter()
        .enumerate()
        .map(|(i, s)| (s, (i + 1) as u32))
        .collect::<HashMap<_, _>>();

    let str_digits_regex = Regex::new(r"(one|two|three|four|five|six|seven|eight|nine)").unwrap();

    let mut sum = 0;

    for line in input.lines() {
        let mut str_digits = str_digits_regex.find_iter(line);
        let first_str_digit = str_digits.next();
        let last_str_digit = str_digits.last();

        let mut char_digits = line.chars().enumerate().filter(|(_, c)| c.is_numeric());
        let first_char_digit = char_digits.next();
        let last_char_digit = char_digits.last();

        println!(
            "{:?} {:?} {:?} {:?}",
            first_str_digit, last_str_digit, first_char_digit, last_char_digit
        );

        let first_digit = match (first_str_digit, first_char_digit) {
            (Some(x), Some(y)) => {
                if x.start() > y.0 {
                    str_digits_map[&x.as_str()]
                } else {
                    y.1.to_digit(10).unwrap()
                }
            }
        };

        // sum += first_digit * 10 + last_digit
    }

    sum
}
