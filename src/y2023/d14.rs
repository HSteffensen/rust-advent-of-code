use std::{collections::HashSet, fmt::Display, mem::swap};

use itertools::Itertools;

use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

struct RocksGrid {
    round_rocks: HashSet<(usize, usize)>,
    square_rocks: HashSet<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Display for RocksGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &(0..self.height)
                .map(|y| {
                    (0..self.width)
                        .map(|x| {
                            let p = (x, y);
                            let r = self.round_rocks.contains(&p);
                            let s = self.square_rocks.contains(&p);
                            if r && s {
                                '!'
                            } else if r {
                                'O'
                            } else if s {
                                '#'
                            } else {
                                '.'
                            }
                        })
                        .join("")
                })
                .join("\n"),
        )
    }
}

impl RocksGrid {
    fn collision(&self, p: (usize, usize)) -> bool {
        self.round_rocks.contains(&p) || self.square_rocks.contains(&p)
    }

    fn tilt_north(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.round_rocks.remove(&(x, y)) {
                    let mut placed = false;
                    'move_rock: for dy in 0..y {
                        if self.collision((x, y - dy - 1)) {
                            self.round_rocks.insert((x, y - dy));
                            placed = true;
                            break 'move_rock;
                        }
                    }
                    if !placed {
                        self.round_rocks.insert((x, 0));
                    }
                }
            }
        }
    }

    fn count_load(&self) -> usize {
        self.round_rocks.iter().map(|(_, y)| self.height - y).sum()
    }

    fn rotate_clockwise(&mut self) {
        self.round_rocks = HashSet::from_iter(
            self.round_rocks
                .iter()
                .map(|(x, y)| (self.height - y - 1, *x)),
        );
        self.square_rocks = HashSet::from_iter(
            self.square_rocks
                .iter()
                .map(|(x, y)| (self.height - y - 1, *x)),
        );
        swap(&mut self.width, &mut self.height);
    }
}

fn parse_input(input: &str) -> RocksGrid {
    let mut round_rocks = HashSet::new();
    let mut square_rocks = HashSet::new();
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                'O' => {
                    round_rocks.insert((x, y));
                }
                '#' => {
                    square_rocks.insert((x, y));
                }
                _ => {}
            };
        }
    }

    RocksGrid {
        round_rocks,
        square_rocks,
        width,
        height,
    }
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let mut rocks = parse_input(input);
        rocks.tilt_north();
        rocks.count_load().to_string()
    }
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let mut rocks = parse_input(input);
        let turns = 1000;
        for _i in 0..turns {
            rocks.tilt_north(); // north
            rocks.rotate_clockwise();
            rocks.tilt_north(); // west
            rocks.rotate_clockwise();
            rocks.tilt_north(); // south
            rocks.rotate_clockwise();
            rocks.tilt_north(); // east
            rocks.rotate_clockwise();
        }
        println!("Final:\n{}", rocks);
        rocks.count_load().to_string()
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
