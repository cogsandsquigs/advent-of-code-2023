use advent_utils::macros::solution;
use std::collections::HashSet;

fn main() {
    part_1();
    part_2();
}

#[solution(part = 1)]
fn part_1(input: &str) -> i32 {
    let mut score_sum = 0;

    for line in input.lines() {
        let nums = &line.split(':').collect::<Vec<_>>()[1..];
        let split_pipe = nums[0].split('|').collect::<Vec<_>>();
        let winning_set = split_pipe[0]
            .split_whitespace()
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let numbers_have = split_pipe[1]
            .split_whitespace()
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let total_winning_numbers = winning_set.intersection(&numbers_have).count();

        if total_winning_numbers > 0 {
            score_sum += 2_i32.pow(total_winning_numbers as u32 - 1);
        }
    }

    score_sum
}

#[solution(part = 2)]
fn part_2(input: &str) -> i32 {
    let mut card_multipliers = vec![1; input.lines().count()];

    for (i, line) in input.lines().enumerate() {
        let nums = &line.split(':').collect::<Vec<_>>()[1..];
        let split_pipe = nums[0].split('|').collect::<Vec<_>>();
        let winning_set = split_pipe[0]
            .split_whitespace()
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect::<HashSet<_>>();
        let numbers_have = split_pipe[1]
            .split_whitespace()
            .map(|s| s.trim().parse::<u32>().unwrap())
            .collect::<HashSet<_>>();

        let total_winning_numbers = winning_set.intersection(&numbers_have).count();

        for j in 1..=total_winning_numbers {
            if i + j >= card_multipliers.len() {
                break;
            }
            card_multipliers[i + j] += card_multipliers[i];
        }
    }

    card_multipliers.iter().sum()
}
