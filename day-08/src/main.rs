use std::collections::HashMap;

use advent_utils::macros::solution;

fn main() {
    part_1();
    part_2();
}

fn parse_input(input: &str) -> (HashMap<&str, (&str, &str)>, Vec<char>) {
    let mut map = HashMap::new();
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();

    lines.next();

    for line in lines {
        let node = &line[0..3];
        let left = &line[7..10];
        let right = &line[12..15];

        map.insert(node, (left, right));
    }

    (map, instructions)
}

#[solution(part = 1)]
fn part_1(input: &str) -> usize {
    let (map, instructions) = parse_input(input);

    let mut steps = 0;
    let mut current_node = "AAA";

    while current_node != "ZZZ" {
        let (left, right) = map.get(current_node).unwrap();

        if instructions[steps % instructions.len()] == 'L' {
            current_node = left;
        } else {
            current_node = right;
        }

        steps += 1;
    }

    steps
}

#[solution(part = 2)]
fn part_2(input: &str) -> usize {
    let (map, instructions) = parse_input(input);

    let mut current_nodes = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .copied()
        .collect::<Vec<_>>();
    let mut steps = 0;

    while !current_nodes.iter().all(|n| n.ends_with('Z')) {
        for i in 0..current_nodes.len() {
            let node = current_nodes[i];

            let (left, right) = map.get(node).unwrap();

            if instructions[steps % instructions.len()] == 'L' {
                current_nodes[i] = left;
            } else {
                current_nodes[i] = right;
            }
        }

        steps += 1;
    }

    steps
}
