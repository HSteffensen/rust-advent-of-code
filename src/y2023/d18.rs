use std::collections::HashSet;

use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take},
    character::complete::{self, newline, one_of, space1},
    combinator::map,
    error::Error,
    multi::separated_list1,
    sequence::{delimited, tuple},
};

use crate::common::{
    grid::{Direction, SquareGrid},
    solution::AocSolution,
};

struct Part1 {}
struct Part2 {}

struct DigInstruction<'a> {
    direction: Direction,
    distance: usize,
    color: &'a str,
}

fn parse_input(input: &str) -> Vec<DigInstruction> {
    separated_list1(
        newline::<_, Error<_>>,
        map(
            tuple((
                one_of("UDLR"),
                space1,
                complete::u32,
                space1,
                delimited(tag("("), take(7usize), tag(")")),
            )),
            |(direction, _, distance, _, color)| DigInstruction {
                direction: match direction {
                    'U' => Direction::Up,
                    'D' => Direction::Down,
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => unreachable!(),
                },
                distance: distance as usize,
                color,
            },
        ),
    )(input)
    .unwrap()
    .1
}

fn dig<'a>(instructions: &'a [DigInstruction]) -> SquareGrid<Option<&'a str>> {
    let (ups, downs, lefts, rights): (usize, usize, usize, usize) =
        instructions
            .iter()
            .fold((0, 0, 0, 0), |(u, d, l, r), b| match b.direction {
                Direction::Up => (u + b.distance, d, l, r),
                Direction::Down => (u, d + b.distance, l, r),
                Direction::Left => (u, d, l + b.distance, r),
                Direction::Right => (u, d, l, r + b.distance),
            });
    let mut grid = SquareGrid {
        width: lefts + rights + 9,
        height: ups + downs + 9,
        data: vec![None; (lefts + rights + 9) * (ups + downs + 9)],
    };
    let mut x = lefts + 5;
    let mut y = ups + 5;
    grid.set(x, y, Some(""));
    for instruction in instructions {
        for _ in 0..instruction.distance {
            (x, y) = grid.travel(x, y, instruction.direction).unwrap();
            grid.set(x, y, Some(instruction.color));
        }
    }
    grid
}

fn count_outside<T>(grid: &SquareGrid<Option<T>>) -> usize {
    if matches!(grid.get(0, 0), Some(Some(_))) {
        unreachable!("0,0 should be empty")
    }
    let mut queue = vec![(0, 0)];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert((0, 0));
    let mut count = 0;
    while let Some((x, y)) = queue.pop() {
        if matches!(grid.get(x, y), Some(Some(_))) {
            continue;
        }
        count += 1;
        let new_positions = grid
            .neighbors_8(x, y)
            .into_iter()
            .filter(|p| !visited.contains(p))
            .collect_vec();
        queue.extend(new_positions.iter());
        visited.extend(new_positions);
    }
    count
}

fn try_count_inside<T>(
    grid: &SquareGrid<Option<T>>,
    start_pos: (usize, usize),
    fail_pos: (usize, usize),
) -> Option<usize> {
    if matches!(grid.get(start_pos.0, start_pos.1), Some(Some(_))) {
        return None;
    }
    let mut queue = vec![start_pos];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    visited.insert(start_pos);
    let mut count = 1;
    while let Some((x, y)) = queue.pop() {
        if (x, y) == fail_pos {
            return None;
        }
        if matches!(grid.get(x, y), Some(Some(_))) {
            continue;
        }
        count += 1;
        let new_positions = grid
            .neighbors_8(x, y)
            .into_iter()
            .filter(|p| !visited.contains(p))
            .collect_vec();
        queue.extend(new_positions.iter());
        visited.extend(new_positions);
    }
    Some(count)
}

fn count_inside<T>(grid: &SquareGrid<Option<T>>) -> usize {
    (grid.width * grid.height) - count_outside(grid)
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let instructions = &parse_input(input);
        let dug = dig(instructions);
        count_inside(&dug).to_string()
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
