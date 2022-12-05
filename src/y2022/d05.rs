use regex::Regex;

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 5;

struct MoveOrder {
    amount: u32,
    from_stack_id: usize,
    to_stack_id: usize,
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<MoveOrder>) {
    let (input_stacks, input_moves) = input.split_once("\n\n").unwrap();
    (parse_stacks(input_stacks), parse_moves(input_moves))
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    todo!()
}

fn parse_moves(input: &str) -> Vec<MoveOrder> {
    let mut moves = Vec::new();
    let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for matches in re.captures_iter(input) {
        let a: u32 = matches[1].parse().unwrap();
        let b: usize = matches[2].parse().unwrap();
        let c: usize = matches[3].parse().unwrap();
        moves.push(MoveOrder {
            amount: a,
            from_stack_id: b,
            to_stack_id: c,
        });
    }
    moves
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let (mut stacks, mut moves) = parse_input(input);
        for move_order in moves {
            for _ in 0..move_order.amount {
                let item = stacks
                    .iter()
                    .nth(move_order.from_stack_id)
                    .unwrap()
                    .pop()
                    .unwrap();
                stacks
                    .iter()
                    .nth(move_order.to_stack_id)
                    .unwrap()
                    .push(item);
            }
        }
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
