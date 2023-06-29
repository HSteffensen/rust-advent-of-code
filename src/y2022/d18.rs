use std::collections::{HashSet, VecDeque};

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 18;

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point3d {
    x: i32,
    y: i32,
    z: i32,
}

struct Grid3d {
    grid: HashSet<Point3d>,
}

fn parse_line(line: &str) -> IResult<&str, Point3d> {
    map(
        tuple((
            complete::i32,
            tag(","),
            complete::i32,
            tag(","),
            complete::i32,
        )),
        |(x, _, y, _, z)| Point3d { x, y, z },
    )(line)
}

fn parse_input(input: &str) -> Grid3d {
    let (_, points) = separated_list0(newline, parse_line)(input).unwrap();
    Grid3d {
        grid: points.into_iter().collect(),
    }
}

impl Point3d {
    fn neighbors(&self) -> HashSet<Point3d> {
        let mut set = HashSet::new();
        set.insert(Point3d {
            x: self.x + 1,
            y: self.y,
            z: self.z,
        });
        set.insert(Point3d {
            x: self.x - 1,
            y: self.y,
            z: self.z,
        });
        set.insert(Point3d {
            x: self.x,
            y: self.y + 1,
            z: self.z,
        });
        set.insert(Point3d {
            x: self.x,
            y: self.y - 1,
            z: self.z,
        });
        set.insert(Point3d {
            x: self.x,
            y: self.y,
            z: self.z + 1,
        });
        set.insert(Point3d {
            x: self.x,
            y: self.y,
            z: self.z - 1,
        });
        set
    }
}

impl Grid3d {
    fn exposed_sides_of_point(&self, point: &Point3d) -> i32 {
        6 - point
            .neighbors()
            .iter()
            .filter(|n| self.grid.contains(n))
            .count() as i32
    }

    fn exposed_sides(&self) -> i32 {
        self.grid
            .iter()
            .map(|p| self.exposed_sides_of_point(p))
            .sum()
    }

    fn exposed_external_sides(&self) -> i32 {
        let mut inverse = self.inverse();
        inverse.expand_cube();
        inverse.reachable_from_corner().inverse().exposed_sides()
    }

    fn inverse(&self) -> Grid3d {
        let x_max = self.grid.iter().map(|p| p.x).max().unwrap();
        let x_min = self.grid.iter().map(|p| p.x).min().unwrap();
        let y_max = self.grid.iter().map(|p| p.y).max().unwrap();
        let y_min = self.grid.iter().map(|p| p.y).min().unwrap();
        let z_max = self.grid.iter().map(|p| p.z).max().unwrap();
        let z_min = self.grid.iter().map(|p| p.z).min().unwrap();
        let mut new_grid = HashSet::new();
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                for z in z_min..=z_max {
                    let p = Point3d { x, y, z };
                    if !self.grid.contains(&p) {
                        new_grid.insert(p);
                    }
                }
            }
        }
        Grid3d { grid: new_grid }
    }

    fn expand_cube(&mut self) {
        let x_max = self.grid.iter().map(|p| p.x).max().unwrap() + 1;
        let x_min = self.grid.iter().map(|p| p.x).min().unwrap() - 1;
        let y_max = self.grid.iter().map(|p| p.y).max().unwrap() + 1;
        let y_min = self.grid.iter().map(|p| p.y).min().unwrap() - 1;
        let z_max = self.grid.iter().map(|p| p.z).max().unwrap() + 1;
        let z_min = self.grid.iter().map(|p| p.z).min().unwrap() - 1;
        for x in x_min..=x_max {
            for y in y_min..=y_max {
                self.grid.insert(Point3d { x, y, z: z_min });
                self.grid.insert(Point3d { x, y, z: z_max });
            }
        }
        for x in x_min..=x_max {
            for z in z_min..=z_max {
                self.grid.insert(Point3d { x, y: y_min, z });
                self.grid.insert(Point3d { x, y: y_max, z });
            }
        }
        for z in z_min..=z_max {
            for y in y_min..=y_max {
                self.grid.insert(Point3d { x: x_min, y, z });
                self.grid.insert(Point3d { x: x_max, y, z });
            }
        }
    }

    fn corner(&self) -> Point3d {
        let x_min = self.grid.iter().map(|p| p.x).min().unwrap();
        let y_min = self.grid.iter().map(|p| p.y).min().unwrap();
        let z_min = self.grid.iter().map(|p| p.z).min().unwrap();
        Point3d {
            x: x_min,
            y: y_min,
            z: z_min,
        }
    }

    fn reachable_from_corner(&self) -> Grid3d {
        let mut reachable = HashSet::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(self.corner());

        while let Some(p) = queue.pop_front() {
            p.neighbors().iter().for_each(|n| {
                if !visited.contains(n) {
                    visited.insert(n.clone());
                    if self.grid.contains(n) {
                        reachable.insert(n.clone());
                        queue.push_back(n.clone());
                    }
                }
            });
        }

        Grid3d { grid: reachable }
    }
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let grid = parse_input(input);
        grid.exposed_sides().to_string()
    }
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let grid = parse_input(input);
        grid.exposed_external_sides().to_string()
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
