use advent_utils::macros::solution;

fn main() {
    part_1();
    part_2();
}

fn to_count(l: &str) -> u32 {
    l.split(':')
        .nth(1)
        .unwrap()
        .split('|')
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.trim().parse::<u32>().unwrap())
                .fold(0_u128, |b, x| b | (1 << x))
        })
        .fold(u128::MAX, |a, x| a & x)
        .count_ones()
}

#[solution(part = 1)]
fn part_1(input: &str) -> i32 {
    input
        .lines()
        .map(to_count)
        .fold(0, |a, x| if x > 0 { a + 2_i32.pow(x - 1) } else { a })
}

#[solution(part = 2)]
fn part_2(input: &str) -> i32 {
    let mut m = vec![1; input.lines().count()];

    input
        .lines()
        .map(|l| to_count(l) as usize)
        .enumerate()
        .for_each(|(i, v)| (1..=v).for_each(|j| m[i + j] += m[i]));

    m.iter().sum()
}
