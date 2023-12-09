use advent_utils::macros::solution;
use num::Integer;
use std::collections::HashMap;

fn main() {
    part_1();
    part_2();
}

fn parse_input(input: &str) -> (HashMap<&str, (&str, &str)>, Vec<char>) {
    let mut map = HashMap::new();
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().collect::<Vec<_>>();

    lines.next(); // Skip next (blank) line

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

    let current_nodes = map
        .keys()
        .filter(|&&node| node.ends_with('A'))
        .collect::<Vec<_>>();
    let mut node_steps = vec![0; current_nodes.len()];

    for i in 0..current_nodes.len() {
        let mut node = current_nodes[i];

        while !node.ends_with('Z') {
            let (left, right) = map.get(node).unwrap();

            if instructions[node_steps[i] % instructions.len()] == 'L' {
                node = left;
            } else {
                node = right;
            }

            node_steps[i] += 1;
        }
    }

    node_steps.iter().copied().reduce(|a, b| a.lcm(&b)).unwrap()
}
