use std::collections::HashSet;

use itertools::Itertools;

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 3;

fn parse_input(input: &str) -> Vec<(&str, &str)> {
    let mut parsed = Vec::new();
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        parsed.push((first, second));
    }
    parsed
}

fn priority(item: &char) -> i32 {
    if item.is_ascii_lowercase() {
        std::convert::Into::<i32>::into(*item as u8 - b'a') + 1
    } else {
        std::convert::Into::<i32>::into(*item as u8 - b'A') + 27
    }
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let rucksacks = parse_input(input);
        let mut total = 0;
        for (first, second) in rucksacks {
            let first_items: HashSet<char> = HashSet::from_iter(first.chars());
            let second_items: HashSet<char> = HashSet::from_iter(second.chars());
            let unique = first_items.intersection(&second_items).next().unwrap();
            total += priority(unique);
        }
        total.to_string()
    }
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let mut total = 0;
        for (first, second, third) in input.lines().tuples() {
            let first_items: HashSet<char> = HashSet::from_iter(first.chars());
            let second_items: HashSet<char> = HashSet::from_iter(second.chars());
            let third_items: HashSet<char> = HashSet::from_iter(third.chars());
            let unique = first_items
                .intersection(&second_items)
                .find(|c| third_items.contains(c))
                .unwrap();
            total += priority(unique);
        }
        total.to_string()
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
