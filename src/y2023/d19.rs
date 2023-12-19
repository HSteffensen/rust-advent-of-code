use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, newline},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, pair, preceded, separated_pair, tuple},
    IResult,
};

use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

#[derive(Debug)]
enum WorkflowCheck {
    XAbove(u64),
    XBelow(u64),
    MAbove(u64),
    MBelow(u64),
    AAbove(u64),
    ABelow(u64),
    SAbove(u64),
    SBelow(u64),
    Always,
}

#[derive(Debug)]
enum WorkflowDestination<'a> {
    Workflow(&'a str),
    Reject,
    Accept,
}

#[derive(Debug)]
struct WorkflowStep<'a> {
    check: WorkflowCheck,
    destination: WorkflowDestination<'a>,
}

fn parse_workflow_check(input: &str) -> IResult<&str, WorkflowCheck> {
    alt((
        map(preceded(tag("x<"), complete::u64), WorkflowCheck::XBelow),
        map(preceded(tag("x>"), complete::u64), WorkflowCheck::XAbove),
        map(preceded(tag("m<"), complete::u64), WorkflowCheck::MBelow),
        map(preceded(tag("m>"), complete::u64), WorkflowCheck::MAbove),
        map(preceded(tag("a<"), complete::u64), WorkflowCheck::ABelow),
        map(preceded(tag("a>"), complete::u64), WorkflowCheck::AAbove),
        map(preceded(tag("s<"), complete::u64), WorkflowCheck::SBelow),
        map(preceded(tag("s>"), complete::u64), WorkflowCheck::SAbove),
    ))(input)
}

fn parse_workflow_destination(input: &str) -> IResult<&str, WorkflowDestination> {
    alt((
        map(tag("R"), |_| WorkflowDestination::Reject),
        map(tag("A"), |_| WorkflowDestination::Accept),
        map(alpha1, WorkflowDestination::Workflow),
    ))(input)
}

fn parse_workflow_step(input: &str) -> IResult<&str, WorkflowStep> {
    alt((
        map(
            separated_pair(parse_workflow_check, tag(":"), parse_workflow_destination),
            |(check, destination)| WorkflowStep { check, destination },
        ),
        map(parse_workflow_destination, |destination| WorkflowStep {
            check: WorkflowCheck::Always,
            destination,
        }),
    ))(input)
}

fn parse_workflows(input: &str) -> IResult<&str, HashMap<&str, Vec<WorkflowStep>>> {
    map(
        separated_list1(
            newline,
            pair(
                alpha1,
                delimited(
                    tag("{"),
                    separated_list1(tag(","), parse_workflow_step),
                    tag("}"),
                ),
            ),
        ),
        HashMap::from_iter,
    )(input)
}

#[derive(Debug)]
struct MachinePart {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

fn parse_machine_part(input: &str) -> IResult<&str, MachinePart> {
    map(
        tuple((
            tag("{x="),
            complete::u64,
            tag(",m="),
            complete::u64,
            tag(",a="),
            complete::u64,
            tag(",s="),
            complete::u64,
            tag("}"),
        )),
        |(_, x, _, m, _, a, _, s, _)| MachinePart { x, m, a, s },
    )(input)
}

fn parse_machine_parts(input: &str) -> IResult<&str, Vec<MachinePart>> {
    separated_list1(newline, parse_machine_part)(input)
}

fn parse_input(input: &str) -> (HashMap<&str, Vec<WorkflowStep>>, Vec<MachinePart>) {
    separated_pair(parse_workflows, tag("\n\n"), parse_machine_parts)(input)
        .unwrap()
        .1
}

fn run_workflow<'a>(
    part: &MachinePart,
    workflow: &'a [WorkflowStep],
) -> &'a WorkflowDestination<'a> {
    &workflow
        .iter()
        .find(
            |WorkflowStep {
                 check,
                 destination: _,
             }| match check {
                WorkflowCheck::XAbove(v) => &part.x > v,
                WorkflowCheck::XBelow(v) => &part.x < v,
                WorkflowCheck::MAbove(v) => &part.m > v,
                WorkflowCheck::MBelow(v) => &part.m < v,
                WorkflowCheck::AAbove(v) => &part.a > v,
                WorkflowCheck::ABelow(v) => &part.a < v,
                WorkflowCheck::SAbove(v) => &part.s > v,
                WorkflowCheck::SBelow(v) => &part.s < v,
                WorkflowCheck::Always => true,
            },
        )
        .unwrap()
        .destination
}

fn workflows_accept(part: &MachinePart, workflows: &HashMap<&str, Vec<WorkflowStep>>) -> bool {
    let mut workflow = &workflows["in"];
    loop {
        match run_workflow(part, workflow) {
            WorkflowDestination::Workflow(w) => {
                workflow = &workflows[w];
            }
            WorkflowDestination::Reject => {
                return false;
            }
            WorkflowDestination::Accept => {
                return true;
            }
        }
    }
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let (workflows, parts) = parse_input(input);
        parts
            .into_iter()
            .filter(|part| workflows_accept(part, &workflows))
            .map(|part| part.x + part.m + part.a + part.s)
            .sum::<u64>()
            .to_string()
    }
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        todo!("{}", input)
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
