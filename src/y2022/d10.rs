use itertools::Itertools;

use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

#[derive(Debug)]
enum Instruction {
    Addx(i32),
    Noop,
}

#[derive(Debug)]
struct CpuState {
    clock: u32,
    x: i32,
}

impl CpuState {
    fn execute_instruction(&mut self, instruction: &Instruction) -> i32 {
        let previous_x = self.x;
        match instruction {
            Instruction::Addx(dx) => {
                self.clock += 2;
                self.x += dx;
            }
            Instruction::Noop => self.clock += 1,
        }
        previous_x
    }
}

impl Default for CpuState {
    fn default() -> Self {
        Self { clock: 0, x: 1 }
    }
}

fn parse_line(line: &str) -> Instruction {
    match line.split_whitespace().collect_vec()[..] {
        ["addx", v] => Instruction::Addx(v.parse().unwrap()),
        ["noop"] => Instruction::Noop,
        _ => unreachable!(),
    }
}

fn parse_input(input: &str) -> Vec<Instruction> {
    input.lines().map(parse_line).collect_vec()
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let instructions = parse_input(input);
        let mut cpu = CpuState::default();
        let mut result = 0;
        let mut target_clocks = vec![20, 60, 100, 140, 180, 220];
        for instruction in instructions {
            let prev_x = cpu.execute_instruction(&instruction);
            if target_clocks.contains(&cpu.clock) {
                target_clocks.remove(0);
                result += prev_x * cpu.clock as i32;
            } else if target_clocks.contains(&(cpu.clock - 1)) {
                target_clocks.remove(0);
                result += prev_x * (cpu.clock as i32 - 1);
            }
        }
        result.to_string()
    }
}

fn pixel_is_lit(cycle: i32, cpu: i32) -> bool {
    let cycle_x = cycle % 40;
    (cpu - 1..=cpu + 1).contains(&cycle_x)
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn do_post_answer() -> bool {
        false
    }

    fn implementation(input: &str) -> String {
        let instruction_vec = parse_input(input);
        let mut instructions = instruction_vec.iter();
        let mut cpu = CpuState::default();
        let mut prev_x = cpu.x;
        let mut line = vec![];
        let mut lines = vec![];
        for cycle in 0..240 {
            if cpu.clock == cycle as u32 {
                if let Some(instruction) = instructions.next() {
                    prev_x = cpu.execute_instruction(instruction);
                } else {
                    unreachable!()
                }
            }
            line.push(if pixel_is_lit(cycle, prev_x) {
                '#'
            } else {
                '.'
            });
            if (cycle + 1) % 40 == 0 {
                let line_str = line.iter().collect::<String>();
                println!("{}", &line_str);
                lines.push(line_str);
                line.clear();
            }
        }
        lines.join("\n")
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
