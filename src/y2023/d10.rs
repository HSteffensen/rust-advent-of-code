use std::collections::{HashMap, HashSet};

use itertools::Itertools;

use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

#[derive(Debug, PartialEq, Eq)]
enum PipeSection {
    Start,
    Ground,
    Vertical,
    Horizontal,
    BendUpRight,
    BendLeftUp,
    BendDownLeft,
    BendRightDown,
}

fn char_to_pipe(c: char) -> PipeSection {
    match c {
        '|' => PipeSection::Vertical,
        '-' => PipeSection::Horizontal,
        'L' => PipeSection::BendUpRight,
        'J' => PipeSection::BendLeftUp,
        '7' => PipeSection::BendDownLeft,
        'F' => PipeSection::BendRightDown,
        '.' => PipeSection::Ground,
        'S' => PipeSection::Start,
        _ => unreachable!("Unexpected pipe char: {}", c),
    }
}

#[derive(Debug, Clone)]
enum StepDirection {
    Up,
    Right,
    Down,
    Left,
}

struct PipeGrid {
    width: usize,
    height: usize,
    grid: Vec<PipeSection>,
}

#[derive(Debug, Clone)]
struct PipeStep {
    x: usize,
    y: usize,
    direction: StepDirection,
    winding: i32,
}

impl PipeGrid {
    fn get_pos(&self, index: usize) -> (usize, usize) {
        (index % self.width, index / self.width)
    }

    fn get(&self, x: usize, y: usize) -> Option<&PipeSection> {
        if (0..self.width).contains(&x) && (0..self.height).contains(&y) {
            self.grid.get(y * self.width + x)
        } else {
            None
        }
    }

    fn start_pos(&self) -> (usize, usize) {
        let start_index = self
            .grid
            .iter()
            .enumerate()
            .find(|(_, p)| **p == PipeSection::Start)
            .map(|(i, _)| i)
            .unwrap();
        self.get_pos(start_index)
    }

    fn first_step(&self) -> PipeStep {
        let (start_x, start_y) = self.start_pos();
        match self.get(start_x + 1, start_y) {
            Some(PipeSection::Horizontal)
            | Some(PipeSection::BendDownLeft)
            | Some(PipeSection::BendLeftUp) => {
                return PipeStep {
                    x: start_x + 1,
                    y: start_y,
                    direction: StepDirection::Right,
                    winding: 0,
                }
            }
            _ => {}
        }
        match self.get(start_x, start_y + 1) {
            Some(PipeSection::Vertical)
            | Some(PipeSection::BendRightDown)
            | Some(PipeSection::BendDownLeft) => {
                return PipeStep {
                    x: start_x,
                    y: start_y + 1,
                    direction: StepDirection::Down,
                    winding: 0,
                }
            }
            _ => {}
        }
        match self.get(start_x - 1, start_y) {
            Some(PipeSection::Horizontal)
            | Some(PipeSection::BendUpRight)
            | Some(PipeSection::BendRightDown) => {
                return PipeStep {
                    x: start_x - 1,
                    y: start_y,
                    direction: StepDirection::Left,
                    winding: 0,
                }
            }
            _ => {}
        }
        match self.get(start_x, start_y - 1) {
            Some(PipeSection::Vertical)
            | Some(PipeSection::BendLeftUp)
            | Some(PipeSection::BendUpRight) => {
                return PipeStep {
                    x: start_x,
                    y: start_y - 1,
                    direction: StepDirection::Up,
                    winding: 0,
                }
            }
            _ => {}
        }
        unreachable!("Expect some adjacent pipe to point towards the start")
    }

    fn next_step(&self, prev_step: &PipeStep) -> PipeStep {
        let x = prev_step.x;
        let y = prev_step.y;
        let winding = prev_step.winding;
        let current_pipe = self.get(x, y).unwrap();
        match (&prev_step.direction, current_pipe) {
            (StepDirection::Up, PipeSection::Vertical) => PipeStep {
                x,
                y: y - 1,
                direction: StepDirection::Up,
                winding,
            },
            (StepDirection::Right, PipeSection::BendLeftUp) => PipeStep {
                x,
                y: y - 1,
                direction: StepDirection::Up,
                winding: winding - 1,
            },
            (StepDirection::Left, PipeSection::BendUpRight) => PipeStep {
                x,
                y: y - 1,
                direction: StepDirection::Up,
                winding: winding + 1,
            },
            (StepDirection::Right, PipeSection::Horizontal) => PipeStep {
                x: x + 1,
                y,
                direction: StepDirection::Right,
                winding,
            },
            (StepDirection::Down, PipeSection::BendUpRight) => PipeStep {
                x: x + 1,
                y,
                direction: StepDirection::Right,
                winding: winding - 1,
            },
            (StepDirection::Up, PipeSection::BendRightDown) => PipeStep {
                x: x + 1,
                y,
                direction: StepDirection::Right,
                winding: winding + 1,
            },
            (StepDirection::Down, PipeSection::Vertical) => PipeStep {
                x,
                y: y + 1,
                direction: StepDirection::Down,
                winding,
            },
            (StepDirection::Left, PipeSection::BendRightDown) => PipeStep {
                x,
                y: y + 1,
                direction: StepDirection::Down,
                winding: winding - 1,
            },
            (StepDirection::Right, PipeSection::BendDownLeft) => PipeStep {
                x,
                y: y + 1,
                direction: StepDirection::Down,
                winding: winding + 1,
            },
            (StepDirection::Left, PipeSection::Horizontal) => PipeStep {
                x: x - 1,
                y,
                direction: StepDirection::Left,
                winding,
            },
            (StepDirection::Up, PipeSection::BendDownLeft) => PipeStep {
                x: x - 1,
                y,
                direction: StepDirection::Left,
                winding: winding - 1,
            },
            (StepDirection::Down, PipeSection::BendLeftUp) => PipeStep {
                x: x - 1,
                y,
                direction: StepDirection::Left,
                winding: winding + 1,
            },
            _ => unreachable!("Unexpected step"),
        }
    }

    fn path(&self) -> impl Iterator<Item = PipeStep> + '_ {
        PipePath {
            current_step: self.first_step(),
            grid: self,
            finished: false,
        }
    }
}

struct PipePath<'a> {
    current_step: PipeStep,
    grid: &'a PipeGrid,
    finished: bool,
}

impl<'a> Iterator for PipePath<'a> {
    type Item = PipeStep;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        let x = self.current_step.x;
        let y = self.current_step.y;
        let current_pipe = self.grid.get(x, y);
        match current_pipe {
            Some(PipeSection::Start) => {
                self.finished = true;
                Some(self.current_step.clone())
            }
            None => unreachable!("Should always have a valid position"),
            _ => {
                let step = self.current_step.clone();
                self.current_step = self.grid.next_step(&self.current_step);
                Some(step)
            }
        }
    }
}

fn parse_input(input: &str) -> PipeGrid {
    PipeGrid {
        width: input.chars().take_while(|&c| c != '\n').count(),
        height: input.lines().count(),
        grid: input
            .lines()
            .flat_map(|line| line.chars().map(char_to_pipe))
            .collect_vec(),
    }
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        (parse_input(input).path().count() / 2).to_string()
    }
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let grid = parse_input(input);

        let mut count = 0;
        for (y, line) in input.lines().enumerate() {
            let mut line_pipes = vec![];
            for (x, _) in line.chars().enumerate() {
                if let Some(pipe) = grid.get(x, y) {
                    match pipe {
                        PipeSection::Vertical
                        | PipeSection::BendRightDown
                        | PipeSection::BendUpRight => line_pipes.push(pipe),

                        PipeSection::BendLeftUp => {
                            todo!()
                        }
                        PipeSection::BendDownLeft => todo!(),

                        _ => {}
                    }
                } else if line_pipes.len() % 2 == 1 {
                    println!("{},{} is in", x, y);
                    count += 1;
                }
            }
        }
        count.to_string()
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
// (1, 4): Vertical, (9, 6): Vertical, (5, 1): Horizontal, (2, 5): BendUpRight, (8, 2): BendDownLeft, (1, 3): Vertical, (8, 1): Horizontal, (4, 6): Vertical, (7, 1): Horizontal, (1, 1): Start, (4, 1): Horizontal, (9, 7): BendLeftUp, (9, 2): Vertical, (6, 2): Horizontal, (6, 5): BendRightDown, (3, 2): Horizontal, (7, 5): Horizontal, (7, 2): Horizontal, (2, 3): Vertical, (4, 7): BendLeftUp, (1, 6): Vertical, (4, 2): Horizontal, (9, 3): Vertical, (9, 4): Vertical, (9, 1): BendDownLeft, (6, 6): Vertical, (1, 5): Vertical, (1, 2): Vertical, (8, 4): Vertical, (2, 1): Horizontal, (2, 2): BendRightDown, (3, 5): Horizontal, (4, 5): BendDownLeft, (7, 7): Horizontal, (3, 7): Horizontal, (8, 3): Vertical, (8, 5): BendLeftUp, (9, 5): Vertical, (8, 7): Horizontal, (6, 1): Horizontal, (2, 7): Horizontal, (6, 7): BendUpRight, (3, 1): Horizontal, (1, 7): BendUpRight, (2, 4): Vertical, (5, 2): Horizontal
