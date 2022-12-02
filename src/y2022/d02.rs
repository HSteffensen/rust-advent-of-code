use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 2;

#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Tie,
}

fn opponent_shape(abc: char) -> Shape {
    match abc {
        'A' => Shape::Rock,
        'B' => Shape::Paper,
        'C' => Shape::Scissors,
        _ => unreachable!("Unexpected opponent, must be A B or C, got: {}", abc),
    }
}

fn self_shape(xyz: char) -> Shape {
    match xyz {
        'X' => Shape::Rock,
        'Y' => Shape::Paper,
        'Z' => Shape::Scissors,
        _ => unreachable!("Unexpected self, must be X Y or Z, got: {}", xyz),
    }
}

fn do_i_win(opponent: &Shape, me: &Shape) -> Outcome {
    match opponent {
        Shape::Rock => match me {
            Shape::Rock => Outcome::Tie,
            Shape::Paper => Outcome::Win,
            Shape::Scissors => Outcome::Loss,
        },
        Shape::Paper => match me {
            Shape::Rock => Outcome::Loss,
            Shape::Paper => Outcome::Tie,
            Shape::Scissors => Outcome::Win,
        },
        Shape::Scissors => match me {
            Shape::Rock => Outcome::Win,
            Shape::Paper => Outcome::Loss,
            Shape::Scissors => Outcome::Tie,
        },
    }
}

fn score_per_shape(shape: &Shape) -> u32 {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    }
}

fn score_per_outcome(outcome: &Outcome) -> u32 {
    match outcome {
        Outcome::Win => 6,
        Outcome::Tie => 3,
        Outcome::Loss => 0,
    }
}

fn round_score(my_shape: &Shape, outcome: &Outcome) -> u32 {
    score_per_outcome(outcome) + score_per_shape(my_shape)
}

fn parse_input(input: &str) -> Vec<(Shape, Shape)> {
    let mut parsed = Vec::new();
    for line in input.lines() {
        let opponent = opponent_shape(line.chars().next().unwrap());
        let me = self_shape(line.chars().nth(2).unwrap());
        parsed.push((opponent, me));
    }
    parsed
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let rounds = parse_input(input);
        let total_score: u32 = rounds
            .iter()
            .map(|(opponent, me)| {
                let outcome = do_i_win(opponent, me);
                round_score(me, &outcome)
            })
            .sum();
        total_score.to_string()
    }
}

fn desired_outcome(xyz: char) -> Outcome {
    match xyz {
        'X' => Outcome::Loss,
        'Y' => Outcome::Tie,
        'Z' => Outcome::Win,
        _ => unreachable!("Unexpected self, must be X Y or Z, got: {}", xyz),
    }
}

fn play_for_desired_outcome(opponent: &Shape, outcome: &Outcome) -> Shape {
    match opponent {
        Shape::Rock => match outcome {
            Outcome::Win => Shape::Paper,
            Outcome::Loss => Shape::Scissors,
            Outcome::Tie => Shape::Rock,
        },
        Shape::Paper => match outcome {
            Outcome::Win => Shape::Scissors,
            Outcome::Loss => Shape::Rock,
            Outcome::Tie => Shape::Paper,
        },
        Shape::Scissors => match outcome {
            Outcome::Win => Shape::Rock,
            Outcome::Loss => Shape::Paper,
            Outcome::Tie => Shape::Scissors,
        },
    }
}

fn parse_input2(input: &str) -> Vec<(Shape, Outcome)> {
    let mut parsed = Vec::new();
    for line in input.lines() {
        let opponent = opponent_shape(line.chars().next().unwrap());
        let me = desired_outcome(line.chars().nth(2).unwrap());
        parsed.push((opponent, me));
    }
    parsed
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let rounds = parse_input2(input);
        let total_score: u32 = rounds
            .iter()
            .map(|(opponent, outcome)| {
                let me = play_for_desired_outcome(opponent, outcome);
                round_score(&me, outcome)
            })
            .sum();
        total_score.to_string()
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
