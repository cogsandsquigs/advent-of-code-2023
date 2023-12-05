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

struct SeedRange {
    // Start, end
    // Not necessarily ordered.
    ranges: Vec<(i64, i64)>,
}

impl Mapping {
    /// Map a seed range to a destination range, accounting for segments becoming disjoint
    fn map_seed_range(&self, src: SeedRange) -> SeedRange {
        let mut ranges = Vec::new();

        for range in src.ranges {
            for segment in &self.segments {
                // Skip if range is less than the start
                if range.1 < segment.src_start {
                    continue;
                }

                // Skip if range is greater than the end
                if range.0 > segment.src_start + segment.length {
                    continue;
                }

                // If the range is wholly contained in the segment, map it
                if range.0 >= segment.src_start && range.1 <= segment.src_start + segment.length {
                    ranges.push((segment.map(range.0), segment.map(range.1)));
                    continue;
                }

                // If the range is outside and inside a segment, split into 3 ranges:
                // one before the segment, one in it, and one after it
                if range.0 < segment.src_start && range.1 > segment.src_start + segment.length {
                    ranges.push((range.0, segment.src_start - 1));
                    ranges.push((segment.dst_start, segment.dst_start + segment.length - 1));
                    ranges.push((segment.src_start + segment.length, range.1));

                    continue;
                }

                todo!()
            }
        }

        SeedRange { ranges }
    }
}

#[solution(part = 2)]
fn part_2(input: &str) -> i64 {
    let (seeds, mappings) = parse_input(input);
    let seeds = seeds
        .iter()
        .tuples()
        .map(|(a, b)| SeedRange {
            ranges: vec![(*a, a + b - 1)],
        })
        .collect_vec();
    let mut min_location = i64::MAX;

    for mut seed in seeds {
        for mapping in &mappings {
            let mapped = mapping.map_seed_range(seed);
            seed = mapped;
        }

        min_location = min_location.min(seed.ranges[0].0);
    }

    min_location
}
