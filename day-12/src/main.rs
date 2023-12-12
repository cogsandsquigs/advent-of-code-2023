use advent_utils::macros::solution;

fn main() {
    part_1();
    part_2();
}

fn parse_input(input: &str) -> impl Iterator<Item = (Vec<&str>, Vec<u64>)> + '_ {
    input.lines().map(|line| {
        let mut iter = line.split_whitespace();

        (
            iter.next()
                .unwrap()
                .split('.')
                .filter(|s| !s.is_empty())
                .collect(),
            iter.next()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect(),
        )
    })
}

// Min, max
fn get_chunk_size_bounds(chunk: &str) -> (usize, usize) {
    (
        chunk
            .split('?')
            .filter(|s| !s.is_empty())
            .min()
            .map(|s| s.len())
            .unwrap_or(0),
        chunk.len(),
    )
}

#[solution(part = 1)]
fn part_1(input: &str) -> u64 {
    let mut ways_sum = 0;

    for (chunks, sizes) in parse_input(input) {
        let mut ways = 0;
        let to_work = sizes.iter().sum::<u64>()
            - chunks
                .iter()
                .flat_map(|c| c.chars())
                .filter(|c| *c == '#')
                .count() as u64;

        println!("{:?}; {:?}; {}", chunks, sizes, to_work);

        // Idea: we count the number of chunks and the number of sizes. If they are equal, then we
        // have a 1:1 mapping between the two and can exploit that to calculate the number of ways
        // trivially. If they're not equal, then we have to do some more work.
        // How do we calculate the number of ways per chunk?
        // We could just look at the number of possible sizes (both min and max), and then manually
        // calculate the number of ways for each size. Will that scale? How will we do the manuall
        // calculation?

        ways_sum += ways;
    }

    ways_sum
}

#[solution(part = 2)]
fn part_2(input: &str) -> u64 {
    0
}
