use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, newline, space1},
    error::Error,
    multi::separated_list1,
    sequence::{pair, preceded, separated_pair},
    IResult,
};

use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

fn parse_numbers(numbers: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64)(numbers)
}

fn parse_input(input: &str) -> Vec<(u64, u64)> {
    let (times, distances) = separated_pair(
        preceded(pair(tag("Time:"), space1), parse_numbers),
        newline,
        preceded(pair(tag("Distance:"), space1), parse_numbers),
    )(input)
    .unwrap()
    .1;
    times.into_iter().zip(distances).collect_vec()
}

fn parse_input_2(input: &str) -> (u64, u64) {
    let (times, distances) = separated_pair::<_, _, _, _, Error<_>, _, _, _>(
        preceded(pair(tag("Time:"), space1), separated_list1(space1, digit1)),
        newline,
        preceded(
            pair(tag("Distance:"), space1),
            separated_list1(space1, digit1),
        ),
    )(input)
    .unwrap()
    .1;
    (
        times.join("").parse().unwrap(),
        distances.join("").parse().unwrap(),
    )
}

fn count_winnable((time, distance): (u64, u64)) -> usize {
    (1..(time + 1))
        .filter(|t| (time - t) * t > distance)
        .count()
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let races = parse_input(input);
        races
            .into_iter()
            .map(count_winnable)
            .product::<usize>()
            .to_string()
    }
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let race = parse_input_2(input);
        count_winnable(race).to_string()
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
