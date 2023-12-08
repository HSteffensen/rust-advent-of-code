use std::{cmp::Ordering, collections::HashMap};

use itertools::Itertools;
use nom::{
    character::complete::{self, anychar, space1},
    combinator::map,
    error::Error,
    sequence::{separated_pair, tuple},
};

use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum CamelCard {
    A,
    K,
    Q,
    J,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum CamelCard2 {
    A,
    K,
    Q,
    T,
    Nine,
    Eight,
    Seven,
    Six,
    Five,
    Four,
    Three,
    Two,
    J,
}

#[derive(Debug, PartialEq, Eq)]
struct CamelHand(CamelCard, CamelCard, CamelCard, CamelCard, CamelCard);

#[derive(Debug, PartialEq, Eq)]
struct CamelHand2(CamelCard2, CamelCard2, CamelCard2, CamelCard2, CamelCard2);

impl PartialOrd for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CamelHand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.0.cmp(&other.0) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.1.cmp(&other.1) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.2.cmp(&other.2) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.3.cmp(&other.3) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.4.cmp(&other.4)
    }
}

impl PartialOrd for CamelHand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CamelHand2 {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type().cmp(&other.hand_type()) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.0.cmp(&other.0) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.1.cmp(&other.1) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.2.cmp(&other.2) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        match self.3.cmp(&other.3) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.4.cmp(&other.4)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum CamelHandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    Pair,
    HighCard,
}

impl CamelHand {
    fn hand_type(&self) -> CamelHandType {
        let mut counter: HashMap<CamelCard, u32> = HashMap::new();
        for card in [self.0, self.1, self.2, self.3, self.4] {
            let count = counter.entry(card).or_insert(0);
            *count += 1;
        }
        let mut counts = counter.values().collect_vec();
        counts.sort_unstable();
        counts.reverse();
        match counts.as_slice() {
            [5] => CamelHandType::FiveOfAKind,
            [4, 1] => CamelHandType::FourOfAKind,
            [3, 2] => CamelHandType::FullHouse,
            [3, 1, 1] => CamelHandType::ThreeOfAKind,
            [2, 2, 1] => CamelHandType::TwoPair,
            [2, 1, 1, 1] => CamelHandType::Pair,
            [1, 1, 1, 1, 1] => CamelHandType::HighCard,
            _ => unreachable!(),
        }
    }
}

impl CamelHand2 {
    fn hand_type(&self) -> CamelHandType {
        let mut counter: HashMap<CamelCard2, u32> = HashMap::new();
        for card in [self.0, self.1, self.2, self.3, self.4] {
            let count = counter.entry(card).or_insert(0);
            *count += 1;
        }
        let joker_count = counter.remove(&CamelCard2::J).unwrap_or(0);
        if joker_count == 5 {
            return CamelHandType::FiveOfAKind;
        }

        let mut counts = counter.values().collect_vec();
        counts.sort_unstable();
        counts.reverse();
        let new_top_count = counts[0] + joker_count;
        counts[0] = &new_top_count;
        match counts.as_slice() {
            [5] => CamelHandType::FiveOfAKind,
            [4, 1] => CamelHandType::FourOfAKind,
            [3, 2] => CamelHandType::FullHouse,
            [3, 1, 1] => CamelHandType::ThreeOfAKind,
            [2, 2, 1] => CamelHandType::TwoPair,
            [2, 1, 1, 1] => CamelHandType::Pair,
            [1, 1, 1, 1, 1] => CamelHandType::HighCard,
            _ => unreachable!(),
        }
    }
}

fn parse_card(card: char) -> CamelCard {
    match card {
        'A' => CamelCard::A,
        'K' => CamelCard::K,
        'Q' => CamelCard::Q,
        'J' => CamelCard::J,
        'T' => CamelCard::T,
        '9' => CamelCard::Nine,
        '8' => CamelCard::Eight,
        '7' => CamelCard::Seven,
        '6' => CamelCard::Six,
        '5' => CamelCard::Five,
        '4' => CamelCard::Four,
        '3' => CamelCard::Three,
        '2' => CamelCard::Two,
        _ => unreachable!("Unexpected card: '{}'", card),
    }
}

fn parse_line(line: &str) -> (CamelHand, u32) {
    separated_pair::<_, _, _, _, Error<_>, _, _, _>(
        map(
            tuple((anychar, anychar, anychar, anychar, anychar)),
            |(a, b, c, d, e)| {
                CamelHand(
                    parse_card(a),
                    parse_card(b),
                    parse_card(c),
                    parse_card(d),
                    parse_card(e),
                )
            },
        ),
        space1,
        complete::u32,
    )(line)
    .unwrap()
    .1
}

fn parse_card_2(card: char) -> CamelCard2 {
    match card {
        'A' => CamelCard2::A,
        'K' => CamelCard2::K,
        'Q' => CamelCard2::Q,
        'T' => CamelCard2::T,
        '9' => CamelCard2::Nine,
        '8' => CamelCard2::Eight,
        '7' => CamelCard2::Seven,
        '6' => CamelCard2::Six,
        '5' => CamelCard2::Five,
        '4' => CamelCard2::Four,
        '3' => CamelCard2::Three,
        '2' => CamelCard2::Two,
        'J' => CamelCard2::J,
        _ => unreachable!("Unexpected card: '{}'", card),
    }
}

fn parse_line_2(line: &str) -> (CamelHand2, u32) {
    separated_pair::<_, _, _, _, Error<_>, _, _, _>(
        map(
            tuple((anychar, anychar, anychar, anychar, anychar)),
            |(a, b, c, d, e)| {
                CamelHand2(
                    parse_card_2(a),
                    parse_card_2(b),
                    parse_card_2(c),
                    parse_card_2(d),
                    parse_card_2(e),
                )
            },
        ),
        space1,
        complete::u32,
    )(line)
    .unwrap()
    .1
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        input
            .lines()
            .map(parse_line)
            .sorted_by(|a, b| a.0.cmp(&b.0))
            .map(|(_, bid)| bid)
            .rev()
            .enumerate()
            .map(|(i, bid)| std::convert::TryInto::<u32>::try_into(i + 1).unwrap() * bid)
            .sum::<u32>()
            .to_string()
    }
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        input
            .lines()
            .map(parse_line_2)
            .sorted_by(|a, b| a.0.cmp(&b.0))
            .map(|(_, bid)| bid)
            .rev()
            .enumerate()
            .map(|(i, bid)| std::convert::TryInto::<u32>::try_into(i + 1).unwrap() * bid)
            .sum::<u32>()
            .to_string()
    }
}

#[test]
fn p1_pull_examples() {
    Part1::get_examples();
}

#[test]
fn p1_run() {
    Part1::solve();
}

#[test]
fn p2_pull_examples() {
    Part2::get_examples();
}

#[test]
fn p2_run() {
    Part2::solve();
}
