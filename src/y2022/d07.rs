use std::str::Lines;

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 7;

enum FsObject {
    Dir(String, Vec<FsObject>),
    File(String, usize),
}

fn parse_input(input: &str) -> FsObject {
    let mut lines = input.lines();

    todo!()
}

fn parse_remaining_input(lines: &mut Lines) -> Option<FsObject> {
    if let Some(line) = lines.next() {
        if line.starts_with("$ cd") {
        } else if line.starts_with("$ ls") {
        } else {
        }
    } else {
        None
    }
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        todo!()
    }
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        todo!()
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
