use std::collections::HashSet;

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 9;

fn move_tail((hx, hy): (i32, i32), (tx, ty): (i32, i32)) -> (i32, i32) {
    let distance_x = hx - tx;
    let distance_y = hy - ty;
    let total_distance = distance_x.abs().max(distance_y.abs());
    let dx = if total_distance.abs() > 1 {
        distance_x.signum()
    } else {
        0
    };
    let dy = if total_distance.abs() > 1 {
        distance_y.signum()
    } else {
        0
    };
    (tx + dx, ty + dy)
}

fn move_head_command(
    command: &str,
    visited: &mut HashSet<(i32, i32)>,
    (hx, hy): (i32, i32),
    tail_pos: (i32, i32),
) -> ((i32, i32), (i32, i32)) {
    let (direction, amount) = command.split_once(' ').unwrap();
    let (dx, dy) = match direction {
        "U" => (0, 1),
        "D" => (0, -1),
        "L" => (-1, 0),
        "R" => (1, 0),
        _ => unreachable!(),
    };
    let mut hx = hx;
    let mut hy = hy;
    let mut tail_pos = tail_pos;
    for _ in 0..amount.parse().unwrap() {
        hx += dx;
        hy += dy;
        tail_pos = move_tail((hx, hy), tail_pos);
        visited.insert(tail_pos);
    }
    ((hx, hy), tail_pos)
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut head_pos = (0, 0);
        let mut tail_pos = head_pos;
        visited.insert(tail_pos);
        for line in input.lines() {
            (head_pos, tail_pos) = move_head_command(line, &mut visited, head_pos, tail_pos);
        }
        visited.len().to_string()
    }
}

fn move_rope(command: &str, visited: &mut HashSet<(i32, i32)>, rope: &mut Vec<(i32, i32)>) {
    let (direction, amount) = command.split_once(' ').unwrap();
    let (dx, dy) = match direction {
        "U" => (0, 1),
        "D" => (0, -1),
        "L" => (-1, 0),
        "R" => (1, 0),
        _ => unreachable!(),
    };
    for _ in 0..amount.parse().unwrap() {
        rope[0].0 += dx;
        rope[0].1 += dy;
        for i in 1..rope.len() {
            rope[i] = move_tail(rope[i - 1], rope[i]);
        }
        visited.insert(*rope.last().unwrap());
    }
}

struct Part1Impl2;

impl AocSolution for Part1Impl2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut rope = vec![(0, 0), (0, 0)];
        visited.insert((0, 0));
        for line in input.lines() {
            move_rope(line, &mut visited, &mut rope);
        }
        visited.len().to_string()
    }
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let mut visited: HashSet<(i32, i32)> = HashSet::new();
        let mut rope = vec![
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
            (0, 0),
        ];
        visited.insert((0, 0));
        for line in input.lines() {
            move_rope(line, &mut visited, &mut rope);
        }
        visited.len().to_string()
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
fn p1_impl2_run() {
    Part1::solve();
}

#[test]
fn p2_run() {
    Part2::solve();
}
