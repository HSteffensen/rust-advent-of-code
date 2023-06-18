use std::{collections::HashMap, fmt::Display, ops::Shr};

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
    blocks: Vec<Vec<u16>>,
    block_index: usize,
    jets: Vec<JetDirection>,
    jet_index: usize,
}

#[derive(PartialEq, Eq, Hash)]
struct GridHash {
    block_index: usize,
    jet_index: usize,
    grid_top: [u16; 13],
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
    fn new(jets: Vec<JetDirection>) -> Self {
        Self {
            grid: vec![0b111111111],
            blocks: blocks(),
            block_index: 0,
            jets,
            jet_index: 0,
        }
    }

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

    fn check_collision(&self, x: u8, y: usize) -> bool {
        self.grid
            .iter()
            .skip(y)
            .zip(self.blocks.get(self.block_index).unwrap().iter())
            .any(|(g, b)| g & (b.shr(x)) != 0)
    }

    fn set_block(&mut self, x: u8, y: usize) {
        let block = self.blocks.get(self.block_index).unwrap();
        (0..block.len()).for_each(|i| {
            self.grid[y + i] |= block[i].shr(x);
        });
    }

    fn next_block(&mut self) -> Vec<u16> {
        self.block_index += 1;
        self.block_index %= self.blocks.len();
        self.blocks.get(self.block_index).unwrap().to_owned()
    }

    fn next_jet(&mut self) -> &JetDirection {
        self.jet_index += 1;
        self.jet_index %= self.jets.len();
        self.jets.get(self.jet_index).unwrap()
    }

    fn fall_new_block(&mut self) {
        let mut block_y = self.height() + 4;
        let mut block_x: u8 = 3;
        let block = self.next_block();
        self.extend_height(block_y + block.len() - 1);
        loop {
            let jet = self.next_jet();
            let new_x = match jet {
                JetDirection::Left => block_x - 1,
                JetDirection::Right => block_x + 1,
            };
            block_x = if (jet != &JetDirection::Left || block_x != 0)
                && !self.check_collision(new_x, block_y)
            {
                new_x
            } else {
                block_x
            };
            let new_y = block_y - 1;
            if self.check_collision(block_x, new_y) {
                break;
            } else {
                block_y = new_y;
            }
        }
        self.set_block(block_x, block_y);
    }

    fn grid_hash(&self) -> GridHash {
        let mut grid = self.grid.iter().rev();
        let grid_top = [
            grid.next().unwrap_or(&0).to_owned(),
            grid.next().unwrap_or(&0).to_owned(),
            grid.next().unwrap_or(&0).to_owned(),
            grid.next().unwrap_or(&0).to_owned(),
            grid.next().unwrap_or(&0).to_owned(),
            grid.next().unwrap_or(&0).to_owned(),
            grid.next().unwrap_or(&0).to_owned(),
            grid.next().unwrap_or(&0).to_owned(),
            grid.next().unwrap_or(&0).to_owned(),
            grid.next().unwrap_or(&0).to_owned(),
            grid.next().unwrap_or(&0).to_owned(),
            grid.next().unwrap_or(&0).to_owned(),
            grid.next().unwrap_or(&0).to_owned(),
        ];
        GridHash {
            block_index: self.block_index,
            jet_index: self.jet_index,
            grid_top,
        }
    }
}

#[test]
fn test_falling_block() {
    let input = parse_input(">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>");
    let mut grid = TetrisGrid::new(input);
    grid.block_index = grid.blocks.len() - 1;
    grid.jet_index = grid.jets.len() - 1;

    assert_eq!(grid.grid, vec![0b111111111]);
    grid.fall_new_block();
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
    grid.fall_new_block();
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

fn solve(grid: &mut TetrisGrid, n: u64) -> usize {
    let mut cache: HashMap<GridHash, u64> = HashMap::new();
    let mut height_history: HashMap<u64, usize> = HashMap::new();
    for i in 0..n {
        grid.fall_new_block();
        let state = grid.grid_hash();
        let height = grid.height();
        if let Some(last_index) = cache.get(&state) {
            let last_height = height_history.get(last_index).unwrap();
            let height_delta = height - last_height;
            let drop_delta = i - last_index;
            let drops_remaining = n - i;
            let loops = drops_remaining / drop_delta;
            let drops_after_loops = drops_remaining % drop_delta;
            let height_after_loops = height + (height_delta * loops as usize);
            let height_during_loop = height_history
                .get(&(last_index + drops_after_loops - 1))
                .unwrap();
            let final_height = height_after_loops + height_during_loop - last_height;
            return final_height;
        } else {
            cache.insert(state, i);
        }
        height_history.insert(i, height);
    }
    unreachable!()
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let input = parse_input(input);
        let mut grid = TetrisGrid::new(input);
        grid.block_index = grid.blocks.len() - 1;
        grid.jet_index = grid.jets.len() - 1;

        solve(&mut grid, 2022).to_string()
    }
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let input = parse_input(input);
        let mut grid = TetrisGrid::new(input);
        grid.block_index = grid.blocks.len() - 1;
        grid.jet_index = grid.jets.len() - 1;

        solve(&mut grid, 1000000000000).to_string()
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
