use advent_utils::macros::solution;

fn main() {
    part_1();
    part_2();
}

fn split_into_chunks(input: &str) -> Vec<&str> {
    input.split('.').filter(|s| !s.is_empty()).collect()
}

fn parse_input(input: &str) -> Vec<(Vec<&str>, Vec<u64>)> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split_whitespace();

            (
                split_into_chunks(iter.next().unwrap()),
                iter.next()
                    .unwrap()
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect(),
            )
        })
        .collect()
}

#[solution(part = 1)]
fn part_1(input: &str) -> u64 {
    let mut ways_sum = 0;
    let parsed = parse_input(input);

    for (chunks, sizes) in parsed {
        let mut ways = 0;

        println!("{:?}; {:?}", chunks, sizes);

        // Idea: we count the number of chunks and the number of sizes. If they are equal, then we
        // have a 1:1 mapping between the two and can exploit that to calculate the number of ways
        // trivially. If they're not equal, then we have to do some more work.
        // What's that work tho?
        // We could work backwards, seeing what satisfies the constraints of the last chunk, and
        // then work our way backwards from there. We could also work forwards, seeing what
        // satisfies the constraints of the first chunk, and then work our way forwards from there.
        // Either way would be tedious, because of the case where we have to satisfy multiple
        // sizes with a single chunk, without knowing which chunk we're working with.

        ways_sum += ways;
    }

    todo!()
}

#[solution(part = 2)]
fn part_2(input: &str) -> u64 {
    0
}
