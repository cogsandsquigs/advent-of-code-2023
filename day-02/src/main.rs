use advent_utils::macros::solution;
use nom::{
    bytes::complete::{tag, take_till},
    character::complete::digit1,
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

fn main() {
    part_1();
    part_2();
}

#[derive(Debug)]
struct Game {
    number: u32,

    // In order: Red, Green, Blue
    rounds: Vec<(u32, u32, u32)>,
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (input, number) = delimited(tag("Game "), digit1, tag(": "))(input)?;
    let (input, rounds) = separated_list1(
        tag("; "),
        separated_list1(
            tag(", "),
            tuple((
                terminated(take_till(|c| c == ' '), tag(" ")),
                take_till(|c| c == ',' || c == ';' || c == '\n'),
            )),
        ),
    )(input)?;

    let mut game = Game {
        number: number.parse().unwrap(),
        rounds: vec![],
    };

    for round in &rounds {
        let mut tuple_round = (0, 0, 0);

        for (score, color) in round {
            match *color {
                "red" => tuple_round.0 = score.parse().unwrap(),
                "green" => tuple_round.1 = score.parse().unwrap(),
                "blue" => tuple_round.2 = score.parse().unwrap(),
                _ => unreachable!(),
            }
        }

        game.rounds.push(tuple_round);
    }

    Ok((input, game))
}

fn to_games(input: &str) -> Vec<Game> {
    input
        .lines()
        .map(|line| parse_game(line).unwrap().1)
        .collect()
}
// red, green, blue
fn is_possible(game: &Game, constraints: (u32, u32, u32)) -> bool {
    for round in &game.rounds {
        if round.0 > constraints.0 || round.1 > constraints.1 || round.2 > constraints.2 {
            return false;
        }
    }

    true
}

#[solution(part = 1)]
fn part_1(input: &str) -> u32 {
    let games: Vec<Game> = to_games(input);

    games
        .iter()
        .filter(|game| is_possible(game, (12, 13, 14)))
        .map(|game| game.number)
        .sum()
}

// red, green, blue
fn fewest_cubes(game: &Game) -> (u32, u32, u32) {
    let mut min = (0, 0, 0);

    for round in &game.rounds {
        if min.0 < round.0 {
            min.0 = round.0;
        }

        if min.1 < round.1 {
            min.1 = round.1;
        }

        if min.2 < round.2 {
            min.2 = round.2;
        }
    }

    min
}

#[solution(part = 2)]
fn part_2(input: &str) -> u32 {
    let games: Vec<Game> = to_games(input);

    games
        .iter()
        .map(|game| {
            let min = fewest_cubes(game);
            min.0 * min.1 * min.2
        })
        .sum()
}
