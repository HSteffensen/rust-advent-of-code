use nom::{
    branch::alt,
    bytes::complete::take,
    character::complete::{self},
    combinator::{consumed, map},
    error::Error,
    multi::many1,
};

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 3;

#[derive(Debug, Clone, PartialEq, Eq)]
enum SchematicPart {
    Empty,
    Symbol(char),
    Number(u32),
}

#[derive(Debug)]
struct EngineSchematic {
    width: usize,
    height: usize,
    data: Vec<SchematicPart>,
}

impl EngineSchematic {
    fn get_pos(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn get(&self, x: usize, y: usize) -> Option<&SchematicPart> {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            self.data.iter().nth(y * self.width + x)
        } else {
            None
        }
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<&SchematicPart> {
        let top_left = if x > 0 && y > 0 {
            self.get(x - 1, y - 1)
        } else {
            None
        };
        let (left, bottom_left) = if x > 0 {
            (self.get(x - 1, y), self.get(x - 1, y + 1))
        } else {
            (None, None)
        };
        let (top, top_right) = if y > 0 {
            (self.get(x, y - 1), self.get(x + 1, y - 1))
        } else {
            (None, None)
        };
        vec![
            top_left,
            top,
            top_right,
            left,
            self.get(x + 1, y),
            bottom_left,
            self.get(x, y + 1),
            self.get(x + 1, y + 1),
        ]
        .into_iter()
        .flatten()
        .collect()
    }

    fn get_part_numbers(&self) -> Vec<u32> {
        let mut result = vec![];
        let mut i = 0;
        while i < self.data.len() {
            let (x, y) = self.get_pos(i);
            let part = &self.data[i];
            if let SchematicPart::Number(number) = part {
                let neighbors = self.get_neighbors(x, y);
                if neighbors
                    .iter()
                    .any(|n| matches!(n, SchematicPart::Symbol(_)))
                {
                    result.push(*number);
                    while i % self.width != self.width - 1 {
                        if let SchematicPart::Number(_) = &self.data[i + 1] {
                            i += 1;
                        } else {
                            break;
                        }
                    }
                }
            }
            i += 1;
        }
        result
    }
}

#[test]
fn test_get_part_numbers() {
    let schematic = EngineSchematic {
        width: 3,
        height: 3,
        data: vec![
            // row 1
            SchematicPart::Symbol('#'),
            SchematicPart::Empty,
            SchematicPart::Empty,
            // row 2
            SchematicPart::Number(1),
            SchematicPart::Empty,
            SchematicPart::Number(2),
            // row 3
            SchematicPart::Number(3),
            SchematicPart::Symbol('#'),
            SchematicPart::Empty,
        ],
    };
    println!("{:?}", schematic.get_neighbors(0, 0));
    println!("{:?}", schematic.get_neighbors(1, 0));
    println!("{:?}", schematic.get_neighbors(2, 0));
    println!("{:?}", schematic.get_neighbors(0, 1));
    println!("{:?}", schematic.get_neighbors(1, 1));
    println!("{:?}", schematic.get_neighbors(2, 1));
    println!("{:?}", schematic.get_neighbors(0, 2));
    println!("{:?}", schematic.get_neighbors(1, 2));
    println!("{:?}", schematic.get_neighbors(2, 2));
    println!("{:?}", schematic.get_part_numbers());
}

fn parse_line(line: &str) -> Vec<SchematicPart> {
    let parsed: Vec<Vec<SchematicPart>> = many1(alt((
        map(complete::char::<&str, Error<_>>('.'), |_| {
            vec![SchematicPart::Empty]
        }),
        map(consumed(complete::u32), |(n_str, n): (&str, u32)| {
            vec![SchematicPart::Number(n); n_str.len()]
        }),
        map(take(1usize), |c: &str| {
            vec![SchematicPart::Symbol(c.chars().nth(0).unwrap())]
        }),
    )))(line)
    .unwrap()
    .1;
    return parsed.into_iter().flatten().collect();
}

#[test]
fn test_parse_line() {
    println!("{:?}", parse_line("123..#.45."),)
}

fn parse_input(input: &str) -> EngineSchematic {
    let width = input.chars().take_while(|c| !c.is_whitespace()).count();
    let height = input.lines().count();
    let data = input.lines().map(parse_line).flatten().collect();
    EngineSchematic {
        width,
        height,
        data,
    }
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        parse_input(input)
            .get_part_numbers()
            .iter()
            .sum::<u32>()
            .to_string()
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
