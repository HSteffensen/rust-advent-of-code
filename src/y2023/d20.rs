use std::collections::{HashMap, VecDeque};

use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alpha1, newline},
    combinator::{map, value},
    error::Error,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
};

use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

#[derive(Debug, Clone, Copy)]
enum PulsePitch {
    Low,
    High,
}

#[derive(Debug, Clone)]
enum Module<'input> {
    FlipFlop {
        name: &'input str,
        is_on: bool,
    },
    Conjunction {
        name: &'input str,
        memory: HashMap<&'input str, PulsePitch>,
    },
    Broadcaster,
}

struct Pulse<'input> {
    source_name: &'input str,
    destination_name: &'input str,
    pitch: PulsePitch,
}

struct ModuleArray<'input> {
    modules: HashMap<&'input str, Module<'input>>,
    connections: HashMap<&'input str, Vec<&'input str>>,
}

impl<'input> ModuleArray<'input> {
    const BROADCASTER: &'static str = "broadcaster";
    const BUTTON: &'static str = "button";
    fn send_pulse(&mut self) -> (u64, u64) {
        let mut low_count = 0;
        let mut high_count = 0;
        let mut pulses = VecDeque::from([Pulse {
            source_name: Self::BUTTON,
            destination_name: Self::BROADCASTER,
            pitch: PulsePitch::Low,
        }]);
        while let Some(pulse) = pulses.pop_back() {
            match pulse.pitch {
                PulsePitch::Low => low_count += 1,
                PulsePitch::High => high_count += 1,
            }
            self.modules
                .entry(pulse.destination_name)
                .and_modify(|module| match module {
                    Module::FlipFlop { name, is_on } => {
                        if matches!(pulse.pitch, PulsePitch::Low) {
                            let new_pitch = if *is_on {
                                *is_on = false;
                                PulsePitch::Low
                            } else {
                                *is_on = true;
                                PulsePitch::High
                            };
                            self.connections[name].iter().for_each(|m| {
                                pulses.push_front(Pulse {
                                    source_name: name,
                                    destination_name: m,
                                    pitch: new_pitch,
                                })
                            })
                        }
                    }
                    Module::Conjunction { name, memory } => {
                        memory.insert(pulse.source_name, pulse.pitch);
                        let new_pitch = if memory.values().all(|v| matches!(v, PulsePitch::High)) {
                            PulsePitch::Low
                        } else {
                            PulsePitch::High
                        };
                        self.connections[name].iter().for_each(|m| {
                            pulses.push_front(Pulse {
                                source_name: name,
                                destination_name: m,
                                pitch: new_pitch,
                            })
                        })
                    }
                    Module::Broadcaster => {
                        self.connections[ModuleArray::BROADCASTER]
                            .iter()
                            .for_each(|m| {
                                pulses.push_front(Pulse {
                                    source_name: ModuleArray::BROADCASTER,
                                    destination_name: m,
                                    pitch: pulse.pitch,
                                })
                            });
                    }
                });
        }

        (low_count, high_count)
    }
}

fn parse_input(input: &str) -> ModuleArray {
    let modules = separated_list1(
        newline::<_, Error<_>>,
        separated_pair(
            alt((
                map(preceded(tag("%"), alpha1), |name| Module::FlipFlop {
                    name,
                    is_on: false,
                }),
                map(preceded(tag("&"), alpha1), |name| Module::Conjunction {
                    name,
                    memory: HashMap::new(),
                }),
                value(Module::Broadcaster, tag("broadcaster")),
            )),
            tag(" -> "),
            separated_list1(tag(", "), alpha1),
        ),
    )(input)
    .unwrap()
    .1;
    let mut module_array = ModuleArray {
        modules: HashMap::from_iter(modules.iter().map(|(m, _)| {
            (
                match m {
                    Module::FlipFlop { name, is_on: _ } => name,
                    Module::Conjunction { name, memory: _ } => name,
                    Module::Broadcaster => ModuleArray::BROADCASTER,
                },
                m.clone(),
            )
        })),
        connections: HashMap::from_iter(modules.iter().map(|(m, t)| {
            (
                match m {
                    Module::FlipFlop { name, is_on: _ } => name,
                    Module::Conjunction { name, memory: _ } => name,
                    Module::Broadcaster => ModuleArray::BROADCASTER,
                },
                t.iter().cloned().collect_vec(),
            )
        })),
    };
    module_array.modules.iter_mut().for_each(|(name, m)| {
        if let Module::Conjunction { name: _, memory } = m {
            module_array
                .connections
                .iter()
                .filter(|(_, t)| t.contains(name))
                .for_each(|(n, _)| {
                    memory.insert(n, PulsePitch::Low);
                });
        }
    });
    module_array
}

impl AocSolution for Part1 {
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let mut module_array = parse_input(input);
        let (low, high) = (0..1000)
            .map(|_| module_array.send_pulse())
            .reduce(|(l1, h1), (l2, h2)| (l1 + l2, h1 + h2))
            .unwrap();
        (low * high).to_string()
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
