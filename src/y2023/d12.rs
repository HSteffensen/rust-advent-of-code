use std::{collections::HashMap, hash::Hash, iter::repeat};

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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum SpringRecord {
    Damaged,
    Empty,
    Unknown,
}

#[derive(Debug)]
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

    fn count_2(&self) -> usize {
        let mut cache: HashMap<(usize, Option<SpringRecord>, usize, u32, bool), usize> =
            HashMap::new();
        self.count_2_helper(0, None, 0, 0, false, &mut cache)
    }

    fn count_2_helper<'a, 'b>(
        &self,
        spring_index: usize,
        force_spring: Option<SpringRecord>,
        cluster_index: usize,
        cluster_count: u32,
        in_cluster: bool,
        cache: &mut HashMap<(usize, Option<SpringRecord>, usize, u32, bool), usize>,
    ) -> usize {
        if spring_index == self.springs.len() {
            return if cluster_index == self.cluster_sizes.len()
                || (cluster_index == (self.cluster_sizes.len() - 1)
                    && cluster_count == self.cluster_sizes[cluster_index])
            {
                1
            } else {
                0
            };
        }
        if cluster_index == self.cluster_sizes.len() {
            return if self
                .springs
                .iter()
                .skip(spring_index)
                .all(|s| s == &SpringRecord::Empty || s == &SpringRecord::Unknown)
            {
                1
            } else {
                0
            };
        }
        let cache_key = (
            spring_index,
            force_spring,
            cluster_index,
            cluster_count,
            in_cluster,
        );
        if let Some(&x) = cache.get(&cache_key) {
            return x;
        }
        let current_spring = force_spring.unwrap_or(self.springs[spring_index]);
        let answer = match current_spring {
            SpringRecord::Damaged => {
                if cluster_count < self.cluster_sizes[cluster_index] {
                    self.count_2_helper(
                        spring_index + 1,
                        None,
                        cluster_index,
                        cluster_count + 1,
                        true,
                        cache,
                    )
                } else {
                    0
                }
            }
            SpringRecord::Empty => {
                if in_cluster {
                    if cluster_count == self.cluster_sizes[cluster_index] {
                        self.count_2_helper(
                            spring_index + 1,
                            None,
                            cluster_index + 1,
                            0,
                            false,
                            cache,
                        )
                    } else {
                        0
                    }
                } else {
                    self.count_2_helper(
                        spring_index + 1,
                        None,
                        cluster_index,
                        cluster_count,
                        in_cluster,
                        cache,
                    )
                }
            }
            SpringRecord::Unknown => {
                self.count_2_helper(
                    spring_index,
                    Some(SpringRecord::Damaged),
                    cluster_index,
                    cluster_count,
                    in_cluster,
                    cache,
                ) + self.count_2_helper(
                    spring_index,
                    Some(SpringRecord::Empty),
                    cluster_index,
                    cluster_count,
                    in_cluster,
                    cache,
                )
            }
        };
        cache.insert(cache_key, answer);
        answer
    }

    fn as_part_2(self) -> RecordRow {
        let mut springs = self.springs;
        springs.push(SpringRecord::Unknown);
        let mut springs = repeat(springs).take(5).flatten().collect_vec();
        springs.pop();
        RecordRow {
            springs,
            cluster_sizes: repeat(self.cluster_sizes).take(5).flatten().collect_vec(),
        }
    }
}

#[test]
fn test_count_2() {
    assert_eq!(1, parse_line("???.### 1,1,3").count_2());
    assert_eq!(10, parse_line("?###???????? 3,2,1").count_2());
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
            .map(|l| l.count_2())
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
        input
            .lines()
            .map(parse_line)
            .map(|l| l.as_part_2().count_2())
            .sum::<usize>()
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
