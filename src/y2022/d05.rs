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

fn split_input(input: &str) -> (&str, &str) {
    let (input_stacks, input_moves) = input.split_once("\n\n").unwrap();
    (input_stacks, input_moves)
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<MoveOrder>) {
    let (input_stacks, input_moves) = split_input(input);
    (parse_stacks(input_stacks), parse_moves(input_moves))
}

fn parse_stacks(input: &str) -> Vec<Vec<char>> {
    let mut stacks = Vec::new();
    let mut lines_rev = input.lines().rev();
    let stack_numbers_line = &lines_rev.next().unwrap();
    for _ in stack_numbers_line.split_whitespace() {
        stacks.push(Vec::new());
    }
    for line in lines_rev {
        let mut chars = line.chars();
        chars.next();
        let mut chars = chars.step_by(4);
        for stack in stacks.iter_mut() {
            let c = chars.next().unwrap();
            if c != ' ' {
                stack.push(c);
            }
        }
    }
    stacks
}

#[test]
fn test_parse_stacks() {
    let examples = Part1::get_examples().unwrap();
    for (example, _) in examples {
        let (input_stacks, input_moves) = split_input(&example);
        println!("stacks---\n{}\n\nmoves---\n{}", input_stacks, input_moves);
        println!("{:?}", parse_stacks(input_stacks))
    }
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
        let (mut stacks, moves) = parse_input(input);
        for move_order in moves {
            for _ in 0..move_order.amount {
                let item = stacks
                    .get_mut(move_order.from_stack_id - 1)
                    .unwrap()
                    .pop()
                    .unwrap();
                stacks
                    .get_mut(move_order.to_stack_id - 1)
                    .unwrap()
                    .push(item);
            }
        }
        stacks.iter().map(|stack| stack.last().unwrap()).collect()
    }
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let (mut stacks, moves) = parse_input(input);
        for move_order in moves {
            let mut crane_holding = Vec::new();
            for _ in 0..move_order.amount {
                let item = stacks
                    .get_mut(move_order.from_stack_id - 1)
                    .unwrap()
                    .pop()
                    .unwrap();
                crane_holding.push(item);
            }
            for c in crane_holding.iter().rev() {
                stacks.get_mut(move_order.to_stack_id - 1).unwrap().push(*c);
            }
        }
        stacks.iter().map(|stack| stack.last().unwrap()).collect()
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
