use std::str::Lines;

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 7;

enum FsObject {
    Dir(String, Vec<FsObject>, usize),
    File(String, usize),
}

fn parse_input(input: &str) -> FsObject {
    let mut lines = input.lines();
    assert_eq!(lines.next().unwrap(), "$ cd /");
    parse_dir_input("/", &mut lines)
}

fn parse_dir_input(dir_name: &str, lines: &mut Lines) -> FsObject {
    assert_eq!(lines.next().unwrap(), "$ ls");
    let mut inner_fs_objects = Vec::new();
    while let Some(line) = lines.next() {
        if line == "$ cd .." {
            break;
        } else if line.starts_with("$ cd ") {
            let subdir = parse_dir_input(line.strip_prefix("$ cd ").unwrap(), lines);
            inner_fs_objects.push(subdir);
        } else {
            let (size, name) = line.split_once(' ').unwrap();
            if size != "dir" {
                let file = FsObject::File(name.to_owned(), size.parse().unwrap());
                inner_fs_objects.push(file);
            }
        }
    }
    let dir_size = inner_fs_objects
        .iter()
        .map(|item| match item {
            FsObject::Dir(_, _, s) => s,
            FsObject::File(_, s) => s,
        })
        .sum();
    FsObject::Dir(dir_name.to_owned(), inner_fs_objects, dir_size)
}

fn sum_objects_at_most(limit: usize, object: &FsObject) -> usize {
    match object {
        FsObject::Dir(_, contents, s) => {
            let contribution = if *s <= limit { *s } else { 0 };
            contribution
                + contents
                    .iter()
                    .map(|o| sum_objects_at_most(limit, o))
                    .sum::<usize>()
        }
        FsObject::File(_, _) => 0,
    }
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        sum_objects_at_most(100000, &parse_input(input)).to_string()
    }
}

fn smallest_dir_at_least(
    min_size: usize,
    mut best_so_far: usize,
    object: &FsObject,
) -> Option<usize> {
    match object {
        FsObject::Dir(_, contents, s) => {
            best_so_far = if *s >= min_size && *s < best_so_far {
                *s
            } else {
                best_so_far
            };
            for obj in contents {
                if let Some(result) = smallest_dir_at_least(min_size, best_so_far, obj) {
                    best_so_far = result;
                }
            }
            Some(best_so_far)
        }
        FsObject::File(_, _) => None,
    }
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let rootdir = parse_input(input);
        let FsObject::Dir(_, _, rootdir_size) = rootdir else {unreachable!()};
        let fs_size = 70000000;
        let needed_space = 30000000;
        let unused_space = fs_size - rootdir_size;
        let space_to_free = needed_space - unused_space;
        let Some(answer) = smallest_dir_at_least(space_to_free, rootdir_size, &rootdir) else {unreachable!()};
        answer.to_string()
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

struct Part1Impl2 {}
struct Part2Impl2 {}

fn directory_sizes<'a>(input: &mut impl Iterator<Item = &'a str>) -> Vec<u64> {
    let mut subdirs = Vec::new();
    let mut dir_size = 0;
    while let Some(line) = input.next() {
        let line: Vec<&str> = line.split_whitespace().collect();
        match line[..] {
            ["$", "cd", ".."] => break,
            ["$", "cd", _] => {
                let mut subdir_sizes = directory_sizes(input);
                dir_size += subdir_sizes.last().unwrap();
                subdirs.append(&mut subdir_sizes);
            }
            ["$", "ls"] => (),
            ["dir", _] => (),
            [size, _] => {
                dir_size += size.parse::<u64>().unwrap();
            }
            _ => (),
        }
    }
    subdirs.push(dir_size);
    subdirs
}

impl AocSolution for Part1Impl2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let sizes = directory_sizes(&mut input.lines());
        println!("{:?}", &sizes);
        sizes
            .iter()
            .filter(|size| **size <= 100000)
            .sum::<u64>()
            .to_string()
    }
}

impl AocSolution for Part2Impl2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let sizes = directory_sizes(&mut input.lines());
        let rootdir_size = sizes.iter().max().unwrap();
        let fs_size = 70000000;
        let needed_space = 30000000;
        let unused_space = fs_size - rootdir_size;
        let space_to_free = needed_space - unused_space;
        sizes
            .iter()
            .filter(|size| **size >= space_to_free)
            .min()
            .unwrap()
            .to_string()
    }
}

#[test]
fn p1_impl2_run() {
    Part1Impl2::solve();
}

#[test]
fn p2_impl2_run() {
    Part2Impl2::solve();
}
