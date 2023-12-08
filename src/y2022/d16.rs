use std::collections::{HashMap, HashSet, LinkedList};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self, newline},
    combinator::{all_consuming, map},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

#[derive(Debug)]
struct ValveCave<'a> {
    flow_rate: u32,
    neighbors: Vec<&'a str>,
}

fn parse_input(input: &str) -> HashMap<&str, ValveCave> {
    let (_, caves) = all_consuming(separated_list0(newline, parse_line))(input.trim()).unwrap();
    let mut cave_map = HashMap::new();
    for (name, cave) in caves {
        cave_map.insert(name, cave);
    }
    cave_map
}

fn parse_line(line: &str) -> IResult<&str, (&str, ValveCave)> {
    // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
    map(
        tuple((
            tag("Valve "),
            take(2usize),
            tag(" has flow rate="),
            complete::u32,
            alt((
                tag("; tunnels lead to valves "),
                tag("; tunnel leads to valve "),
            )),
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

#[derive(Debug)]
struct CavePath<'a> {
    name: &'a str,
    time: u32,
    flow_rate: u32,
}

fn cave_paths<'a>(
    cave_map: &'a HashMap<&'a str, ValveCave<'a>>,
) -> HashMap<&'a str, Vec<CavePath<'a>>> {
    let usable_valves = cave_map
        .iter()
        .filter_map(|(name, valve)| {
            if *name == "AA" || valve.flow_rate > 0 {
                Some(name)
            } else {
                None
            }
        })
        .collect_vec();

    let mut all_paths = HashMap::new();
    for cave_name in usable_valves {
        let mut distances_to = vec![];
        let mut visited = HashSet::new();
        let mut queue = LinkedList::new();
        queue.push_back((*cave_name, 0u32));
        visited.insert(*cave_name);
        while let Some((next, dist)) = queue.pop_front() {
            let valve = cave_map.get(next).unwrap();
            if valve.flow_rate > 0 {
                distances_to.push((next, dist + 1, valve));
            }
            for n in &valve.neighbors {
                if !visited.contains(n) {
                    queue.push_back((n, dist + 1));
                    visited.insert(n);
                }
            }
        }
        let mut paths = vec![];
        for (n, d, v) in distances_to {
            paths.push(CavePath {
                name: n,
                time: d,
                flow_rate: v.flow_rate,
            });
        }
        all_paths.insert(*cave_name, paths);
    }

    all_paths
}

#[test]
fn test_cave_paths() {
    let (example, _) = &Part1::get_examples()[0];
    let cave_map = parse_input(example);
    println!("{:?}", cave_map);
    let paths = cave_paths(&cave_map);
    println!("{:?}", paths);
}

fn most_pressure_released<'a>(
    current_location: &'a str,
    cave_paths: &HashMap<&'a str, Vec<CavePath<'a>>>,
    minutes_elapsed: u32,
    pressure_so_far: u32,
    visited: &mut Vec<&'a str>,
) -> u32 {
    let mut max_so_far = pressure_so_far;
    for CavePath {
        name,
        time,
        flow_rate,
    } in cave_paths.get(&current_location).unwrap()
    {
        let new_time = minutes_elapsed + time;
        if new_time < 30 && !visited.contains(name) {
            visited.push(name);
            let new_pressure = pressure_so_far + (flow_rate * (30 - new_time));
            let pressure_released =
                most_pressure_released(name, cave_paths, new_time, new_pressure, visited);
            visited.pop();
            max_so_far = max_so_far.max(pressure_released);
        }
    }
    max_so_far
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let cave_map = parse_input(input);
        let paths = cave_paths(&cave_map);
        let mut visited = vec![];
        most_pressure_released("AA", &paths, 0, 0, &mut visited).to_string()
    }
}

fn most_pressure_released_p2<'a>(
    location1: &'a str,
    minutes1: u32,
    location2: &'a str,
    minutes2: u32,
    pressure_so_far: u32,
    cave_paths: &HashMap<&'a str, Vec<CavePath<'a>>>,
    visited: &mut Vec<&'a str>,
) -> u32 {
    let mut max_so_far = pressure_so_far;
    for CavePath {
        name,
        time,
        flow_rate,
    } in cave_paths.get(&location1).unwrap()
    {
        let new_time = minutes1 + time;
        if new_time < 26 && !visited.contains(name) {
            visited.push(name);
            let new_pressure = pressure_so_far + (flow_rate * (26 - new_time));
            let (location1, minutes1, location2, minutes2) = if new_time < minutes2 {
                (*name, new_time, location2, minutes2)
            } else {
                (location2, minutes2, *name, new_time)
            };
            let pressure_released = most_pressure_released_p2(
                location1,
                minutes1,
                location2,
                minutes2,
                new_pressure,
                cave_paths,
                visited,
            );
            visited.pop();
            max_so_far = max_so_far.max(pressure_released);
        }
    }
    max_so_far
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let cave_map = parse_input(input);
        let paths = cave_paths(&cave_map);
        let mut visited = vec![];
        most_pressure_released_p2("AA", 0, "AA", 0, 0, &paths, &mut visited).to_string()
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
