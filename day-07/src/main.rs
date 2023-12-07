use advent_utils::macros::solution;
use std::{cmp::Ordering, os::macos::raw};

fn main() {
    part_1();
    part_2();
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CardDay1 {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,  // 11
    Queen, // 12
    King,  // 13
    Ace,   // 14
}

impl Into<usize> for CardDay1 {
    fn into(self) -> usize {
        self as usize
    }
}

impl TryFrom<char> for CardDay1 {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '2' => Ok(CardDay1::Two),
            '3' => Ok(CardDay1::Three),
            '4' => Ok(CardDay1::Four),
            '5' => Ok(CardDay1::Five),
            '6' => Ok(CardDay1::Six),
            '7' => Ok(CardDay1::Seven),
            '8' => Ok(CardDay1::Eight),
            '9' => Ok(CardDay1::Nine),
            'T' => Ok(CardDay1::Ten),
            'J' => Ok(CardDay1::Jack),
            'Q' => Ok(CardDay1::Queen),
            'K' => Ok(CardDay1::King),
            'A' => Ok(CardDay1::Ace),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandKind {
    /// Returns the kind of hand given a list of cardDay1s.
    fn get_kind_day1(cards: &[CardDay1]) -> Self {
        let mut counts = [0; 15];
        for card in cards {
            counts[*card as usize] += 1;
        }

        let mut has_pair = false;
        let mut has_three = false;
        let mut has_four = false;
        let mut has_five = false;
        let mut has_2_pair = false;
        for count in counts.iter() {
            match count {
                2 => {
                    if has_pair {
                        has_2_pair = true;
                    }
                    has_pair = true;
                }
                3 => has_three = true,
                4 => has_four = true,
                5 => has_five = true,
                _ => (),
            }
        }

        if has_five {
            HandKind::FiveOfAKind
        } else if has_four {
            HandKind::FourOfAKind
        } else if has_three && has_pair {
            HandKind::FullHouse
        } else if has_three {
            HandKind::ThreeOfAKind
        } else if has_2_pair {
            HandKind::TwoPair
        } else if has_pair {
            HandKind::OnePair
        } else {
            HandKind::HighCard
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand<T>
where
    T: Ord,
{
    kind: HandKind,
    card: Vec<T>,
    bid: u64,
}

impl<T> Ord for Hand<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => self.card.cmp(&other.card),
            o => o,
        }
    }
}

impl<T> PartialOrd for Hand<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_hands_day1(input: &str) -> Vec<Hand<CardDay1>> {
    input
        .lines()
        .map(|l| {
            let mut splits = l.split_whitespace();
            let card = splits
                .next()
                .unwrap()
                .chars()
                .map(|c| CardDay1::try_from(c).unwrap())
                .collect::<Vec<_>>();
            let bid = splits.next().unwrap().parse().unwrap();
            Hand {
                kind: HandKind::get_kind_day1(&card),
                card,
                bid,
            }
        })
        .collect()
}

#[solution(part = 1)]
fn part_1(input: &str) -> u64 {
    let mut hands = parse_hands_day1(input);
    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i as u64 + 1))
        .sum()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CardDay2 {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen, // 12
    King,  // 13
    Ace,   // 14
}

impl TryFrom<char> for CardDay2 {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'J' => Ok(CardDay2::Joker),
            '2' => Ok(CardDay2::Two),
            '3' => Ok(CardDay2::Three),
            '4' => Ok(CardDay2::Four),
            '5' => Ok(CardDay2::Five),
            '6' => Ok(CardDay2::Six),
            '7' => Ok(CardDay2::Seven),
            '8' => Ok(CardDay2::Eight),
            '9' => Ok(CardDay2::Nine),
            'T' => Ok(CardDay2::Ten),
            'Q' => Ok(CardDay2::Queen),
            'K' => Ok(CardDay2::King),
            'A' => Ok(CardDay2::Ace),
            _ => Err(()),
        }
    }
}

fn parse_hands_day2(input: &str) -> Vec<Hand<CardDay2>> {
    input
        .lines()
        .map(|l| {
            let mut splits = l.split_whitespace();
            let card = splits
                .next()
                .unwrap()
                .chars()
                .map(|c| CardDay2::try_from(c).unwrap())
                .collect::<Vec<_>>();
            let bid = splits.next().unwrap().parse().unwrap();
            Hand {
                kind: HandKind::get_kind_day2(&card),
                card,
                bid,
            }
        })
        .collect()
}

impl HandKind {
    /// Returns the kind of hand given a list of cardDay1s.
    fn get_kind_day2(cards: &[CardDay2]) -> Self {
        let mut raw_counts: [i32; 15] = [0; 15];
        for card in cards {
            raw_counts[*card as usize] += 1;
        }

        let joker_count = raw_counts[CardDay2::Joker as usize];
        let mut max_score = HandKind::get_raw_kind_day2(raw_counts);

        if joker_count == 0 {
            return max_score;
        }

        for i in 0..15 {
            if raw_counts[i] == 0 || i == CardDay2::Joker as usize {
                continue;
            }

            let original_count = raw_counts[i];
            raw_counts[i] = (raw_counts[i] + joker_count).min(5); // Ensure the count doesn't exceed 5
            max_score = max_score.max(HandKind::get_raw_kind_day2(raw_counts));
            raw_counts[i] = original_count; // Restore the original count
        }

        max_score
    }

    fn get_raw_kind_day2(counts: [i32; 15]) -> Self {
        let mut has_pair = false;
        let mut has_three = false;
        let mut has_four = false;
        let mut has_five = false;
        let mut has_2_pair = false;
        for count in counts.iter() {
            match count {
                2 => {
                    if has_pair {
                        has_2_pair = true;
                    }
                    has_pair = true;
                }
                3 => has_three = true,
                4 => has_four = true,
                5 => has_five = true,
                x if x > &5 => has_five = true,
                _ => (),
            }
        }

        if has_five {
            HandKind::FiveOfAKind
        } else if has_four {
            HandKind::FourOfAKind
        } else if has_three && has_pair {
            HandKind::FullHouse
        } else if has_three {
            HandKind::ThreeOfAKind
        } else if has_2_pair {
            HandKind::TwoPair
        } else if has_pair {
            HandKind::OnePair
        } else {
            HandKind::HighCard
        }
    }
}

#[solution(part = 2)]
fn part_2(input: &str) -> u64 {
    let mut hands = parse_hands_day2(input);
    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i as u64 + 1))
        .sum()
}
