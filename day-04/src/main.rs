use advent_utils::macros::solution;

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
            .fold(0_u128, |b, x| b | (1 << x));
        let numbers_have = split_pipe[1]
            .split_whitespace()
            .map(|s| s.trim().parse::<u32>().unwrap())
            .fold(0_u128, |b, x| b | (1 << x));
        let total_winning_numbers = (winning_set & numbers_have).count_ones();

        if total_winning_numbers > 0 {
            score_sum += 2_i32.pow(total_winning_numbers - 1);
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
            .fold(0_u128, |b, x| b | (1 << x));
        let numbers_have = split_pipe[1]
            .split_whitespace()
            .map(|s| s.trim().parse::<u32>().unwrap())
            .fold(0_u128, |b, x| b | (1 << x));
        let total_winning_numbers = (winning_set & numbers_have).count_ones();

        for j in 1..=total_winning_numbers as usize {
            if i + j >= card_multipliers.len() {
                break;
            }
            card_multipliers[i + j] += card_multipliers[i];
        }
    }

    card_multipliers.iter().sum()
}
