use std::{num::ParseIntError, str::FromStr};

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 13;

#[derive(Debug, PartialEq, Eq)]
enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

#[derive(Debug, PartialEq)]
enum ParsePacketError {
    ExpectedListStart(Option<char>, usize),
    ExpectedListDelimiterOrEnd(Option<char>, usize),
    ExpectedIntStart(Option<char>, usize),
    ParseIntError(ParseIntError),
}

fn packet_list_from_chars(input: &str, index: &mut usize) -> Result<Packet, ParsePacketError> {
    let mut packets = vec![];
    match input.chars().nth(*index) {
        Some('[') => {
            *index += 1;
        }
        e => return Err(ParsePacketError::ExpectedListStart(e, *index)),
    };
    if let Some(']') = input.chars().nth(*index) {
        *index += 1;
        return Ok(Packet::List(packets));
    }
    loop {
        let p = packet_from_str(input, index)?;
        packets.push(p);
        match input.chars().nth(*index) {
            Some(']') => {
                *index += 1;
                break;
            }
            Some(',') => {
                *index += 1;
            }
            e => return Err(ParsePacketError::ExpectedListDelimiterOrEnd(e, *index)),
        }
    }

    Ok(Packet::List(packets))
}

fn packet_int_from_str(input: &str, index: &mut usize) -> Result<Packet, ParsePacketError> {
    let mut current_int_str = "".to_owned();
    let mut chars = input.chars().skip(*index);
    match chars.next() {
        Some(c) => {
            current_int_str.push(c);
            *index += 1;
        }
        e => return Err(ParsePacketError::ExpectedIntStart(e, *index)),
    };
    for c in chars {
        if c.is_ascii_digit() {
            *index += 1;
            current_int_str.push(c);
        } else {
            break;
        }
    }
    match current_int_str.parse() {
        Ok(value) => Ok(Packet::Int(value)),
        Err(e) => Err(ParsePacketError::ParseIntError(e)),
    }
}

fn packet_from_str(input: &str, index: &mut usize) -> Result<Packet, ParsePacketError> {
    let first_char = input.chars().nth(*index).unwrap();
    if first_char.is_ascii_digit() {
        packet_int_from_str(input, index)
    } else if first_char == '[' {
        packet_list_from_chars(input, index)
    } else {
        unreachable!()
    }
}

#[test]
fn test_packet_list() {
    assert_eq!(
        packet_from_str("[1,2,10]", &mut 0).unwrap(),
        Packet::List(vec![Packet::Int(1), Packet::Int(2), Packet::Int(10)])
    );
    assert_eq!(
        packet_from_str("[1,2,[18,42],10]", &mut 0).unwrap(),
        Packet::List(vec![
            Packet::Int(1),
            Packet::Int(2),
            Packet::List(vec![Packet::Int(18), Packet::Int(42)]),
            Packet::Int(10)
        ])
    );
    assert_eq!(packet_from_str("[]", &mut 0).unwrap(), Packet::List(vec![]));
    assert_eq!(
        packet_from_str("[[[]]]", &mut 0).unwrap(),
        Packet::List(vec![Packet::List(vec![Packet::List(vec![]),]),])
    );
}

impl FromStr for Packet {
    type Err = ParsePacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        packet_from_str(s, &mut 0)
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self {
            Packet::Int(value) => match other {
                Packet::Int(other_value) => value.cmp(other_value),
                Packet::List(_) => Packet::List(vec![Packet::Int(*value)]).cmp(other),
            },
            Packet::List(values) => match other {
                Packet::Int(other_value) => {
                    self.cmp(&Packet::List(vec![Packet::Int(*other_value)]))
                }
                Packet::List(other_values) => values.iter().cmp(other_values.iter()),
            },
        }
    }
}

fn parse_input(input: &str) -> Vec<(Packet, Packet)> {
    let pairs = input.split("\n\n");
    let mut result = vec![];
    for (left, right) in pairs.map(|s| s.split_once('\n').unwrap()) {
        result.push((left.parse().unwrap(), right.parse().unwrap()));
    }
    result
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let packet_pairs = parse_input(input);
        packet_pairs
            .iter()
            .enumerate()
            .filter_map(|(i, (left, right))| if left <= right { Some(i + 1) } else { None })
            .sum::<usize>()
            .to_string()
    }
}

fn parse_input_p2(input: &str) -> Vec<Packet> {
    let mut result = vec![];
    for line in input.lines() {
        if !line.is_empty() {
            result.push(line.parse().unwrap());
        }
    }
    result
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let mut packets = parse_input_p2(input);
        let divider_1 = "[[2]]".parse().unwrap();
        let divider_2 = "[[6]]".parse().unwrap();
        packets.push(divider_1);
        packets.push(divider_2);
        packets.sort();
        let divider_1 = "[[2]]".parse().unwrap();
        let divider_2 = "[[6]]".parse().unwrap();
        let divider_1_index = packets
            .iter()
            .enumerate()
            .find_map(|(i, p)| if *p == divider_1 { Some(i) } else { None })
            .unwrap();
        let divider_2_index = packets
            .iter()
            .enumerate()
            .find_map(|(i, p)| if *p == divider_2 { Some(i) } else { None })
            .unwrap();
        ((divider_1_index + 1) * (divider_2_index + 1)).to_string()
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
