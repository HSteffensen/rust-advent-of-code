use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, multispace1},
    combinator::map,
    multi::separated_list1,
    sequence::{pair, separated_pair, tuple},
    IResult,
};

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 4;

struct Card {
    id: usize,
    winning_numbers: HashSet<u32>,
    drawn_numbers: HashSet<u32>,
}

fn parse_numbers(input: &str) -> IResult<&str, HashSet<u32>> {
    map(separated_list1(multispace1, complete::u32), |numbers| {
        HashSet::from_iter(numbers.into_iter())
    })(input)
}

fn parse_card(line: &str) -> Card {
    map(
        tuple((
            pair(tag("Card"), multispace1),
            complete::u32,
            pair(tag(":"), multispace1),
            separated_pair(parse_numbers, pair(tag(" |"), multispace1), parse_numbers),
        )),
        |(_, id, _, (winning_numbers, drawn_numbers))| Card {
            id: id.try_into().unwrap(),
            winning_numbers,
            drawn_numbers,
        },
    )(line)
    .unwrap()
    .1
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        input
            .lines()
            .map(parse_card)
            .map(|card| (card.winning_numbers.intersection(&card.drawn_numbers)).count())
            .map(|intersection| match intersection {
                0 => 0,
                _ => 2u32.pow((intersection - 1).try_into().unwrap()),
            })
            .sum::<u32>()
            .to_string()
    }
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let cards = input.lines().map(parse_card).collect_vec();
        let mut card_counts: HashMap<usize, usize> =
            HashMap::from_iter(cards.iter().map(|c| (c.id, 1)));

        for card in cards {
            let count = card_counts[&card.id];
            let score = (card.winning_numbers.intersection(&card.drawn_numbers)).count();
            for i in card.id..(card.id + score) {
                card_counts.entry(i + 1).and_modify(|v| *v += count);
            }
        }
        card_counts.into_values().sum::<usize>().to_string()
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
