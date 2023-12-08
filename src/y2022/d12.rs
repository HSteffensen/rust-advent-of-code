use std::collections::{HashMap, HashSet, LinkedList};

use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

type Point2d = (i32, i32);

struct HeightGrid {
    grid: HashMap<Point2d, u8>,
    start: Point2d,
    end: Point2d,
}

impl HeightGrid {
    fn neighbors(&self, (px, py): Point2d) -> Vec<Point2d> {
        let mut result = vec![];
        if let Some(p_height) = self.grid.get(&(px, py)) {
            for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let new_p = (px + dx, py + dy);
                if let Some(height) = self.grid.get(&new_p) {
                    if *height <= p_height + 1 {
                        result.push(new_p);
                    }
                }
            }
        }
        result
    }

    fn path_length_start_to_end(&self) -> u32 {
        self.path_length(self.start)
    }

    fn path_length(&self, start: Point2d) -> u32 {
        let mut queue = LinkedList::new();
        queue.push_back((start, 0));
        let mut visited: HashSet<Point2d> = HashSet::new();
        visited.insert(start);
        while let Some((p, len)) = queue.pop_front() {
            let neighbors = self.neighbors(p);
            let new_len = len + 1;
            for n in neighbors {
                if n == self.end {
                    return new_len;
                } else if !visited.contains(&n) {
                    visited.insert(n);
                    queue.push_back((n, new_len));
                }
            }
        }
        u32::MAX
    }
}

fn parse_input(input: &str) -> HeightGrid {
    let mut grid = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        for (x, c) in line.as_bytes().iter().enumerate() {
            let x = x as i32;
            grid.insert(
                (x, y),
                match c {
                    b'S' => {
                        start = (x, y);
                        b'a'
                    }
                    b'E' => {
                        end = (x, y);
                        b'z'
                    }
                    _ => *c,
                } - b'a',
            );
        }
    }
    HeightGrid { grid, start, end }
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let grid = parse_input(input);
        grid.path_length_start_to_end().to_string()
    }
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let grid = parse_input(input);
        (0..42)
            .filter_map(|i| grid.grid.get(&(0, i)).map(|h| ((0, i), h)))
            .filter(|(_, h)| **h == 0)
            .map(|(p, _)| grid.path_length(p))
            .min()
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
