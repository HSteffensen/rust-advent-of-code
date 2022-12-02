use std::collections::HashMap;

use regex::Regex;

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 3;

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let fabric = parse_fabric_claims(input);
        fabric
            .into_iter()
            .fold(
                0,
                |count, (_, v)| if v.len() > 1 { count + 1 } else { count },
            )
            .to_string()
    }
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(_input: &str) -> String {
        // let fabric = parse_fabric_claims(input);
        // let overlapping_claim_ids = HashSet::from_iter(
        //     fabric
        //         .iter()
        //         .filter(|(_, v)| v.len() > 1)
        //         .flat_map(|(_, v)| v),
        // );
        // let all
        todo!()
    }
}

fn parse_fabric_claims(input: &str) -> HashMap<(u32, u32), Vec<u32>> {
    let mut fabric = HashMap::new();
    let re = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)").unwrap();
    for matches in re.captures_iter(input) {
        let id: u32 = matches[1].parse().unwrap();
        let x_0: u32 = matches[2].parse().unwrap();
        let y_0: u32 = matches[3].parse().unwrap();
        let dx: u32 = matches[4].parse().unwrap();
        let dy: u32 = matches[5].parse().unwrap();

        for x in x_0..(x_0 + dx) {
            for y in y_0..(y_0 + dy) {
                fabric
                    .entry((x, y))
                    .and_modify(|v: &mut Vec<u32>| v.push(id))
                    .or_insert_with(|| vec![id]);
            }
        }
    }
    fabric
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
