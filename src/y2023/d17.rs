use std::collections::{BinaryHeap, HashMap, HashSet};

use itertools::Itertools;
use nom::{
    character::complete::{self, newline, one_of},
    combinator::map,
    error::Error,
    multi::{many1, separated_list1},
};

use crate::common::{
    grid::{Direction, SquareGrid},
    solution::AocSolution,
};

struct Part1 {}
struct Part2 {}

fn parse_input(input: &str) -> SquareGrid<u64> {
    let data = separated_list1(
        newline::<_, Error<_>>,
        many1(map(one_of("0123456789"), |c| {
            c.to_digit(10).unwrap() as u64
        })),
    )(input)
    .unwrap()
    .1;
    SquareGrid {
        width: data[0].len(),
        height: data.len(),
        data: data.into_iter().flatten().collect_vec(),
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct CrucibleStep {
    x: usize,
    y: usize,
    direction: Direction,
    steps_same_direction: u64,
    coolness: u64,
}

impl PartialOrd for CrucibleStep {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CrucibleStep {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.coolness.cmp(&other.coolness) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => std::cmp::Ordering::Equal,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Less,
        }
    }
}

fn best_crucible_path_coolness(grid: &SquareGrid<u64>) -> u64 {
    let mut queue = BinaryHeap::new();
    let mut visited: HashSet<(usize, usize, Direction, u64)> = HashSet::new();
    queue.push(CrucibleStep {
        x: 0,
        y: 0,
        direction: Direction::Right,
        steps_same_direction: 0,
        coolness: 0,
    });
    queue.push(CrucibleStep {
        x: 0,
        y: 0,
        direction: Direction::Down,
        steps_same_direction: 0,
        coolness: 0,
    });
    visited.extend(
        queue
            .iter()
            .map(|s| (s.x, s.y, s.direction, s.steps_same_direction)),
    );
    let worst_case: u64 = (0..grid.width)
        .map(|x| grid.get(x, x).unwrap())
        .sum::<u64>()
        + (1..grid.height)
            .map(|x| grid.get(x, x - 1).unwrap())
            .sum::<u64>();
    while let Some(step) = queue.pop() {
        if step.x == grid.width - 1 && step.y == grid.height - 1 {
            return step.coolness;
        }
        let directions = match step.direction {
            Direction::Up | Direction::Down => [Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => [Direction::Up, Direction::Down],
        };
        for direction in directions {
            let next_steps = (1..4)
                .filter_map(|i| crucible_travel(grid, &step, direction, i))
                .filter(|s| {
                    s.coolness <= worst_case
                        && !visited.contains(&(s.x, s.y, s.direction, s.steps_same_direction))
                })
                .collect_vec();
            visited.extend(
                next_steps
                    .iter()
                    .map(|s| (s.x, s.y, s.direction, s.steps_same_direction)),
            );
            queue.extend(next_steps.into_iter());
        }
    }
    unreachable!()
}

fn crucible_travel(
    grid: &SquareGrid<u64>,
    from: &CrucibleStep,
    new_direction: Direction,
    steps: usize,
) -> Option<CrucibleStep> {
    let mut next_step = from.clone();
    next_step.steps_same_direction = 0;
    next_step.direction = new_direction;
    for _ in 0..steps {
        if let Some((x, y)) = grid.travel(next_step.x, next_step.y, next_step.direction) {
            next_step.x = x;
            next_step.y = y;
            next_step.steps_same_direction += 1;
            next_step.coolness += grid.get(x, y).unwrap();
        } else {
            return None;
        }
    }
    Some(next_step)
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let grid = parse_input(input);
        best_crucible_path_coolness(&grid).to_string()
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
