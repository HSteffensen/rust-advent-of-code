use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

fn holiday_ascii_string_helper(input: &str) -> usize {
    let mut value = 0;
    for c in input.chars() {
        value += c as usize;
        value *= 17;
        value %= 256
    }
    value
}

#[derive(Debug, Clone)]
struct Lens<'a> {
    label: &'a str,
    focus: usize,
}

#[derive(Debug)]
enum Instruction<'a> {
    Add { lens: Lens<'a>, box_index: usize },
    Remove { label: &'a str, box_index: usize },
}

fn parse_instruction(text: &str) -> Instruction {
    if text.ends_with('-') {
        let label = &text[0..text.len() - 1];
        Instruction::Remove {
            label,
            box_index: holiday_ascii_string_helper(label),
        }
    } else {
        let (label, focus) = text.split_once('=').unwrap();
        Instruction::Add {
            lens: Lens {
                label,
                focus: focus.parse().unwrap(),
            },
            box_index: holiday_ascii_string_helper(label),
        }
    }
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        input
            .trim()
            .split(',')
            .map(holiday_ascii_string_helper)
            .sum::<usize>()
            .to_string()
    }
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
        for instruction in input.trim().split(',').map(parse_instruction) {
            match instruction {
                Instruction::Add { lens, box_index } => {
                    let current_box = &mut boxes[box_index];
                    if let Some(b) = current_box.iter_mut().find(|b| b.label == lens.label) {
                        b.focus = lens.focus;
                    } else {
                        current_box.push(lens);
                    }
                }
                Instruction::Remove { label, box_index } => {
                    let current_box = &mut boxes[box_index];
                    if let Some((i, _)) = current_box
                        .iter_mut()
                        .enumerate()
                        .find(|(_, b)| b.label == label)
                    {
                        current_box.remove(i);
                    }
                }
            }
        }
        boxes
            .into_iter()
            .enumerate()
            .map(|(i, b)| {
                b.iter()
                    .enumerate()
                    .map(|(j, l)| (i + 1) * (j + 1) * l.focus)
                    .sum::<usize>()
            })
            .sum::<usize>()
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
