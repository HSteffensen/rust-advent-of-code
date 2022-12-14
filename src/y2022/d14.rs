use std::{collections::HashMap, fmt::Display};

use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete::newline, combinator::map, multi::separated_list1,
    sequence::separated_pair, IResult,
};

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 14;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Point2d(i64, i64);

#[derive(Debug, PartialEq)]
enum Block {
    Sand,
    Rock,
}

#[derive(Debug)]
struct RockGrid {
    grid: HashMap<Point2d, Block>,
    sand_source_column: i64,
    top_boundary: i64,
    bottom_boundary: i64,
    left_boundary: i64,
    right_boundary: i64,
}

impl Default for RockGrid {
    fn default() -> Self {
        let sand_column = 500;
        Self {
            grid: Default::default(),
            sand_source_column: sand_column,
            top_boundary: Default::default(),
            bottom_boundary: Default::default(),
            left_boundary: Default::default(),
            right_boundary: Default::default(),
        }
    }
}

impl Display for RockGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut lines = vec![];
        for y in (self.top_boundary - 1)..=(self.bottom_boundary + 1) {
            let line: String = ((self.left_boundary - 1)..=(self.right_boundary + 1))
                .map(|x| match self.grid.get(&Point2d(x, y)) {
                    Some(Block::Rock) => '#',
                    Some(Block::Sand) => 'o',
                    None => '.',
                })
                .collect();
            lines.push(line);
        }
        write!(f, "{}", lines.join("\n"))
    }
}

impl RockGrid {
    fn add_rock_path(&mut self, path: Vec<Point2d>) {
        if self.grid.is_empty() && !path.is_empty() {
            self.left_boundary = path[0].0;
            self.right_boundary = path[0].0;
            self.top_boundary = path[0].1;
            self.bottom_boundary = path[0].1;
        }
        for (p1, p2) in path.iter().tuple_windows() {
            let first_x = p1.0.min(p2.0);
            let second_x = p1.0.max(p2.0);
            let first_y = p1.1.min(p2.1);
            let second_y = p1.1.max(p2.1);
            for (x, y) in (first_x..=second_x).cartesian_product(first_y..=second_y) {
                self.grid.insert(Point2d(x, y), Block::Rock);
                self.left_boundary = self.left_boundary.min(x);
                self.right_boundary = self.right_boundary.max(x);
                self.top_boundary = self.top_boundary.min(y);
                self.bottom_boundary = self.bottom_boundary.max(y);
            }
        }
    }

    fn add_sand_particle(&mut self) {
        while self
            .grid
            .get(&Point2d(self.sand_source_column, self.top_boundary))
            .is_some()
        {
            self.top_boundary -= 1;
        }
        self.grid.insert(
            Point2d(self.sand_source_column, self.top_boundary),
            Block::Sand,
        );
    }

    fn fall_sand(&mut self) -> bool {
        let mut settled = true;
        let sand_positions = self
            .grid
            .iter()
            .filter(|(_, v)| **v == Block::Sand)
            .map(|(p, _)| *p)
            .collect_vec();
        for Point2d(x, y) in sand_positions {
            if y > self.bottom_boundary {
                self.grid.remove(&Point2d(x, y));
            } else if self.grid.get(&Point2d(x, y + 1)).is_none() {
                settled = false;
                self.grid.remove(&Point2d(x, y));
                self.grid.insert(Point2d(x, y + 1), Block::Sand);
            } else if self.grid.get(&Point2d(x - 1, y + 1)).is_none() {
                settled = false;
                self.grid.remove(&Point2d(x, y));
                self.grid.insert(Point2d(x - 1, y + 1), Block::Sand);
            } else if self.grid.get(&Point2d(x + 1, y + 1)).is_none() {
                settled = false;
                self.grid.remove(&Point2d(x, y));
                self.grid.insert(Point2d(x + 1, y + 1), Block::Sand);
            }
        }
        !settled
    }

    fn count_sand(&self) -> usize {
        self.grid.iter().filter(|(_, v)| **v == Block::Sand).count()
    }

    fn drop_one_new_sand(&mut self) -> bool {
        let prev_sand_count = self.count_sand();
        self.add_sand_particle();
        while self.fall_sand() {}
        self.count_sand() != prev_sand_count
    }

    fn drop_many_new_sands(&mut self) {
        while self.drop_one_new_sand() {}
    }

    fn drop_many_new_sands_p2(&mut self) {
        self.grid.insert(Point2d(500, 0), Block::Sand);
        for y in 1..self.bottom_boundary {
            for x in self.left_boundary..self.right_boundary {
                let pos = Point2d(x, y);
                if let Some(Block::Rock) = self.grid.get(&pos) {
                    continue;
                }
                if let Some(Block::Sand) = self.grid.get(&Point2d(x, y - 1)) {
                    self.grid.insert(pos, Block::Sand);
                } else if let Some(Block::Sand) = self.grid.get(&Point2d(x - 1, y - 1)) {
                    self.grid.insert(pos, Block::Sand);
                } else if let Some(Block::Sand) = self.grid.get(&Point2d(x + 1, y - 1)) {
                    self.grid.insert(pos, Block::Sand);
                }
            }
        }
    }
}

#[test]
fn test_add_rock_path() {
    let mut grid = RockGrid::default();
    println!("{:?}", grid);
    println!("{}\n", grid);
    grid.add_rock_path(vec![Point2d(498, 4), Point2d(498, 6), Point2d(496, 6)]);
    println!("{:?}", grid);
    println!("{}\n", grid);
    grid.add_rock_path(vec![
        Point2d(503, 4),
        Point2d(502, 4),
        Point2d(502, 9),
        Point2d(494, 9),
    ]);
    println!("{:?}", grid);
    println!("{}\n", grid);
    grid.add_sand_particle();
    println!("{}\n", grid);
    assert!(grid.fall_sand());
    println!("{}\n", grid);
    assert!(grid.fall_sand());
    println!("{}\n", grid);
    assert!(grid.fall_sand());
    println!("{}\n", grid);
    assert!(grid.fall_sand());
    println!("{}\n", grid);
    assert!(!grid.fall_sand());
    println!("{}\n", grid);
    assert!(!grid.fall_sand());
    println!("{}\n", grid);
    assert!(grid.drop_one_new_sand());
    println!("{}\n", grid);
    assert!(grid.drop_one_new_sand());
    println!("{}\n", grid);
    assert!(grid.drop_one_new_sand());
    assert!(grid.drop_one_new_sand());
    assert!(grid.drop_one_new_sand());
    println!("{}\n", grid);
    grid.drop_many_new_sands();
    println!("{}\n", grid);
}

fn parse_rock_position(input: &str) -> IResult<&str, Point2d> {
    map(
        separated_pair(
            nom::character::complete::i64,
            tag(","),
            nom::character::complete::i64,
        ),
        |(x, y)| Point2d(x, y),
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
    let mut rock_grid = RockGrid::default();
    for path in rock_paths {
        rock_grid.add_rock_path(path);
    }
    rock_grid
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let mut rock_grid = parse_input(input);
        rock_grid.drop_many_new_sands();
        rock_grid.count_sand().to_string()
    }
}

fn parse_input_p2(input: &str) -> RockGrid {
    let mut rock_grid = parse_input(input);
    rock_grid.add_rock_path(vec![
        Point2d(
            rock_grid.left_boundary - rock_grid.bottom_boundary - 2,
            rock_grid.bottom_boundary + 2,
        ),
        Point2d(
            rock_grid.right_boundary + rock_grid.bottom_boundary + 2,
            rock_grid.bottom_boundary + 2,
        ),
    ]);
    rock_grid
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let mut rock_grid = parse_input_p2(input);
        rock_grid.drop_many_new_sands_p2();
        rock_grid.count_sand().to_string()
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
