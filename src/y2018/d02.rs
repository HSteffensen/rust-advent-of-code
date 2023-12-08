use std::collections::{HashMap, HashSet};

use crate::common::solution::AocSolution;

pub struct Part1 {}
struct Part2 {}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let mut count2s = 0;
        let mut count3s = 0;
        for line in input.lines() {
            let counts = count_letters(line);
            if contains_some_letter_n_times(&counts, 2) {
                count2s += 1;
            }
            if contains_some_letter_n_times(&counts, 3) {
                count3s += 1;
            }
        }
        println!("{} * {}", count2s, count3s);
        (count2s * count3s).to_string()
    }

    fn map_example_input(example: &str) -> String {
        example.replace(", ", "\n")
    }
}

fn contains_some_letter_n_times<K>(count_map: &HashMap<K, u32>, n: u32) -> bool {
    count_map.iter().any(|(_, v)| v == &n)
}

fn count_letters(input: &str) -> HashMap<char, u32> {
    let mut map = HashMap::new();
    for letter in input.chars() {
        map.entry(letter).and_modify(|v| *v += 1).or_insert(1);
    }
    map
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let mut seen = HashSet::new();
        for line in input.lines() {
            for sliced in remove_letters(line) {
                if seen.contains(&sliced) {
                    return sliced.word;
                } else {
                    seen.insert(sliced);
                }
            }
        }
        unreachable!("Should always find a match.")
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct WordMissingALetter {
    word: String,
    position: usize,
}

fn remove_letters(input: &'_ str) -> impl Iterator<Item = WordMissingALetter> + '_ {
    input.char_indices().map(|(i, _)| WordMissingALetter {
        word: format!("{}{}", &input[0..i], &input[(i + 1)..]),
        position: i,
    })
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
