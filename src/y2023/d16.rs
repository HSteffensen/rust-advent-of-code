use std::collections::HashSet;

use itertools::Itertools;

use crate::common::{
    grid::{Direction, SquareGrid},
    solution::AocSolution,
};

struct Part1 {}
struct Part2 {}

#[derive(Debug)]
enum MirrorSquare {
    Empty,
    VerticalSplitter,
    HorizontalSplitter,
    DiagonalDownMirror,
    DiagonalUpMirror,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct LightBeam {
    x: usize,
    y: usize,
    direction: Direction,
}

impl LightBeam {
    fn travel_grid(&self, grid: &SquareGrid<MirrorSquare>) -> Vec<LightBeam> {
        if !grid.contains(self.x, self.y) {
            return vec![];
        }
        match grid.get(self.x, self.y) {
            Some(mirror) => match mirror {
                MirrorSquare::Empty => vec![self.direction],
                MirrorSquare::VerticalSplitter => match self.direction {
                    Direction::Up | Direction::Down => vec![self.direction],
                    Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
                },
                MirrorSquare::HorizontalSplitter => match self.direction {
                    Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
                    Direction::Left | Direction::Right => vec![self.direction],
                },
                MirrorSquare::DiagonalDownMirror => match self.direction {
                    Direction::Up => vec![Direction::Left],
                    Direction::Down => vec![Direction::Right],
                    Direction::Left => vec![Direction::Up],
                    Direction::Right => vec![Direction::Down],
                },
                MirrorSquare::DiagonalUpMirror => match self.direction {
                    Direction::Up => vec![Direction::Right],
                    Direction::Down => vec![Direction::Left],
                    Direction::Left => vec![Direction::Down],
                    Direction::Right => vec![Direction::Up],
                },
            },
            None => vec![],
        }
        .into_iter()
        .flat_map(|d| {
            grid.travel(self.x, self.y, d)
                .map(|(x, y)| LightBeam { x, y, direction: d })
        })
        .collect_vec()
    }
}

fn parse_input(input: &str) -> SquareGrid<MirrorSquare> {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let data = input
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| match c {
            '.' => MirrorSquare::Empty,
            '|' => MirrorSquare::VerticalSplitter,
            '-' => MirrorSquare::HorizontalSplitter,
            '\\' => MirrorSquare::DiagonalDownMirror,
            '/' => MirrorSquare::DiagonalUpMirror,
            _ => unreachable!(),
        })
        .collect_vec();
    SquareGrid {
        width,
        height,
        data,
    }
}

fn energized_count(
    grid: &SquareGrid<MirrorSquare>,
    x: usize,
    y: usize,
    direction: Direction,
) -> usize {
    let mut light_beams = vec![LightBeam { x, y, direction }];
    let mut visited: HashSet<LightBeam> = HashSet::new();
    while !light_beams.is_empty() {
        light_beams.iter().for_each(|l| {
            visited.insert(*l);
        });
        light_beams = light_beams
            .into_iter()
            .flat_map(|l| l.travel_grid(grid))
            .filter(|l| !visited.contains(l))
            .collect_vec();
    }
    visited.into_iter().map(|l| (l.x, l.y)).unique().count()
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let grid = parse_input(input);
        energized_count(&grid, 0, 0, Direction::Right).to_string()
    }
}

#[allow(dead_code)]
fn print_light_path(grid: SquareGrid<MirrorSquare>, visited: &HashSet<LightBeam>) {
    for y in 0..grid.height {
        for x in 0..grid.width {
            match grid.get(x, y) {
                Some(MirrorSquare::Empty) => {
                    let light_visited = visited
                        .iter()
                        .filter(|l| l.x == x && l.y == y)
                        .collect_vec();
                    match light_visited.len() {
                        0 => print!("."),
                        1 => print!("{}", light_visited[0].direction),
                        c => print!("{}", c),
                    }
                }
                Some(MirrorSquare::VerticalSplitter) => print!("|"),
                Some(MirrorSquare::HorizontalSplitter) => print!("-"),
                Some(MirrorSquare::DiagonalDownMirror) => print!("\\"),
                Some(MirrorSquare::DiagonalUpMirror) => print!("/"),
                None => unreachable!(),
            }
        }
        println!();
    }
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let grid = parse_input(input);
        (0..grid.height)
            .flat_map(|y| {
                [
                    energized_count(&grid, 0, y, Direction::Right),
                    energized_count(&grid, grid.width - 1, y, Direction::Left),
                ]
            })
            .chain((0..grid.width).flat_map(|x| {
                [
                    energized_count(&grid, x, 0, Direction::Down),
                    energized_count(&grid, x, grid.height - 1, Direction::Up),
                ]
            }))
            .max()
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
