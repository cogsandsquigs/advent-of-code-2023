use advent_utils::macros::solution;

fn main() {
    part_1();
    part_2();
}

fn hash(s: &str) -> usize {
    let mut current_value = 0;

    for b in s.bytes() {
        current_value += b as usize;
        current_value *= 17;
        current_value %= 256;
    }

    current_value
}

#[solution(part = 1)]
fn part_1(input: &str) -> usize {
    input.split(',').map(hash).sum()
}

#[solution(part = 2)]
fn part_2(input: &str) -> u64 {
    let mut boxes: Vec<Vec<(&str, u64)>> = vec![Vec::new(); 256];

    for s in input.split(',') {
        if s.contains('-') {
            let label = s.split('-').next().unwrap();
            let box_idx = hash(label);

            let rm_idx = boxes[box_idx]
                .iter()
                .enumerate()
                .find(|(_, (lens_label, _))| &label == lens_label)
                .map(|(i, _)| i);

            if let Some(i) = rm_idx {
                boxes[box_idx].remove(i);
            };
        }
        // Must contain '='
        else {
            let mut iter = s.split('=');
            let label = iter.next().unwrap();
            let box_idx = hash(label);
            let focal_length = iter.next().unwrap().parse::<u64>().unwrap();

            let lens_box = &mut boxes[box_idx];
            let mut contains_label = false;

            for (lens_label, lens_focal_length) in lens_box.iter_mut() {
                if &label == lens_label {
                    contains_label = true;
                    *lens_focal_length = focal_length;
                    break;
                }
            }

            if !contains_label {
                (lens_box).push((label, focal_length));
            }
        }
    }

    boxes
        .iter()
        .enumerate()
        .filter(|(_, b)| !b.is_empty())
        .flat_map(|(i, b)| b.iter().map(move |x| (i, x)).enumerate())
        .map(|(slot_num, (box_num, (_, focal_length)))| {
            (slot_num as u64 + 1) * (box_num as u64 + 1) * focal_length
        })
        .sum()
}
