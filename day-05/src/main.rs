use advent_utils::macros::solution;
use itertools::Itertools;

fn main() {
    part_1();
    part_2();
}

#[derive(Debug)]
struct MapSegment {
    dst_start: i64,
    src_start: i64,
    length: i64,
}

impl MapSegment {
    /// Check if a value is contained in the segment
    fn contains(&self, value: i64) -> bool {
        value >= self.src_start && value < self.src_start + self.length
    }

    /// Map a value to the destination value
    fn map(&self, src: i64) -> i64 {
        if src >= self.src_start && src < self.src_start + self.length {
            self.dst_start + src - self.src_start
        } else {
            src
        }
    }
}

#[derive(Debug)]
struct Mapping {
    // The segments of the mapping
    segments: Vec<MapSegment>,
}

impl Mapping {
    /// Map a source value to a destination value
    fn map(&self, src: i64) -> i64 {
        for segment in &self.segments {
            if segment.contains(src) {
                return segment.map(src);
            }
        }

        src
    }
}

fn parse_input(input: &str) -> (Vec<i64>, Vec<Mapping>) {
    let mut parts = input.split("\n\n");
    let seeds = parts.next().unwrap()[7..]
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    let mappings = parts
        .map(|part| Mapping {
            segments: part
                .lines()
                .skip(1)
                .map(|line| {
                    let mut parts = line.split(' ');
                    let dst_start = parts.next().unwrap().parse().unwrap();
                    let src_start = parts.next().unwrap().parse().unwrap();
                    let length = parts.next().unwrap().parse().unwrap();
                    MapSegment {
                        src_start,
                        dst_start,
                        length,
                    }
                })
                .collect(),
        })
        .collect();

    (seeds, mappings)
}

#[solution(part = 1)]
fn part_1(input: &str) -> i64 {
    let (seeds, mappings) = parse_input(input);
    let mut min_location = i64::MAX;

    for mut seed in seeds {
        for mapping in &mappings {
            let mapped = mapping.map(seed);
            seed = mapped;
        }

        min_location = min_location.min(seed);
    }

    min_location
}

// struct SeedRange {
//     // Start, end
//     // Not necessarily ordered.
//     ranges: Vec<(i64, i64)>,
// }

// impl Mapping {
//     /// Map a seed range to a destination range, accounting for segments becoming disjoint
//     fn map_seed_range(&self, src: SeedRange) -> SeedRange {
//         let mut old_ranges = src.ranges.clone();
//         let mut new_ranges = vec![];

//         while let Some((start, end)) = old_ranges.pop() {
//             for segment in &self.segments {
//                 // Skip the segment if it doesn't overlap with the range
//                 if (segment.src_start >= start && segment.src_start >= end)
//                     || (segment.src_start + segment.length <= start
//                         && segment.src_start + segment.length <= end)
//                 {
//                     continue;
//                 }

//                 // If the segment is contained in the range, map the range and return
//                 let new_range = segment.map_range(start, end);
//             }

//             new_ranges.push((self.map(start), self.map(end)));
//         }

//         SeedRange { ranges: new_ranges }
//     }
// }

#[solution(part = 2)]
fn part_2(input: &str) -> i64 {
    let (seeds, mappings) = parse_input(input);
    let seeds = seeds
        .iter()
        .tuples()
        .flat_map(|(a, b)| *a..(a + b))
        .collect_vec();
    let mut min_location = i64::MAX;

    for mut seed in seeds {
        for mapping in &mappings {
            let mapped = mapping.map(seed);
            seed = mapped;
        }

        min_location = min_location.min(seed);
    }

    min_location
}
