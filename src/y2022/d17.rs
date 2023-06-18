use std::{fmt::Display, ops::Shr};

use itertools::Itertools;

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 17;

#[derive(PartialEq)]
enum JetDirection {
    Left,
    Right,
}

fn parse_input(input: &str) -> Vec<JetDirection> {
    input
        .trim()
        .chars()
        .map(|c| match c {
            '<' => JetDirection::Left,
            '>' => JetDirection::Right,
            _ => unreachable!(),
        })
        .collect_vec()
}

// Each rock appears so that its left edge is two units away from the left wall
// and its bottom edge is three units above the highest rock in the room
// (or the floor, if there isn't one).
fn blocks() -> Vec<Vec<u16>> {
    vec![
        vec![0b111100000],
        vec![0b010000000, 0b111000000, 0b010000000],
        vec![0b111000000, 0b001000000, 0b001000000],
        vec![0b100000000, 0b100000000, 0b100000000, 0b100000000],
        vec![0b110000000, 0b110000000],
    ]
}

struct TetrisGrid {
    grid: Vec<u16>,
}

impl Default for TetrisGrid {
    fn default() -> Self {
        Self {
            grid: vec![0b111111111],
        }
    }
}

impl Display for TetrisGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut rows = self.grid.iter().rev().map(|r| format!("{r:09b}")).map(|r| {
            r.chars()
                .map(|c| match c {
                    '0' => '.',
                    '1' => '#',
                    _ => unreachable!(),
                })
                .join("")
        });
        f.write_str(&rows.join("\n"))
    }
}

impl TetrisGrid {
    fn height(&self) -> usize {
        self.grid
            .iter()
            .enumerate()
            .rev()
            .find(|&(_, row)| row & 0b011111110 != 0)
            .map(|(y, _)| y)
            .unwrap_or(0)
    }

    fn extend_height(&mut self, new_height: usize) {
        while self.grid.len() < new_height + 1 {
            self.grid.push(0b100000001);
        }
    }

    fn check_collision(&self, block: &[u16], x: u8, y: usize) -> bool {
        self.grid
            .iter()
            .skip(y)
            .zip(block.iter())
            .any(|(g, b)| g & (b.shr(x)) != 0)
    }

    fn set_block(&mut self, block: &Vec<u16>, x: u8, y: usize) {
        (0..block.len()).for_each(|i| {
            self.grid[y + i] |= block[i].shr(x);
        });
    }

    fn fall_new_block(
        &mut self,
        block: &Vec<u16>,
        stream: &mut dyn Iterator<Item = &JetDirection>,
    ) {
        let mut block_y = self.height() + 4;
        let mut block_x: u8 = 3;
        self.extend_height(block_y + block.len() - 1);
        for jet in stream {
            let new_x = match jet {
                JetDirection::Left => block_x - 1,
                JetDirection::Right => block_x + 1,
            };
            block_x = if (jet != &JetDirection::Left || block_x != 0)
                && !self.check_collision(block, new_x, block_y)
            {
                new_x
            } else {
                block_x
            };
            let new_y = block_y - 1;
            if self.check_collision(block, block_x, new_y) {
                break;
            } else {
                block_y = new_y;
            }
        }
        self.set_block(block, block_x, block_y);
    }
}

#[test]
fn test_tetris_block() {
    let mut grid = TetrisGrid::default();
    assert_eq!(grid.grid.len(), 1);
    assert_eq!(grid.height(), 0);
    grid.extend_height(3);
    assert_eq!(grid.grid.len(), 4);
    assert_eq!(grid.height(), 0);
    assert_eq!(
        grid.grid,
        vec![0b111111111, 0b100000001, 0b100000001, 0b100000001]
    );
    let block = vec![0b10000000, 0b10000000];
    grid.set_block(&block, 1, 1);
    assert_eq!(
        grid.grid,
        vec![0b111111111, 0b101000001, 0b101000001, 0b100000001]
    );
    assert_eq!(grid.height(), 2);
    let block = vec![0b01000000, 0b11100000, 0b01000000];
    grid.extend_height(6);
    grid.set_block(&block, 2, 3);
    assert_eq!(
        grid.grid,
        vec![
            0b111111111,
            0b101000001,
            0b101000001,
            0b100010001,
            0b100111001,
            0b100010001,
            0b100000001
        ]
    );
    assert_eq!(grid.height(), 5);
}

#[test]
fn test_collision() {
    let grid = TetrisGrid {
        grid: vec![
            0b111111111,
            0b101010101,
            0b100010001,
            0b100000001,
            0b100000001,
        ],
    };
    let block = vec![0b100000000, 0b100000000];
    (0..7).step_by(2).for_each(|x| {
        assert!(grid.check_collision(&block, x, 1));
        assert!(!grid.check_collision(&block, x + 1, 1));
        assert!(grid.check_collision(&block, x, 0));
        assert!(grid.check_collision(&block, x + 1, 0));
    });
    assert!(grid.check_collision(&block, 8, 1));
    assert!(grid.check_collision(&block, 8, 0));
    assert!(grid.check_collision(&block, 0, 3));
    assert!(grid.check_collision(&block, 8, 3));
    (1..8).for_each(|x| {
        assert!(!grid.check_collision(&block, x, 3), "{}", x);
    });
}

#[test]
fn test_falling_block() {
    let mut grid = TetrisGrid::default();
    let input = parse_input(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
    let mut stream = input.iter().cycle();
    let binding = blocks();
    let mut blocks = binding.iter().cycle();

    assert_eq!(grid.grid, vec![0b111111111]);
    grid.fall_new_block(blocks.next().unwrap(), &mut stream);
    assert_eq!(
        grid.grid,
        vec![
            0b111111111,
            0b100111101,
            0b100000001,
            0b100000001,
            0b100000001,
        ]
    );
    grid.fall_new_block(blocks.next().unwrap(), &mut stream);
    assert_eq!(
        grid.grid,
        vec![
            0b111111111,
            0b100111101,
            0b100010001,
            0b100111001,
            0b100010001,
            0b100000001,
            0b100000001,
            0b100000001,
        ]
    );
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let mut grid = TetrisGrid::default();
        let input = parse_input(input);
        let mut stream = input.iter().cycle();
        let binding = blocks();
        let blocks = binding.iter().cycle();

        for (i, block) in blocks.enumerate() {
            if i < 2022 {
                grid.fall_new_block(block, &mut stream);
            } else {
                break;
            }
        }

        grid.height().to_string()
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
