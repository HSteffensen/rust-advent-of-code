use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, one_of},
    combinator::map,
    error::Error,
    multi::{many1, separated_list1},
    sequence::separated_pair,
};
use regex::Regex;

use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

enum SpringRecord {
    Damaged,
    Empty,
    Unknown,
}

struct RecordRow {
    springs: Vec<SpringRecord>,
    cluster_sizes: Vec<u32>,
}

impl RecordRow {
    fn all_possibilities(&self) -> Vec<String> {
        self.all_possibilities_from(0)
    }

    fn all_possibilities_from(&self, index: usize) -> Vec<String> {
        if index == self.springs.len() {
            return vec!["".to_string()];
        }
        let next = self.all_possibilities_from(index + 1);
        match self.springs[index] {
            SpringRecord::Damaged => next.into_iter().map(|r| format!("#{}", r)).collect_vec(),
            SpringRecord::Empty => next.into_iter().map(|r| format!(".{}", r)).collect_vec(),
            SpringRecord::Unknown => next
                .into_iter()
                .flat_map(|r| [format!("#{}", r), format!(".{}", r)])
                .collect_vec(),
        }
    }

    fn count_possibilities(&self) -> usize {
        let some_empty = r"\.+";
        let any_empty = r"\.*";
        let raw_regex = [any_empty.to_string()]
            .into_iter()
            .chain(
                self.cluster_sizes
                    .iter()
                    .map(|count| format!("#{{{}}}", count))
                    .zip(
                        (0..self.cluster_sizes.len() - 1)
                            .map(|_| some_empty.to_string())
                            .chain([any_empty.to_string()]),
                    )
                    .flat_map(|(a, b)| [a, b]),
            )
            .join("");
        let regex = Regex::new(format!("^{}$", raw_regex).as_str()).unwrap();
        self.all_possibilities()
            .into_iter()
            .filter(|line| regex.is_match(line))
            .count()
    }
}

fn parse_line(line: &str) -> RecordRow {
    map(
        separated_pair::<_, _, _, _, Error<_>, _, _, _>(
            many1(map(one_of("#.?"), |c| match c {
                '#' => SpringRecord::Damaged,
                '.' => SpringRecord::Empty,
                '?' => SpringRecord::Unknown,
                _ => unreachable!(),
            })),
            tag(" "),
            separated_list1(tag(","), complete::u32),
        ),
        |(springs, cluster_sizes)| RecordRow {
            springs,
            cluster_sizes,
        },
    )(line)
    .unwrap()
    .1
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        input
            .lines()
            .map(parse_line)
            .map(|l| l.count_possibilities())
            .sum::<usize>()
            .to_string()
    }
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        todo!("{}", input)
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
