use std::collections::HashMap;

use nom::{
    bytes::complete::tag, character::complete::newline, multi::separated_list1,
    sequence::separated_pair, IResult,
};

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 14;

type Point2d = (i64, i64);
enum Block {
    Sand,
    Rock,
}

struct RockGrid {
    grid: HashMap<Point2d, Block>,
    sand_source: Point2d,
    lowest_rock: i64,
}

fn parse_rock_position(input: &str) -> IResult<&str, Point2d> {
    separated_pair(
        nom::character::complete::i64,
        tag(","),
        nom::character::complete::i64,
    )(input)
}

fn parse_rock_path(input: &str) -> IResult<&str, Vec<Point2d>> {
    separated_list1(tag(" -> "), parse_rock_position)(input)
}

fn parse_rock_paths(input: &str) -> IResult<&str, Vec<Vec<Point2d>>> {
    separated_list1(newline, parse_rock_path)(input)
}

fn parse_input(input: &str) -> RockGrid {
    let (_, rock_paths) = parse_rock_paths(input).unwrap();

    todo!()
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
