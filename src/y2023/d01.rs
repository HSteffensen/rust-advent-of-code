use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::{self, anychar},
    combinator::{all_consuming, map, map_parser, peek, value},
    error::Error,
    multi::many0,
    sequence::tuple,
    IResult,
};

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 1;

fn first_digit(input: &str) -> char {
    let digit = input.chars().find_or_first(|c| c.is_numeric());
    match digit {
        Some(c) => c,
        None => unreachable!(),
    }
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        input
            .lines()
            .map(|line| {
                let first = first_digit(line);
                let last = first_digit(line.chars().rev().collect::<String>().as_str());
                vec![first, last]
                    .iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap()
            })
            .sum::<i32>()
            .to_string()
    }
}

fn peek_digit_or_word_number(input: &str) -> IResult<&str, Option<i32>> {
    peek(alt((
        value(Some(0), tag::<_, _, Error<_>>("zero")),
        value(Some(1), tag("one")),
        value(Some(2), tag("two")),
        value(Some(3), tag("three")),
        value(Some(4), tag("four")),
        value(Some(5), tag("five")),
        value(Some(6), tag("six")),
        value(Some(7), tag("seven")),
        value(Some(8), tag("eight")),
        value(Some(9), tag("nine")),
        map_parser(take(1u32), map(complete::i32, |v| Some(v))),
        value(None, anychar),
    )))(input)
}

#[test]
fn test_peek_digit_or_word_number() {
    assert_eq!(peek_digit_or_word_number("3").unwrap().1, Some(3));
    assert_eq!(peek_digit_or_word_number("3wow").unwrap().1, Some(3));
    assert_eq!(peek_digit_or_word_number("three").unwrap().1, Some(3));
    assert_eq!(peek_digit_or_word_number("threewow").unwrap().1, Some(3));
    assert_eq!(peek_digit_or_word_number("w").unwrap().1, None);
    assert_eq!(peek_digit_or_word_number("w3").unwrap().1, None);
    assert_eq!(peek_digit_or_word_number("wthree").unwrap().1, None);
    assert_eq!(peek_digit_or_word_number("3four").unwrap().1, Some(3));
    assert_eq!(peek_digit_or_word_number("three4").unwrap().1, Some(3));
    assert_eq!(peek_digit_or_word_number("33").unwrap().1, Some(3));
}

fn parse_digits_or_word_numbers(input: &str) -> i32 {
    let parse_result = all_consuming(many0(map(
        tuple((peek_digit_or_word_number, anychar)),
        |(v, _)| v,
    )))(input);
    let (_, parsed) = parse_result.unwrap();
    let parsed: Vec<i32> = parsed.into_iter().flatten().collect();
    let first = parsed.first().unwrap();
    let last = parsed.last().unwrap();
    (first * 10) + last
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        input
            .lines()
            .map(|line| parse_digits_or_word_numbers(line))
            .sum::<i32>()
            .to_string()
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

struct Part1Simpler {}
struct Part2Simpler {}

fn find_digits(input: &str) -> u32 {
    let first = input
        .chars()
        .find(|c| c.is_numeric())
        .unwrap()
        .to_digit(10)
        .unwrap();
    let last = input
        .chars()
        .rfind(|c| c.is_numeric())
        .unwrap()
        .to_digit(10)
        .unwrap();
    (first * 10) + last
}

impl AocSolution for Part1Simpler {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        input.lines().map(find_digits).sum::<u32>().to_string()
    }
}

fn replace_number_words(input: &str) -> String {
    input
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine")
        .replace("zero", "zero0zero")
}

impl AocSolution for Part2Simpler {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        input
            .lines()
            .map(replace_number_words)
            .map(|s| find_digits(s.as_str()))
            .sum::<u32>()
            .to_string()
    }
}

#[test]
fn p1_simpler_run() {
    Part1Simpler::solve();
}

#[test]
fn p2_simpler_run() {
    Part1Simpler::solve();
}
