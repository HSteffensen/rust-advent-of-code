use std::collections::{HashMap, HashSet};

use nom::{
    bytes::complete::{tag, take},
    character::complete::{self, newline},
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 16;

struct ValveCave<'a> {
    flow_rate: u32,
    neighbors: Vec<&'a str>,
}

fn parse_line(line: &str) -> IResult<&str, (&str, ValveCave)> {
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    map(
        tuple((
            tag("Valve "),
            take(2usize),
            tag(" has flow rate="),
            complete::u32,
            tag("; tunnels lead to valves "),
            separated_list0(tag(", "), take(2usize)),
        )),
        |(_, name, _, flow_rate, _, neighbors)| {
            (
                name,
                ValveCave {
                    flow_rate,
                    neighbors,
                },
            )
        },
    )(line)
}

fn most_pressure_released<'a>(
    cave_map: HashMap<&'a str, ValveCave>,
    current_cave: &'a str,
    minutes_remaining: u32,
    pressure_released_so_far: u32,
    opened: &mut HashSet<&'a str>,
) -> u32 {
    let mut max_pressure_so_far = pressure_released_so_far;
    if !opened.contains(current_cave) && cave_map.get(current_cave).unwrap().flow_rate > 0 {
        opened.insert(current_cave);
    }
    todo!()
}

fn parse_input(input: &str) -> HashMap<&str, ValveCave> {
    let (_, caves) = all_consuming(separated_list0(newline, parse_line))(input).unwrap();
    let mut cave_map = HashMap::new();
    for (name, cave) in caves {
        cave_map.insert(name, cave);
    }
    cave_map
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
