use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 9;

fn move_tail((hx, hy): (i32, i32), (tx, ty): (i32, i32)) -> (i32, i32) {
    let dir_x = (tx - hx).signum();
    let dir_y = (ty - hy).signum();
    let dx = (tx + dir_x - hx).signum();
    let dy = (ty + dir_y - hy).signum();
    (tx + dx, ty + dy)
}

fn move_head() {}

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
