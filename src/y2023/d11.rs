use std::ops::Div;

use itertools::Itertools;

use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

fn find_galaxies_after_expansion(input: &str, expansion_factor: usize) -> Vec<(usize, usize)> {
    let extra_size = expansion_factor - 1;
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let columns = grid[0].len();
    let expanding_columns = (0..columns)
        .filter(|x| grid.iter().all(|row| row[*x] == '.'))
        .collect_vec();
    let expanding_rows = input
        .lines()
        .enumerate()
        .filter(|(_, l)| l.chars().all(|c| c == '.'))
        .map(|(i, _)| i)
        .collect_vec();
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| if c == '#' { Some((x, y)) } else { None })
        })
        .map(|(x, y)| {
            (
                x + (expanding_columns.iter().take_while(|c| **c < x).count() * extra_size),
                y + (expanding_rows.iter().take_while(|r| **r < y).count() * extra_size),
            )
        })
        .collect_vec()
}

fn all_galaxy_distances(galaxies: &[(usize, usize)]) -> usize {
    galaxies
        .iter()
        .cartesian_product(galaxies.iter())
        .map(|((x1, y1), (x2, y2))| x1.abs_diff(*x2) + y1.abs_diff(*y2))
        .sum::<usize>()
        .div(2) // overcounted by a->b and b->a, so the sum is double the answer
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let galaxies_after_expansion = find_galaxies_after_expansion(input, 2);
        all_galaxy_distances(&galaxies_after_expansion).to_string()
    }
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let galaxies_after_expansion = find_galaxies_after_expansion(input, 1000000);
        all_galaxy_distances(&galaxies_after_expansion).to_string()
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
