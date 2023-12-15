use advent_utils::macros::solution;

fn main() {
    part_1();
    part_2();
}

fn hash(s: &str) -> u64 {
    let mut current_value = 0;

    for b in s.bytes() {
        current_value += b as u64;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

#[solution(part = 1)]
fn part_1(input: &str) -> u64 {
    input.split(',').map(hash).sum()
}

#[solution(part = 2)]
fn part_2(input: &str) -> u64 {
    let mut boxes = [Vec<(&str, u64)>; 256];

    for s in input.split(',') {
        if s.contains('-') {
            let label = s.split('-').next().unwrap();
            let box_idx = hash(label);

            todo!()
        }
        // Must contain '='
        else {
            let mut iter = s.split('=');
            let label = iter.next().unwrap();
            let box_idx = hash(label);
            let lens_val = iter.next().unwrap().parse::<u64>().unwrap();

            todo!()
        }
    }

    todo!()
}
