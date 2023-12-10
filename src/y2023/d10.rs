use std::collections::HashMap;

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
struct PipeStep(StepDirection, (usize, usize));

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
                return PipeStep(StepDirection::Right, (start_x + 1, start_y))
            }
            _ => {}
        }
        match self.get(start_x, start_y + 1) {
            Some(PipeSection::Vertical)
            | Some(PipeSection::BendRightDown)
            | Some(PipeSection::BendDownLeft) => {
                return PipeStep(StepDirection::Down, (start_x, start_y + 1))
            }
            _ => {}
        }
        match self.get(start_x - 1, start_y) {
            Some(PipeSection::Horizontal)
            | Some(PipeSection::BendUpRight)
            | Some(PipeSection::BendRightDown) => {
                return PipeStep(StepDirection::Left, (start_x - 1, start_y))
            }
            _ => {}
        }
        match self.get(start_x, start_y - 1) {
            Some(PipeSection::Vertical)
            | Some(PipeSection::BendLeftUp)
            | Some(PipeSection::BendUpRight) => {
                return PipeStep(StepDirection::Up, (start_x, start_y - 1))
            }
            _ => {}
        }
        unreachable!("Expect some adjacent pipe to point towards the start")
    }

    fn next_step(&self, PipeStep(step, (x, y)): &PipeStep) -> PipeStep {
        let x = *x;
        let y = *y;
        let current_pipe = self.get(x, y).unwrap();
        match (step, current_pipe) {
            (StepDirection::Up, PipeSection::Vertical)
            | (StepDirection::Right, PipeSection::BendLeftUp)
            | (StepDirection::Left, PipeSection::BendUpRight) => {
                PipeStep(StepDirection::Up, (x, y - 1))
            }
            (StepDirection::Right, PipeSection::Horizontal)
            | (StepDirection::Up, PipeSection::BendRightDown)
            | (StepDirection::Down, PipeSection::BendUpRight) => {
                PipeStep(StepDirection::Right, (x + 1, y))
            }
            (StepDirection::Down, PipeSection::Vertical)
            | (StepDirection::Left, PipeSection::BendRightDown)
            | (StepDirection::Right, PipeSection::BendDownLeft) => {
                PipeStep(StepDirection::Down, (x, y + 1))
            }
            (StepDirection::Left, PipeSection::Horizontal)
            | (StepDirection::Up, PipeSection::BendDownLeft)
            | (StepDirection::Down, PipeSection::BendLeftUp) => {
                PipeStep(StepDirection::Left, (x - 1, y))
            }
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
        let (x, y) = self.current_step.1;
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
        let pipe_positions: HashMap<(usize, usize), &PipeSection> = HashMap::from_iter(
            grid.path()
                .map(|p| (p.1, grid.get(p.1 .0, p.1 .1).unwrap())),
        );
        let mut count = 0;
        for (y, line) in input.lines().enumerate() {
            let mut vert_count = 0;
            let mut last_corner = None;
            for (x, _) in line.chars().enumerate() {
                if let Some(pipe) = pipe_positions.get(&(x, y)) {
                    // bug: probably gives a wrong answer if the Start pipe is part of a vertical s-bend
                    // but it does give the correct answer if the Start pipe is part of a U-bend
                    if matches!(pipe, PipeSection::Vertical) {
                        vert_count += 1;
                    } else if matches!(pipe, PipeSection::BendUpRight | PipeSection::BendRightDown)
                    {
                        last_corner = Some(pipe);
                    } else if matches!(pipe, PipeSection::BendDownLeft) {
                        if matches!(last_corner, Some(PipeSection::BendUpRight)) {
                            vert_count += 1;
                        }
                        last_corner = None
                    } else if matches!(pipe, PipeSection::BendLeftUp) {
                        if matches!(last_corner, Some(PipeSection::BendRightDown)) {
                            vert_count += 1;
                        }
                        last_corner = None
                    }
                } else if vert_count % 2 == 1 {
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
