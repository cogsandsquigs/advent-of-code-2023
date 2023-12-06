use advent_utils::macros::solution;

fn main() {
    part_1();
    part_2();
}

fn parse_input_day_1(input: &str) -> Vec<(u64, u64)> {
    let mut input = input.lines();
    let times = input
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap());
    let distances = input
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .filter(|x| !x.is_empty())
        .map(|x| x.parse().unwrap());

    times.zip(distances).collect()
}

// We can make this waaay faster with a bit of math
#[solution(part = 1)]
fn part_1(input: &str) -> u64 {
    let mut ways_product = 1;

    for (t, d) in parse_input_day_1(input) {
        let t = t as f64;
        let d = d as f64;
        // We can solve for $-x^{2}+tx-d\ge0$ to find the range of $x$ that satisfies the conditions where
        // $t$ is the time and $d$ is the distance, and $x$ is the time held pressing the button.

        let mut lower = ((t - ((-4.0 * d + t.powi(2)).sqrt())) / 2.0).ceil() as u64;
        let mut upper = ((t + ((-4.0 * d + t.powi(2)).sqrt())) / 2.0).floor() as u64;

        // Check bounds on lower and upper
        if !(lower as f64 * (t - lower as f64) > d) {
            lower += 1;
        }

        if !(upper as f64 * (t - upper as f64) > d) {
            upper -= 1;
        }

        println!("{} {}", lower, upper);

        ways_product *= upper - lower + 1;
    }

    ways_product
}

fn parse_input_day_2(input: &str) -> (u64, u64) {
    let mut input = input.lines();
    let time = input
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();
    let distance = input
        .next()
        .unwrap()
        .split(':')
        .nth(1)
        .unwrap()
        .replace(' ', "")
        .parse()
        .unwrap();

    (time, distance)
}

#[solution(part = 2)]
fn part_2(input: &str) -> u64 {
    let (t, d) = parse_input_day_2(input);
    let t = t as f64;
    let d = d as f64;
    // We can solve for $-x^{2}+tx-d\ge0$ to find the range of $x$ that satisfies the conditions where
    // $t$ is the time and $d$ is the distance, and $x$ is the time held pressing the button.

    let mut lower = ((t - ((-4.0 * d + t.powi(2)).sqrt())) / 2.0).ceil() as u64;
    let mut upper = ((t + ((-4.0 * d + t.powi(2)).sqrt())) / 2.0).floor() as u64;

    // Check bounds on lower and upper
    if !(lower as f64 * (t - lower as f64) > d) {
        lower += 1;
    }

    if !(upper as f64 * (t - upper as f64) > d) {
        upper -= 1;
    }

    println!("{} {}", lower, upper);

    upper - lower + 1
}
