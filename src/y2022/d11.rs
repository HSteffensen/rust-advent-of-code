use itertools::Itertools;

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 11;

struct Monkey {
    items: Vec<u64>,
    inspection_count: usize,
    op: Box<dyn Fn(u64) -> u64>,
    test_factor: u64,
    test_pass: usize,
    test_fail: usize,
}

fn parse_monkey(input: &str) -> Monkey {
    let mut lines = input.lines();
    assert!(lines.next().unwrap().starts_with("Monkey "));
    let items_line = lines
        .next()
        .unwrap()
        .strip_prefix("  Starting items: ")
        .unwrap();
    let items = items_line
        .split(", ")
        .map(|item| item.parse().unwrap())
        .collect();
    let op_line = lines
        .next()
        .unwrap()
        .strip_prefix("  Operation: new = ")
        .unwrap();
    let op: Box<dyn Fn(u64) -> u64> = if let Some(r_arg) = op_line.strip_prefix("old + ") {
        if r_arg == "old" {
            Box::new(|x| x + x)
        } else {
            let r_val: u64 = r_arg.parse().unwrap();
            Box::new(move |x| x + r_val)
        }
    } else if let Some(r_arg) = op_line.strip_prefix("old * ") {
        if r_arg == "old" {
            Box::new(|x| x * x)
        } else {
            let r_val: u64 = r_arg.parse().unwrap();
            Box::new(move |x| x * r_val)
        }
    } else {
        unreachable!()
    };
    let test_line = lines
        .next()
        .unwrap()
        .strip_prefix("  Test: divisible by ")
        .unwrap();

    let test_factor = test_line.parse().unwrap();
    let test_pass_line = lines
        .next()
        .unwrap()
        .strip_prefix("    If true: throw to monkey ")
        .unwrap();

    let test_pass = test_pass_line.parse().unwrap();
    let test_fail_line = lines
        .next()
        .unwrap()
        .strip_prefix("    If false: throw to monkey ")
        .unwrap();

    let test_fail = test_fail_line.parse().unwrap();

    Monkey {
        items,
        inspection_count: 0,
        op,
        test_factor,
        test_pass,
        test_fail,
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input.split("\n\n").map(parse_monkey).collect()
}

fn round_of_throws(monkeys: &mut Vec<Monkey>, dampen: bool, common_factor: u64) {
    (0..monkeys.len()).for_each(|i| {
        monkeys[i].inspection_count += monkeys[i].items.len();
        let Monkey {
            items,
            inspection_count: _,
            op,
            test_factor,
            test_pass,
            test_fail,
        } = &monkeys[i];
        let test_pass = *test_pass;
        let test_fail = *test_fail;
        let mut test_pass_items = vec![];
        let mut test_fail_items = vec![];
        for item in items {
            let mut item_concern = op(*item);
            if dampen {
                item_concern /= 3;
            } else {
                item_concern %= common_factor;
            }
            if item_concern % test_factor == 0 {
                test_pass_items.push(item_concern);
            } else {
                test_fail_items.push(item_concern);
            }
        }
        monkeys[test_pass].items.append(&mut test_pass_items);
        monkeys[test_fail].items.append(&mut test_fail_items);
        monkeys[i].items.clear();
    });
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let mut monkeys = parse_input(input);
        for _ in 0..20 {
            round_of_throws(&mut monkeys, true, 0);
        }
        monkeys
            .iter()
            .map(|m| m.inspection_count)
            .sorted_unstable_by(Ord::cmp)
            .rev()
            .take(2)
            .product::<usize>()
            .to_string()
    }
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let mut monkeys = parse_input(input);
        let common_factor: u64 = monkeys.iter().map(|m| m.test_factor).product();
        for i in 0..10000 {
            if i % 1000 == 0 || i == 1 || i == 20 {
                let counts = monkeys.iter().map(|m| m.inspection_count).collect_vec();
                println!("{:?}", &counts);
            }
            round_of_throws(&mut monkeys, false, common_factor);
        }
        let counts = monkeys
            .iter()
            .map(|m| m.inspection_count)
            .sorted_unstable_by(Ord::cmp)
            .rev()
            .collect_vec();
        println!("{:?}", &counts);
        counts.iter().take(2).product::<usize>().to_string()
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
