use itertools::Itertools;
use nom::{
    character::complete::{self, newline, space1},
    error::Error,
    multi::separated_list1,
};

use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    separated_list1(
        newline::<_, Error<_>>,
        separated_list1(space1, complete::i64),
    )(input)
    .unwrap()
    .1
}

fn next_in_sequence(numbers: &[i64]) -> i64 {
    if numbers.iter().all(|n| n == &0) {
        return 0;
    }
    let last = numbers.last().unwrap();
    let diffs = numbers
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec();
    next_in_sequence(&diffs) + last
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        parse_input(input)
            .into_iter()
            .map(|v| next_in_sequence(&v))
            .sum::<i64>()
            .to_string()
    }
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        parse_input(input)
            .into_iter()
            .map(|v| next_in_sequence(&v.into_iter().rev().collect_vec()))
            .sum::<i64>()
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
