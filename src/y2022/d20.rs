use std::{collections::LinkedList, fmt::Display};

use itertools::Itertools;

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 20;

#[derive(Debug)]
struct CircleListNode {
    value: i32,
    next: usize,
    prev: usize,
}

#[derive(Debug)]
struct CircleList {
    array: Vec<CircleListNode>,
    current_index: usize,
}

impl Display for CircleList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("CircleList{:?}", self.to_vec()))
    }
}

impl From<Vec<i32>> for CircleList {
    fn from(list: Vec<i32>) -> Self {
        let size = list.len();
        CircleList {
            array: list
                .into_iter()
                .enumerate()
                .map(|(i, v)| CircleListNode {
                    value: v,
                    next: (i + 1) % size,
                    prev: (i + size - 1) % size,
                })
                .collect_vec(),
            current_index: 0,
        }
    }
}

impl CircleList {
    fn mix(&mut self) {
        let start_index = self.current_index;
        let size = self.array.len();
        for i in 0..size {
            self.mix_step((start_index + i) % size);
        }
    }

    fn mix_step(&mut self, mixing_index: usize) {
        let next_index = self.array[mixing_index].next;
        let prev_index = self.array[mixing_index].prev;

        // remove current item
        self.array[next_index].prev = prev_index;
        self.array[prev_index].next = next_index;

        let mut move_index = next_index;
        let mut moves = self.array[mixing_index].value;
        while moves != 0 {
            if moves > 0 {
                move_index = self.array[move_index].next;
                moves -= 1;
            } else {
                move_index = self.array[move_index].prev;
                moves += 1;
            }
        }

        // insert before move_index
        let move_prev_index = self.array[move_index].prev;
        self.array[mixing_index].next = move_index;
        self.array[mixing_index].prev = move_prev_index;
        self.array[move_prev_index].next = mixing_index;
        self.array[move_index].prev = mixing_index;

        // the front of the list has moved
        if self.current_index == mixing_index && self.array[self.current_index].value != 0 {
            self.current_index = next_index;
        }
    }

    fn to_vec(&self) -> Vec<i32> {
        let size = self.array.len();
        let mut result = vec![];
        let mut item = &self.array[self.current_index];
        for _ in 0..size {
            result.push(item.value);
            item = &self.array[item.next];
        }
        result
    }
}

#[test]
fn test_mix() {
    let mut arr = CircleList::from(vec![1, 2, -3, 3, -2, 0, 4]);
    println!("{}", arr);
    arr.mix();
    println!("{}", arr);
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let input_list = input
            .lines()
            .map(|l| str::parse::<i32>(l).unwrap())
            .collect_vec();
        let mut list = CircleList::from(input_list);
        list.mix();
        let result = list.to_vec();
        let zero_index = result
            .iter()
            .enumerate()
            .find_map(|(i, v)| if v == &0 { Some(i) } else { None })
            .unwrap();
        let size = result.len();
        (result[(zero_index + 1000) % size]
            + result[(zero_index + 2000) % size]
            + result[(zero_index + 3000) % size])
            .to_string()
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
