use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{terminated, tuple},
    IResult,
};

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 2;

struct Game {
    id: u32,
    pulls: Vec<Pull>,
}

struct Pull {
    red: u32,
    green: u32,
    blue: u32,
}

enum ColorCubes {
    Red(u32),
    Blue(u32),
    Green(u32),
}

fn parse_colored_cubes(input: &str) -> IResult<&str, ColorCubes> {
    alt((
        map(terminated(complete::u32, tag(" green")), ColorCubes::Green),
        map(terminated(complete::u32, tag(" red")), ColorCubes::Red),
        map(terminated(complete::u32, tag(" blue")), ColorCubes::Blue),
    ))(input)
}

fn collect_pull(cubes: Vec<ColorCubes>) -> Pull {
    cubes.iter().fold(
        Pull {
            red: 0,
            green: 0,
            blue: 0,
        },
        |pull, cubes| Pull {
            red: pull.red
                + if let ColorCubes::Red(count) = cubes {
                    *count
                } else {
                    0
                },
            green: pull.green
                + if let ColorCubes::Green(count) = cubes {
                    *count
                } else {
                    0
                },
            blue: pull.blue
                + if let ColorCubes::Blue(count) = cubes {
                    *count
                } else {
                    0
                },
        },
    )
}

fn parse_pull(pull: &str) -> IResult<&str, Pull> {
    map(
        separated_list1(tag(", "), parse_colored_cubes),
        collect_pull,
    )(pull)
}

fn parse_line(line: &str) -> Game {
    map(
        tuple((
            tag("Game "),
            complete::u32,
            tag(": "),
            separated_list0(tag("; "), parse_pull),
        )),
        |(_, id, _, pulls)| Game { id, pulls },
    )(line)
    .unwrap()
    .1
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;
        input
            .lines()
            .map(parse_line)
            .filter(|game| {
                game.pulls.iter().all(|pull| {
                    pull.red <= max_red && pull.green <= max_green && pull.blue <= max_blue
                })
            })
            .map(|game| game.id)
            .sum::<u32>()
            .to_string()
    }
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        input
            .lines()
            .map(parse_line)
            .map(|game| {
                game.pulls.iter().fold(
                    Pull {
                        red: 0,
                        green: 0,
                        blue: 0,
                    },
                    |a, b| Pull {
                        red: a.red.max(b.red),
                        green: a.green.max(b.green),
                        blue: a.blue.max(b.blue),
                    },
                )
            })
            .map(|cubes| cubes.red * cubes.green * cubes.blue)
            .sum::<u32>()
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
