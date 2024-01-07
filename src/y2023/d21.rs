use std::collections::HashMap;

use itertools::Itertools;
use num::Integer;

use crate::common::{grid::SquareGrid, solution::AocSolution};

struct Part1 {}
struct Part2 {}

enum GardenBlock {
    Plot,
    Rock,
}

struct Garden {
    grid: SquareGrid<GardenBlock>,
    start: (usize, usize),
}

fn neighbors(x: i64, y: i64) -> Vec<(i64, i64)> {
    vec![(x - 1, y), (x + 1, y), (x, y - 1), (x, y + 1)]
}

impl Garden {
    fn count_reachable_in_exact_steps(&self, steps: usize) -> usize {
        let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
        visited.insert(self.start, 0);
        self.count_reachable_helper(self.start, 0, &mut visited, steps);
        visited.into_values().filter(|v| v.is_even()).count()
    }

    fn count_reachable_helper(
        &self,
        (x, y): (usize, usize),
        depth: usize,
        visited: &mut HashMap<(usize, usize), usize>,
        max_depth: usize,
    ) {
        if depth == max_depth {
            return;
        }
        let new_depth = depth + 1;
        for neighbor in self.grid.neighbors_4(x, y) {
            if let Some(GardenBlock::Rock) = self.grid.get(neighbor.0, neighbor.1) {
                continue;
            }
            if let Some(prev_depth) = visited.get(&neighbor) {
                if *prev_depth <= new_depth {
                    continue;
                }
            }
            visited.insert(neighbor, new_depth);
            self.count_reachable_helper(neighbor, new_depth, visited, max_depth);
        }
    }

    fn count_reachable_in_exact_steps_infinite(&self, steps: usize) -> usize {
        let mut visited: HashMap<(i64, i64), usize> = HashMap::new();
        let (start_x, start_y) = self.start;
        let start = (start_x as i64, start_y as i64);
        visited.insert(start, 0);
        self.count_reachable_helper_infinite(start, 0, &mut visited, steps);
        visited.into_values().filter(|v| v.is_even()).count()
    }

    fn count_reachable_helper_infinite(
        &self,
        (x, y): (i64, i64),
        depth: usize,
        visited: &mut HashMap<(i64, i64), usize>,
        max_depth: usize,
    ) {
        if depth == max_depth {
            return;
        }
        let new_depth = depth + 1;
        for neighbor in neighbors(x, y) {
            if let Some(GardenBlock::Rock) =
                self.grid.get_infinitely_looping(neighbor.0, neighbor.1)
            {
                continue;
            }
            if let Some(prev_depth) = visited.get(&neighbor) {
                if *prev_depth <= new_depth {
                    continue;
                }
            }
            visited.insert(neighbor, new_depth);
            self.count_reachable_helper_infinite(neighbor, new_depth, visited, max_depth);
        }
    }
}

fn parse_input(input: &str) -> Garden {
    let width = input.lines().next().unwrap().len();
    let height = input.lines().count();
    let grid = SquareGrid {
        width,
        height,
        data: input
            .lines()
            .flat_map(|l| l.chars())
            .map(|c| match c {
                '.' | 'S' => GardenBlock::Plot,
                '#' => GardenBlock::Rock,
                _ => unreachable!(),
            })
            .collect_vec(),
    };
    let (start_y, start_line) = input
        .lines()
        .enumerate()
        .find(|(_, l)| l.contains('S'))
        .unwrap();
    let (start_x, _) = start_line
        .chars()
        .enumerate()
        .find(|(_, c)| *c == 'S')
        .unwrap();
    Garden {
        grid,
        start: (start_x, start_y),
    }
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let garden = parse_input(input);
        let search_depth = 64;
        garden
            .count_reachable_in_exact_steps(search_depth)
            .to_string()
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
fn test_p1() {
    let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    let parsed = parse_input(input);
    let search_depth = 6;
    let result = parsed.count_reachable_in_exact_steps(search_depth);
    assert_eq!(result, 16);
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
fn test_p2() {
    let input = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........";
    let parsed = parse_input(input);
    assert_eq!(parsed.count_reachable_in_exact_steps_infinite(6), 16);
    assert_eq!(parsed.count_reachable_in_exact_steps_infinite(10), 50);
    assert_eq!(parsed.count_reachable_in_exact_steps_infinite(50), 1594);
    assert_eq!(parsed.count_reachable_in_exact_steps_infinite(100), 6536);
    assert_eq!(parsed.count_reachable_in_exact_steps_infinite(500), 167004);
    assert_eq!(parsed.count_reachable_in_exact_steps_infinite(1000), 668697);
    assert_eq!(
        parsed.count_reachable_in_exact_steps_infinite(5000),
        16733044
    );
}

#[test]
fn p2_pull_examples() {
    Part2::get_examples();
}

#[test]
fn p2_run() {
    Part2::solve();
}
