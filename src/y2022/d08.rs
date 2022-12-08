use itertools::Itertools;

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 8;

struct Tree {
    height: u32,
    visible_north: bool,
    visible_west: bool,
    visible_south: bool,
    visible_east: bool,
}

type TreeGrid = Vec<Vec<Tree>>;

fn parse_input(input: &str) -> TreeGrid {
    let mut result = vec![];
    for line in input.lines() {
        let mut line_result = vec![];
        for c in line.chars() {
            let height = c.to_digit(10).unwrap();
            let tree = Tree {
                height,
                visible_west: false,
                visible_south: false,
                visible_east: false,
                visible_north: false,
            };
            line_result.push(tree);
        }
        result.push(line_result);
    }
    result
}

fn set_visibilities(trees: &mut TreeGrid) {
    for row in trees.iter_mut() {
        let mut sun_height = 0;
        for (i, tree) in row.iter_mut().enumerate() {
            if i == 0 || tree.height > sun_height {
                tree.visible_east = true;
                sun_height = tree.height;
            }
        }
    }
    for row in trees.iter_mut() {
        let mut sun_height = 0;
        for (i, tree) in row.iter_mut().rev().enumerate() {
            if i == 0 || tree.height > sun_height {
                tree.visible_east = true;
                sun_height = tree.height;
            }
        }
    }
    let col_count = trees.iter().next().unwrap().len();
    for col_i in 0..col_count {
        let col = trees.iter_mut().flatten().skip(col_i).step_by(col_count);
        let mut sun_height = 0;
        for (i, tree) in col.enumerate() {
            if i == 0 || tree.height > sun_height {
                tree.visible_north = true;
                sun_height = tree.height;
            }
        }
    }
    for col_i in 0..col_count {
        let col = trees
            .iter_mut()
            .rev()
            .flatten()
            .skip(col_i)
            .step_by(col_count);
        let mut sun_height = 0;
        for (i, tree) in col.enumerate() {
            if i == 0 || tree.height > sun_height {
                tree.visible_north = true;
                sun_height = tree.height;
            }
        }
    }
}

fn is_visible(tree: &Tree) -> bool {
    tree.visible_north || tree.visible_west || tree.visible_south || tree.visible_east
}

#[test]
fn test_visibilities() {
    let examples = Part1::get_examples();
    let (example, _) = examples.first().unwrap();
    let mut trees = parse_input(example);
    {
        let trees = &trees;
        println!("tree:");
        for row in trees {
            let row_str: String = row.iter().map(|tree| tree.height.to_string()).collect();
            println!("{}", row_str);
        }
    };
    set_visibilities(&mut trees);
    {
        let trees = &trees;
        println!("tree visibilities:");
        for row in trees {
            let row_str: String = row
                .iter()
                .map(|tree| if is_visible(tree) { '*' } else { '.' })
                .collect();
            println!("{}", row_str);
        }
    };
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let mut trees = parse_input(input);
        set_visibilities(&mut trees);
        trees
            .iter()
            .flatten()
            .filter(|tree| is_visible(tree))
            .count()
            .to_string()
    }
}

fn get_scenic_score(trees: &TreeGrid, x: usize, y: usize) -> u32 {
    let row = trees.get(y).unwrap();
    let height = trees.len();
    let width = row.len();
    let col = trees.iter().flatten().skip(x).step_by(width).collect_vec();
    let chosen_tree_height = row.get(x).unwrap().height;
    let mut north_vis = 0;
    let mut west_vis = 0;
    let mut south_vis = 0;
    let mut east_vis = 0;
    for tree in row.iter().skip(x + 1) {
        west_vis += 1;
        if tree.height >= chosen_tree_height {
            break;
        }
    }
    for tree in row.iter().rev().skip(width - x) {
        east_vis += 1;
        if tree.height >= chosen_tree_height {
            break;
        }
    }
    for tree in col.iter().skip(y + 1) {
        south_vis += 1;
        if tree.height >= chosen_tree_height {
            break;
        }
    }
    for tree in col.iter().rev().skip(height - y) {
        north_vis += 1;
        if tree.height >= chosen_tree_height {
            break;
        }
    }
    north_vis * west_vis * south_vis * east_vis
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let trees = parse_input(input);
        let height = trees.len();
        let width = trees.get(0).unwrap().len();
        let mut result = 0;
        for y in 0..height {
            for x in 0..width {
                let score = get_scenic_score(&trees, x, y);
                result = result.max(score);
            }
        }
        result.to_string()
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
