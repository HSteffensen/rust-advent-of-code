use std::collections::HashSet;

use itertools::Itertools;

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 6;

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        for (i, (a, b, c, d)) in input.chars().tuple_windows().enumerate() {
            if HashSet::<&char>::from_iter(vec![a, b, c, d].iter()).len() == 4 {
                return (i + 4).to_string();
            }
        }
        unreachable!()
    }
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let size = 14;
        for i in 0..input.len() - size {
            let input14 = &input[i..i + size];
            if HashSet::<char>::from_iter(input14.chars()).len() == size {
                return (i + size).to_string();
            }
        }
        unreachable!()
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
