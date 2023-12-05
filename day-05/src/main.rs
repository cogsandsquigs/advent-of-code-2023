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
    src_end: i64,
}

impl MapSegment {
    fn new_from_input(dst_start: i64, src_start: i64, length: i64) -> Self {
        Self {
            dst_start,
            src_start,
            src_end: src_start + length - 1,
        }
    }

    /// Check if a value is contained in the segment
    fn contains(&self, value: i64) -> bool {
        value >= self.src_start && value <= self.src_end
    }

    /// Map a value to the destination value
    fn map(&self, src: i64) -> i64 {
        if src >= self.src_start && src <= self.src_end {
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
                    MapSegment::new_from_input(dst_start, src_start, length)
                })
                .sorted_by_key(|x| x.src_start)
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

impl Mapping {
    /// Maps a seed range to a number of ranges
    fn map_seed_range(&self, mut seed_range: (i64, i64)) -> Vec<(i64, i64)> {
        let mut new_ranges: Vec<(i64, i64)> = vec![];

        for splitter in &self.segments {
            // If the seed range starts inside a segment
            if seed_range.0 >= splitter.src_start {
                // If the seed range is wholly inside the splitter, then we just append to the ranges and return.
                if seed_range.1 <= splitter.src_end {
                    new_ranges.push((splitter.map(seed_range.0), splitter.map(seed_range.1)));
                    continue;
                }
                // Otherwise, we split into 2 segments: one we push (the one before, in the range), and one we loop on again
                else {
                    new_ranges.push((splitter.map(seed_range.0), splitter.map(splitter.src_end)));
                    seed_range = (splitter.src_end + 1, seed_range.1);
                }
            }
            // If the seed range starts outside a segment but ends inside/after one
            else if seed_range.0 < splitter.src_start {
                // If it's not in the splitter, continue
                if seed_range.1 <= splitter.src_start {
                    continue;
                }
                // If the seed ends inside the splitter, we just make the 2 new seed ranges and return
                else if seed_range.1 <= splitter.src_end {
                    new_ranges.push((
                        splitter.map(seed_range.0),
                        splitter.map(splitter.src_start - 1),
                    ));
                    new_ranges.push((splitter.src_start, seed_range.1));
                    continue;
                }
                // Otherwise, we split into 3 segments: the before, the inside, and the after (which we loop on again)
                else {
                    new_ranges.push((
                        splitter.map(seed_range.0),
                        splitter.map(splitter.src_start - 1),
                    ));
                    new_ranges.push((splitter.src_start, splitter.src_end));
                    seed_range = (splitter.src_end + 1, seed_range.1);
                }
            }
        }

        new_ranges
    }
}

#[solution(part = 2)]
fn part_2(input: &str) -> i64 {
    let (seeds, mappings) = parse_input(input);
    let mut seed_ranges = seeds
        .iter()
        .tuples()
        .map(|(a, b)| (*a, (a + b)))
        .collect_vec();

    for mapping in &mappings {
        let mut new_seeds = vec![];

        for seed_range in seed_ranges {
            new_seeds.append(&mut mapping.map_seed_range(seed_range))
        }

        seed_ranges = new_seeds.iter().unique().copied().collect_vec();
    }

    seed_ranges.iter().sorted_by_key(|x| x.0).next().unwrap().0
}
