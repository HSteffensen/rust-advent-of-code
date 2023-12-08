use std::ops::RangeInclusive;

use regex::Regex;

use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

fn parse_ranges(input: &str) -> Vec<(u32, u32, u32, u32)> {
    let mut ranges = Vec::new();
    let re = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    for matches in re.captures_iter(input) {
        let a: u32 = matches[1].parse().unwrap();
        let b: u32 = matches[2].parse().unwrap();
        let c: u32 = matches[3].parse().unwrap();
        let d: u32 = matches[4].parse().unwrap();
        ranges.push((a, b, c, d));
    }
    ranges
}

fn range_fully_contains(outer: &RangeInclusive<&u32>, inner_start: &u32, inner_end: &u32) -> bool {
    outer.contains(&inner_start) && outer.contains(&inner_end)
}

fn range_partially_contains(
    outer: &RangeInclusive<&u32>,
    inner_start: &u32,
    inner_end: &u32,
) -> bool {
    outer.contains(&inner_start) || outer.contains(&inner_end)
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        parse_ranges(input)
            .iter()
            .filter(|(a, b, c, d)| {
                range_fully_contains(&(a..=b), c, d) || range_fully_contains(&(c..=d), a, b)
            })
            .count()
            .to_string()
    }
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        parse_ranges(input)
            .iter()
            .filter(|(a, b, c, d)| {
                range_partially_contains(&(a..=b), c, d) || range_partially_contains(&(c..=d), a, b)
            })
            .count()
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
