use std::collections::HashMap;

use advent_utils::macros::solution;

fn main() {
    // part_1();
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

    let current_nodes = map
        .keys()
        .filter(|k| k.ends_with('A'))
        .copied()
        .collect::<Vec<_>>();

    // (steps before cycle, steps in cycle (start of cycle to ends with z))
    let mut node_paths = vec![];
    // let mut node_steps_before_cycle = vec![];
    // let mut node_steps_in_cycle = vec![];

    for mut node in current_nodes.iter() {
        let mut node_path = vec![*node];
        let mut steps = 0;

        while !node.ends_with('Z') {
            let (left, right) = map.get(node).unwrap();

            if instructions[steps % instructions.len()] == 'L' {
                node = left;
            } else {
                node = right;
            }

            steps += 1;
            node_path.push(*node);
        }

        let (last_left, last_right) = map.get(node).unwrap();
        let cycle_start = if instructions[steps % instructions.len()] == 'L' {
            last_left
        } else {
            last_right
        };

        let cycle_steps = node_path
            .iter()
            .rev()
            .position(|n| n == cycle_start)
            .unwrap();

        node_paths.push((steps - cycle_steps, cycle_steps));
        // node_steps_before_cycle.push(steps - cycle_steps);
        // node_steps_in_cycle.push(cycle_steps);
    }

    // for (x, y) in node_paths.iter() {
    //     print!("x = {} mod {}, ", x, y);
    // }
    // println!();

    todo!()
}
