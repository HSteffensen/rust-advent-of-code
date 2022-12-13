use std::str::FromStr;

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 13;

enum Packet {
    Int(i32),
    List(Vec<Packet>),
}

enum ParsePacketError {
    PacketFormatError,
}

fn packet_from_chars(chars: &mut impl Iterator<Item = char>) -> Result<Packet, ParsePacketError> {
    todo!()
}

impl FromStr for Packet {
    type Err = ParsePacketError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut chars = s.chars();
        assert_eq!(chars.next(), Some('['));
        let packet = packet_from_chars(&mut chars);
        assert_eq!(chars.next(), Some(']'));
        assert_eq!(chars.next(), None);
        packet
    }
}

fn parse_input(input: &str) -> Vec<(Packet, Packet)> {
    let pairs = input.split("\n\n");
    let mut result = vec![];
    for (left, right) in pairs.map(|s| s.split_once('\n').unwrap()) {}
    result
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
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
