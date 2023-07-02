use std::{
    collections::HashSet,
    fmt::{self, Display},
};

use crate::common::solution::AocSolution;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 19;

#[derive(Debug, PartialEq, Eq, Hash)]
struct BotCosts {
    id: i32,
    ore_bot_ore: i32,
    clay_bot_ore: i32,
    obsidian_bot_ore: i32,
    obsidian_bot_clay: i32,
    geode_bot_ore: i32,
    geode_bot_obsidian: i32,
}

impl Display for BotCosts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            format!(
                "costs{{id={}, ore={}, clay={}, obs=({},{}), geo=({},{})}}",
                self.id,
                self.ore_bot_ore,
                self.clay_bot_ore,
                self.obsidian_bot_ore,
                self.obsidian_bot_clay,
                self.geode_bot_ore,
                self.geode_bot_obsidian
            )
            .as_str(),
        )
    }
}

fn parse_line(line: &str) -> IResult<&str, BotCosts> {
    map(
        tuple((
            tag("Blueprint "),
            complete::i32,
            tag(": Each ore robot costs "),
            complete::i32,
            tag(" ore. Each clay robot costs "),
            complete::i32,
            tag(" ore. Each obsidian robot costs "),
            complete::i32,
            tag(" ore and "),
            complete::i32,
            tag(" clay. Each geode robot costs "),
            complete::i32,
            tag(" ore and "),
            complete::i32,
            tag(" obsidian."),
        )),
        |(_, a, _, b, _, c, _, d, _, e, _, f, _, g, _)| BotCosts {
            id: a,
            ore_bot_ore: b,
            clay_bot_ore: c,
            obsidian_bot_ore: d,
            obsidian_bot_clay: e,
            geode_bot_ore: f,
            geode_bot_obsidian: g,
        },
    )(line)
}

fn parse_input(input: &str) -> IResult<&str, Vec<BotCosts>> {
    separated_list0(newline, parse_line)(input)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct BotBuildState<'a> {
    costs: &'a BotCosts,
    time_step: i32,
    ores: i32,
    ore_bots: i32,
    clays: i32,
    clay_bots: i32,
    obsidians: i32,
    obsidian_bots: i32,
    geodes: i32,
    geode_bots: i32,
}

impl<'a> Display for BotBuildState<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(
            format!(
                "state{{t={}, ore=({},{}), clay=({},{}), obs=({},{}), geo=({},{}), {}}}",
                self.time_step,
                self.ores,
                self.ore_bots,
                self.clays,
                self.clay_bots,
                self.obsidians,
                self.obsidian_bots,
                self.geodes,
                self.geode_bots,
                self.costs,
            )
            .as_str(),
        )
    }
}

impl<'a> BotBuildState<'a> {
    fn max_possible_geodes_in(&self, minutes: i32) -> i32 {
        self.max_possible_geodes_helper(minutes, 0)
    }

    fn max_possible_geodes_helper(&self, minutes: i32, mut max_so_far: i32) -> i32 {
        let time_left = minutes - self.time_step;
        let theoretical_geode_production = (time_left * time_left - 1) / 2;
        let current_geode_production = self.geodes + self.geode_bots * (minutes - self.time_step);
        let theoretical_max = current_geode_production + theoretical_geode_production;
        if self.time_step == minutes - 1 {
            self.geodes + self.geode_bots
        } else if theoretical_max <= max_so_far {
            // if we managed to build a geode bot every minute until we finish,
            // we still wouldn't beat max_so_far, so give up
            theoretical_max
        } else {
            let next_steps = self.possible_next_steps(minutes);
            if next_steps.is_empty() {
                return current_geode_production;
            }
            for step in next_steps {
                max_so_far = step
                    .max_possible_geodes_helper(minutes, max_so_far)
                    .max(max_so_far);
            }
            max_so_far
        }
    }

    fn possible_next_steps(&self, max_minutes: i32) -> Vec<BotBuildState> {
        let mut result = vec![];

        // build geode bot
        if self.obsidian_bots > 0 {
            let time_to_build = ((self.costs.geode_bot_ore - self.ores) as f64
                / self.ore_bots as f64)
                .max(
                    (self.costs.geode_bot_obsidian - self.obsidians) as f64
                        / self.obsidian_bots as f64,
                )
                .max(0.0)
                .ceil() as i32
                + 1;
            // no benefit in building geode on time_step=24, so we only care until 23
            if self.time_step + time_to_build <= max_minutes - 1 {
                result.push(BotBuildState {
                    costs: self.costs,
                    time_step: self.time_step + time_to_build,
                    ores: self.ores + self.ore_bots * time_to_build - self.costs.geode_bot_ore,
                    ore_bots: self.ore_bots,
                    clays: self.clays + self.clay_bots * time_to_build,
                    clay_bots: self.clay_bots,
                    obsidians: self.obsidians + self.obsidian_bots * time_to_build
                        - self.costs.geode_bot_obsidian,
                    obsidian_bots: self.obsidian_bots,
                    geodes: self.geodes + self.geode_bots * time_to_build,
                    geode_bots: self.geode_bots + 1,
                })
            }
        }

        // build obsidian bot
        if self.obsidian_bots < self.costs.geode_bot_obsidian && self.clay_bots > 0 {
            let time_to_build = ((self.costs.obsidian_bot_ore - self.ores) as f64
                / self.ore_bots as f64)
                .max((self.costs.obsidian_bot_clay - self.clays) as f64 / self.clay_bots as f64)
                .max(0.0)
                .ceil() as i32
                + 1;
            // no benefit in building obs on time_step=23, so we only care until 22
            if self.time_step + time_to_build <= max_minutes - 2 {
                result.push(BotBuildState {
                    costs: self.costs,
                    time_step: self.time_step + time_to_build,
                    ores: self.ores + self.ore_bots * time_to_build - self.costs.obsidian_bot_ore,
                    ore_bots: self.ore_bots,
                    clays: self.clays + self.clay_bots * time_to_build
                        - self.costs.obsidian_bot_clay,
                    clay_bots: self.clay_bots,
                    obsidians: self.obsidians + self.obsidian_bots * time_to_build,
                    obsidian_bots: self.obsidian_bots + 1,
                    geodes: self.geodes + self.geode_bots * time_to_build,
                    geode_bots: self.geode_bots,
                })
            }
        }

        // build clay bot
        if self.clay_bots < self.costs.obsidian_bot_clay {
            let time_to_build = ((self.costs.clay_bot_ore - self.ores) as f64
                / self.ore_bots as f64)
                .max(0.0)
                .ceil() as i32
                + 1;
            // no benefit in building clay on time_step=22, so we only care until 21
            if self.time_step + time_to_build <= max_minutes - 3 {
                result.push(BotBuildState {
                    costs: self.costs,
                    time_step: self.time_step + time_to_build,
                    ores: self.ores + self.ore_bots * time_to_build - self.costs.clay_bot_ore,
                    ore_bots: self.ore_bots,
                    clays: self.clays + self.clay_bots * time_to_build,
                    clay_bots: self.clay_bots + 1,
                    obsidians: self.obsidians + self.obsidian_bots * time_to_build,
                    obsidian_bots: self.obsidian_bots,
                    geodes: self.geodes + self.geode_bots * time_to_build,
                    geode_bots: self.geode_bots,
                })
            }
        }

        // build ore bot
        if self.ore_bots // only build if any recipe could go faster
            < self
                .costs
                .clay_bot_ore
                .max(self.costs.obsidian_bot_ore)
                .max(self.costs.geode_bot_ore)
        {
            let time_to_build = ((self.costs.ore_bot_ore - self.ores) as f64 / self.ore_bots as f64)
                .max(0.0)
                .ceil() as i32
                + 1;
            // no benefit in building ore on time_step=21, so we only care until 20
            if self.time_step + time_to_build <= max_minutes - 4 {
                result.push(BotBuildState {
                    costs: self.costs,
                    time_step: self.time_step + time_to_build,
                    ores: self.ores + self.ore_bots * time_to_build - self.costs.ore_bot_ore,
                    ore_bots: self.ore_bots + 1,
                    clays: self.clays + self.clay_bots * time_to_build,
                    clay_bots: self.clay_bots,
                    obsidians: self.obsidians + self.obsidian_bots * time_to_build,
                    obsidian_bots: self.obsidian_bots,
                    geodes: self.geodes + self.geode_bots * time_to_build,
                    geode_bots: self.geode_bots,
                })
            }
        }

        result
    }
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let (_, costs) = parse_input(input).unwrap();
        costs
            .iter()
            .map(|c| {
                let m = BotBuildState {
                    costs: c,
                    time_step: 0,
                    ores: 0,
                    ore_bots: 1,
                    clays: 0,
                    clay_bots: 0,
                    obsidians: 0,
                    obsidian_bots: 0,
                    geodes: 0,
                    geode_bots: 0,
                }
                .max_possible_geodes_in(24);
                println!("Blueprint {}: {} geodes, {} score", c.id, m, m * c.id);
                m * c.id
            })
            .sum::<i32>()
            .to_string()
    }
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let (_, costs) = parse_input(input).unwrap();
        costs
            .iter()
            .take(3)
            .map(|c| {
                let m = BotBuildState {
                    costs: c,
                    time_step: 0,
                    ores: 0,
                    ore_bots: 1,
                    clays: 0,
                    clay_bots: 0,
                    obsidians: 0,
                    obsidian_bots: 0,
                    geodes: 0,
                    geode_bots: 0,
                }
                .max_possible_geodes_in(32);
                println!("Blueprint {}: {} geodes", c.id, m);
                m
            })
            .product::<i32>()
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

#[test]
fn test_next_steps() {
    let costs = BotCosts {
        id: 1,
        ore_bot_ore: 4,
        clay_bot_ore: 2,
        obsidian_bot_ore: 3,
        obsidian_bot_clay: 14,
        geode_bot_ore: 2,
        geode_bot_obsidian: 7,
    };
    let bot_state = BotBuildState {
        costs: &costs,
        time_step: 1,
        ores: 1,
        ore_bots: 2,
        clays: 2,
        clay_bots: 2,
        obsidians: 3,
        obsidian_bots: 2,
        geodes: 4,
        geode_bots: 2,
    };
    let actual = bot_state.possible_next_steps(24);
    let expected = vec![
        BotBuildState {
            // built geode
            costs: &costs,
            time_step: 3,
            ores: 3,
            ore_bots: 2,
            clays: 6,
            clay_bots: 2,
            obsidians: 0,
            obsidian_bots: 2,
            geodes: 8,
            geode_bots: 3,
        },
        BotBuildState {
            // built obsidian
            costs: &costs,
            time_step: 7,
            ores: 10,
            ore_bots: 2,
            clays: 0,
            clay_bots: 2,
            obsidians: 15,
            obsidian_bots: 3,
            geodes: 16,
            geode_bots: 2,
        },
        BotBuildState {
            // built clay
            costs: &costs,
            time_step: 2,
            ores: 1,
            ore_bots: 2,
            clays: 4,
            clay_bots: 3,
            obsidians: 5,
            obsidian_bots: 2,
            geodes: 6,
            geode_bots: 2,
        },
        BotBuildState {
            // built ore
            costs: &costs,
            time_step: 3,
            ores: 1,
            ore_bots: 3,
            clays: 6,
            clay_bots: 2,
            obsidians: 7,
            obsidian_bots: 2,
            geodes: 8,
            geode_bots: 2,
        },
    ];
    println!(
        "actual\n{}\nexpected\n{}",
        actual.iter().join("\n"),
        expected.iter().join("\n"),
    );
    assert_eq!(actual, expected);
}
