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

#[derive(Debug, Clone)]
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

struct PartRange<'a> {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
    destination: WorkflowDestination<'a>,
}

fn run_range_workflows(workflows: &HashMap<&str, Vec<WorkflowStep>>) -> u64 {
    let mut ranges = vec![PartRange {
        x: (1, 4001),
        m: (1, 4001),
        a: (1, 4001),
        s: (1, 4001),
        destination: WorkflowDestination::Workflow("in"),
    }];
    let mut accepted = 0;
    while let Some(PartRange {
        x,
        m,
        a,
        s,
        destination,
    }) = ranges.pop()
    {
        match destination {
            WorkflowDestination::Workflow(d) => {
                let workflow = &workflows[d];
                workflow
                    .iter()
                    .fold((x, m, a, s), |(x, m, a, s), check| match check.check {
                        WorkflowCheck::XAbove(v) => {
                            let (j, k) = x;
                            if k < v {
                                ranges.push(PartRange {
                                    x,
                                    m,
                                    a,
                                    s,
                                    destination: check.destination.clone(),
                                });
                                ((0, 0), (0, 0), (0, 0), (0, 0))
                            } else if j < v {
                                ranges.push(PartRange {
                                    x: (v, k),
                                    m,
                                    a,
                                    s,
                                    destination: check.destination.clone(),
                                });
                                ((j, v), m, a, s)
                            } else {
                                (x, m, a, s)
                            }
                        }
                        WorkflowCheck::XBelow(v) => {
                            let (j, k) = x;
                            if v < j {
                                ranges.push(PartRange {
                                    x,
                                    m,
                                    a,
                                    s,
                                    destination: check.destination.clone(),
                                });
                                ((0, 0), (0, 0), (0, 0), (0, 0))
                            } else if v < k {
                                ranges.push(PartRange {
                                    x: (j, v),
                                    m,
                                    a,
                                    s,
                                    destination: check.destination.clone(),
                                });
                                ((v, k), m, a, s)
                            } else {
                                (x, m, a, s)
                            }
                        }
                        WorkflowCheck::MAbove(v) => {
                            let (j, k) = m;
                            if k < v {
                                ranges.push(PartRange {
                                    x,
                                    m,
                                    a,
                                    s,
                                    destination: check.destination.clone(),
                                });
                                ((0, 0), (0, 0), (0, 0), (0, 0))
                            } else if j < v {
                                ranges.push(PartRange {
                                    x,
                                    m: (v, k),
                                    a,
                                    s,
                                    destination: check.destination.clone(),
                                });
                                (x, (j, v), a, s)
                            } else {
                                (x, m, a, s)
                            }
                        }
                        WorkflowCheck::MBelow(v) => {
                            let (j, k) = m;
                            if v < j {
                                ranges.push(PartRange {
                                    x,
                                    m,
                                    a,
                                    s,
                                    destination: check.destination.clone(),
                                });
                                ((0, 0), (0, 0), (0, 0), (0, 0))
                            } else if v < k {
                                ranges.push(PartRange {
                                    x,
                                    m: (j, v),
                                    a,
                                    s,
                                    destination: check.destination.clone(),
                                });
                                (x, (v, k), a, s)
                            } else {
                                (x, m, a, s)
                            }
                        }
                        WorkflowCheck::AAbove(v) => {
                            let (j, k) = a;
                            if k < v {
                                ranges.push(PartRange {
                                    x,
                                    m,
                                    a,
                                    s,
                                    destination: check.destination.clone(),
                                });
                                ((0, 0), (0, 0), (0, 0), (0, 0))
                            } else if j < v {
                                ranges.push(PartRange {
                                    x,
                                    m,
                                    a: (v, k),
                                    s,
                                    destination: check.destination.clone(),
                                });
                                (x, m, (j, v), s)
                            } else {
                                (x, m, a, s)
                            }
                        }
                        WorkflowCheck::ABelow(v) => {
                            let (j, k) = a;
                            if v < j {
                                ranges.push(PartRange {
                                    x,
                                    m,
                                    a,
                                    s,
                                    destination: check.destination.clone(),
                                });
                                ((0, 0), (0, 0), (0, 0), (0, 0))
                            } else if v < k {
                                ranges.push(PartRange {
                                    x,
                                    m,
                                    a: (j, v),
                                    s,
                                    destination: check.destination.clone(),
                                });
                                (x, m, (v, k), s)
                            } else {
                                (x, m, a, s)
                            }
                        }
                        WorkflowCheck::SAbove(v) => {
                            let (j, k) = s;
                            if k < v {
                                ranges.push(PartRange {
                                    x,
                                    m,
                                    a,
                                    s,
                                    destination: check.destination.clone(),
                                });
                                ((0, 0), (0, 0), (0, 0), (0, 0))
                            } else if j < v {
                                ranges.push(PartRange {
                                    x,
                                    m,
                                    a,
                                    s: (v, k),
                                    destination: check.destination.clone(),
                                });
                                (x, m, a, (j, v))
                            } else {
                                (x, m, a, s)
                            }
                        }
                        WorkflowCheck::SBelow(v) => {
                            let (j, k) = s;
                            if v < j {
                                ranges.push(PartRange {
                                    x,
                                    m,
                                    a,
                                    s,
                                    destination: check.destination.clone(),
                                });
                                ((0, 0), (0, 0), (0, 0), (0, 0))
                            } else if v < k {
                                ranges.push(PartRange {
                                    x,
                                    m,
                                    a,
                                    s: (j, v),
                                    destination: check.destination.clone(),
                                });
                                (x, m, a, (v, k))
                            } else {
                                (x, m, a, s)
                            }
                        }
                        WorkflowCheck::Always => {
                            ranges.push(PartRange {
                                x,
                                m,
                                a,
                                s,
                                destination: check.destination.clone(),
                            });
                            ((0, 0), (0, 0), (0, 0), (0, 0))
                        }
                    });
            }
            WorkflowDestination::Reject => {}
            WorkflowDestination::Accept => {
                accepted += (x.1 - x.0) * (m.1 - m.0) * (a.1 - a.0) * (s.1 - s.0);
            }
        }
    }
    accepted
}

impl AocSolution for Part2 {
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let (workflows, _) = parse_input(input);
        run_range_workflows(&workflows).to_string()
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
