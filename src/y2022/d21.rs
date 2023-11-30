use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::{
        self,
        complete::{newline, one_of},
    },
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 21;

#[derive(Debug, Clone)]
enum MonkeyMathFunction {
    Plus,
    Minus,
    Times,
    DividedBy,
}

#[derive(Debug, Clone)]
enum MonkeyYell<'a> {
    Function(MonkeyMathFunction, &'a str, &'a str),
    Value(i64),
}

fn parse_input(input: &str) -> HashMap<&str, MonkeyYell> {
    let (_, captured) = separated_list0(newline, parse_line)(input).unwrap();
    HashMap::from_iter(captured)
}

fn parse_line(line: &str) -> IResult<&str, (&str, MonkeyYell)> {
    map(
        tuple((
            take(4usize),
            tag(": "),
            alt((
                map(character::complete::i64, MonkeyYell::Value),
                map(
                    tuple((
                        take(4usize),
                        tag(" "),
                        one_of("+-*/"),
                        tag(" "),
                        take(4usize),
                    )),
                    |(a, _, b, _, c)| {
                        MonkeyYell::Function(
                            match b {
                                '+' => MonkeyMathFunction::Plus,
                                '-' => MonkeyMathFunction::Minus,
                                '*' => MonkeyMathFunction::Times,
                                '/' => MonkeyMathFunction::DividedBy,
                                _ => unreachable!(),
                            },
                            a,
                            c,
                        )
                    },
                ),
            )),
        )),
        |(monkey, _, yell)| (monkey, yell),
    )(line)
}

fn evaluate_monkey<'a, 'b>(
    monkey_name: &'a str,
    all_monkeys: &'b HashMap<&'a str, MonkeyYell<'a>>,
) -> i64 {
    let monkey_yell = all_monkeys.get(monkey_name).unwrap().clone();

    match monkey_yell {
        MonkeyYell::Function(f, a, b) => {
            let a_value = evaluate_monkey(a, all_monkeys);
            let b_value = evaluate_monkey(b, all_monkeys);
            let v = match f {
                MonkeyMathFunction::Plus => a_value + b_value,
                MonkeyMathFunction::Minus => a_value - b_value,
                MonkeyMathFunction::Times => a_value * b_value,
                MonkeyMathFunction::DividedBy => a_value / b_value,
            };
            v
        }
        MonkeyYell::Value(v) => v,
    }
}

#[test]
fn test_parse() {
    let input = &Part1::get_examples()[0].0;
    println!("{:?}", parse_input(input));
}

#[test]
fn test_evaluate() {
    let input = &Part1::get_examples()[0].0;
    let mut monkeys = parse_input(input);
    assert_eq!(evaluate_monkey("zczc", &mut monkeys), 2);
    assert_eq!(evaluate_monkey("drzm", &mut monkeys), 30);
    match monkeys["drzm"] {
        MonkeyYell::Function(_, _, _) => panic!("unexpected match arm in test"),
        MonkeyYell::Value(v) => assert_eq!(v, 30),
    };
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let yells = parse_input(input);
        evaluate_monkey("root", &yells).to_string()
    }
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let mut yells = parse_input(input);
        yells.insert("humn", MonkeyYell::Value(1));
        if let MonkeyYell::Function(_, left, right) = yells["root"] {
            let lv = evaluate_monkey(left, &yells);
            let rv = evaluate_monkey(right, &yells);

            yells.insert("humn", MonkeyYell::Value(1000));
            let lv2 = evaluate_monkey(left, &yells);

            let (constant, monkey) = if lv == lv2 { (lv, right) } else { (rv, left) };
            let (mut lower, mut upper) = (1, 2 << 20);
            yells.insert("humn", MonkeyYell::Value(lower));
            let lower_result = evaluate_monkey(monkey, &yells);
            yells.insert("humn", MonkeyYell::Value(upper));
            let upper_result = evaluate_monkey(monkey, &yells);
            let sign = (upper_result - lower_result).signum();

            println!(
                "target = {}\nstart = {}:{}, {}:{}",
                constant, lower, lower_result, upper, upper_result
            );

            let mut i = 0;
            while lower < upper {
                if i >= 10 {
                    todo!()
                }
                i += 1;
                let middle = upper + lower / 2;
                yells.insert("humn", MonkeyYell::Value(middle));
                let middle_result = evaluate_monkey(monkey, &yells);
                println!("{}, {}:{}, {}", lower, middle, middle_result, upper);
                println!(
                    "{}-{} * {} = {}",
                    middle_result,
                    constant,
                    sign,
                    (middle_result - constant) * sign
                );
                if middle_result == constant {
                    return middle.to_string();
                } else if (middle_result - constant) * sign > 0 {
                    upper = middle;
                } else {
                    lower = middle;
                }
            }
            lower.to_string()
        } else {
            panic!()
        }
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
