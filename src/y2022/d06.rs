use std::collections::HashSet;

use itertools::Itertools;

use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        for (i, (a, b, c, d)) in input.chars().tuple_windows().enumerate() {
            if HashSet::<&char>::from_iter([a, b, c, d].iter()).len() == 4 {
                return (i + 4).to_string();
            }
        }
        unreachable!()
    }
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

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

struct Part1Impl2 {}
struct Part2Impl2 {}

fn find_window_end(input: &str, size: usize) -> usize {
    let (window_start, _) = input
        .chars()
        .collect_vec()
        .windows(size)
        .find_position(|window| window.iter().all_unique())
        .unwrap();
    window_start + size
}

impl AocSolution for Part1Impl2 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        find_window_end(input, 4).to_string()
    }
}

impl AocSolution for Part2Impl2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        find_window_end(input, 14).to_string()
    }
}

#[test]
fn p1_impl2_run() {
    Part1Impl2::solve();
}

#[test]
fn p2_impl2_run() {
    Part2Impl2::solve();
}
