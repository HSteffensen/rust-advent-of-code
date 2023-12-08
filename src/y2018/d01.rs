use std::collections::HashSet;

use crate::common::{data::input_to_ints, solution::AocSolution};

pub struct Part1 {}
struct Part2 {}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let ints = input_to_ints(input);
        let sum: i64 = ints.iter().sum();
        sum.to_string()
    }

    fn map_example_input(example: &str) -> String {
        example.replace(", ", "\n")
    }
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let ints = input_to_ints(input);
        let ints = ints.iter().cycle();
        let mut visited = HashSet::new();
        visited.insert(0);
        let mut total = 0;
        for next in ints {
            total += next;
            if visited.contains(&total) {
                return total.to_string();
            } else {
                visited.insert(total);
                continue;
            }
        }
        "".to_string()
    }

    fn map_example_input(example: &str) -> String {
        example.replace(", ", "\n")
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
