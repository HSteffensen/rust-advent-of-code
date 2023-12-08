use std::collections::HashMap;

use nom::{
    bytes::complete::{tag, take},
    character::complete::{newline, one_of},
    combinator::map,
    error::Error,
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
};
use num::integer::lcm;

use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

enum Instruction {
    R,
    L,
}

fn parse_input(input: &str) -> (Vec<Instruction>, HashMap<&str, (&str, &str)>) {
    let mut desert_map = HashMap::new();
    let instructions = separated_pair::<_, _, _, _, Error<_>, _, _, _>(
        many1(map(one_of("RL"), |c| match c {
            'R' => Instruction::R,
            'L' => Instruction::L,
            other => unreachable!("{}", other),
        })),
        tag("\n\n"),
        separated_list1(
            newline,
            map(
                tuple((
                    take(3usize),
                    tag(" = ("),
                    take(3usize),
                    tag(", "),
                    take(3usize),
                    tag(")"),
                )),
                |(a, _, b, _, c, _)| desert_map.insert(a, (b, c)),
            ),
        ),
    )(input)
    .unwrap()
    .1
     .0;

    (instructions, desert_map)
}

fn count_steps_to_node(
    instructions: &[Instruction],
    desert_map: &HashMap<&str, (&str, &str)>,
    start_node: &str,
    target_suffix: &str,
) -> u64 {
    let mut pos = start_node;
    let mut count = 0;
    for step in instructions.iter().cycle() {
        let map_node = desert_map[pos];
        pos = match step {
            Instruction::R => map_node.1,
            Instruction::L => map_node.0,
        };
        count += 1;
        if pos.ends_with(target_suffix) {
            break;
        };
    }
    count
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let (instructions, desert_map) = parse_input(input);
        count_steps_to_node(&instructions, &desert_map, "AAA", "ZZZ").to_string()
    }
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let (instructions, desert_map) = parse_input(input);
        desert_map
            .keys()
            .filter(|key| key.ends_with('A'))
            .map(|loc| count_steps_to_node(&instructions, &desert_map, loc, "Z"))
            .reduce(lcm)
            .unwrap()
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
