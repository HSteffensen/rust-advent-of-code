use std::collections::BinaryHeap;

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 1;

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let elves_foods = parse_input(input);
        let max: u64 = elves_foods
            .iter()
            .map(|foods| foods.iter().sum())
            .max()
            .unwrap();
        max.to_string()
    }
}

fn parse_input(input: &str) -> Vec<Vec<u64>> {
    let mut elves = Vec::new();
    for chunk in input.split("\n\n") {
        let mut foods = Vec::new();
        for line in chunk.lines() {
            let calories = line.parse().unwrap();
            foods.push(calories);
        }
        elves.push(foods);
    }
    elves
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let elves_foods = parse_input(input);
        let elves_sums = elves_foods.iter().map(|foods| foods.iter().sum::<u64>());
        let heap = BinaryHeap::from_iter(elves_sums);
        let max_3 = heap.into_iter().take(3);
        max_3.sum::<u64>().to_string()
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
