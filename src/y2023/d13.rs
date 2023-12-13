use itertools::Itertools;

use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

fn find_horizontal_mirror(input: &str) -> usize {
    let lines = input.lines().collect_vec();
    'outer: for i in 0..lines.len() - 1 {
        if lines[i] != lines[i + 1] {
            continue;
        }
        let possible_mirror_size = (lines.len().abs_diff(i + 1)).min(i + 1);
        for j in 1..possible_mirror_size {
            if lines[i - j] != lines[i + 1 + j] {
                continue 'outer;
            }
        }
        return i + 1;
    }
    0
}

#[test]
fn test_find_horizontal_mirror() {
    assert_eq!(
        0,
        find_horizontal_mirror(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
        )
    );
    assert_eq!(
        4,
        find_horizontal_mirror(
            "#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
        )
    );
}

fn transpose_string(input: &str) -> String {
    let mut line_iters = input.lines().map(|l| l.chars()).collect_vec();
    let width = input.lines().next().unwrap().len();
    (0..width)
        .map(|_| line_iters.iter_mut().map(|l| (*l).next().unwrap()).join(""))
        .join("\n")
}

#[test]
fn test_transpose_string() {
    assert_eq!(
        "#.##..#
..##...
##..###
#....#.
.#..#.#
.#..#.#
#....#.
##..###
..##...",
        transpose_string(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
        )
    );
    assert_eq!(
        5,
        find_horizontal_mirror(&transpose_string(
            "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#."
        ))
    );
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        input
            .split("\n\n")
            .map(|grid| {
                100 * find_horizontal_mirror(grid) + find_horizontal_mirror(&transpose_string(grid))
            })
            .sum::<usize>()
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
