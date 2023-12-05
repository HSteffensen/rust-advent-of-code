use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 5;

struct AlmanacMapping {
    destination_start: u64,
    source_start: u64,
    length: u64,
}

impl AlmanacMapping {
    fn contains_source(&self, source: &u64) -> bool {
        let source_end = self.source_start + self.length;
        (self.source_start..source_end).contains(source)
    }

    fn map_source(&self, source: u64) -> Option<u64> {
        Some(source)
            .filter(|s| self.contains_source(s))
            .map(|s| s + self.destination_start - self.source_start)
    }
}

struct Almanac {
    seeds: Vec<u64>,
    seed_to_soil: Vec<AlmanacMapping>,
    soil_to_fertilizer: Vec<AlmanacMapping>,
    fertilizer_to_water: Vec<AlmanacMapping>,
    water_to_light: Vec<AlmanacMapping>,
    light_to_temperature: Vec<AlmanacMapping>,
    temperature_to_humidity: Vec<AlmanacMapping>,
    humidity_to_location: Vec<AlmanacMapping>,
}

struct AlmanacItem {
    id: u64,
}

impl AlmanacItem {
    fn map_source(&self, mappings: &Vec<AlmanacMapping>) -> AlmanacItem {
        let mapped = mappings
            .iter()
            .find_map(|mapping| mapping.map_source(self.id))
            .unwrap_or(self.id);
        AlmanacItem { id: mapped }
    }
}

impl Almanac {
    fn seed_to_location(&self, seed: u64) -> u64 {
        AlmanacItem { id: seed }
            .map_source(&self.seed_to_soil)
            .map_source(&self.soil_to_fertilizer)
            .map_source(&self.fertilizer_to_water)
            .map_source(&self.water_to_light)
            .map_source(&self.light_to_temperature)
            .map_source(&self.temperature_to_humidity)
            .map_source(&self.humidity_to_location)
            .id
    }
}

fn parse_mappings(input: &str) -> IResult<&str, Vec<AlmanacMapping>> {
    separated_list1(
        newline,
        map(
            tuple((complete::u64, space1, complete::u64, space1, complete::u64)),
            |(d, _, s, _, l)| AlmanacMapping {
                destination_start: d,
                source_start: s,
                length: l,
            },
        ),
    )(input)
}

fn parse_input(input: &str) -> Almanac {
    map(
        separated_pair(
            preceded(tag("seeds: "), separated_list1(space1, complete::u64)),
            tag("\n\n"),
            separated_pair(
                preceded(tag("seed-to-soil map:\n"), parse_mappings),
                tag("\n\n"),
                separated_pair(
                    preceded(tag("soil-to-fertilizer map:\n"), parse_mappings),
                    tag("\n\n"),
                    separated_pair(
                        preceded(tag("fertilizer-to-water map:\n"), parse_mappings),
                        tag("\n\n"),
                        separated_pair(
                            preceded(tag("water-to-light map:\n"), parse_mappings),
                            tag("\n\n"),
                            separated_pair(
                                preceded(tag("light-to-temperature map:\n"), parse_mappings),
                                tag("\n\n"),
                                separated_pair(
                                    preceded(tag("temperature-to-humidity map:\n"), parse_mappings),
                                    tag("\n\n"),
                                    preceded(tag("humidity-to-location map:\n"), parse_mappings),
                                ),
                            ),
                        ),
                    ),
                ),
            ),
        ),
        |(seeds, (soils, (ferts, (waters, (lights, (temps, (humids, locs)))))))| Almanac {
            seeds,
            seed_to_soil: soils,
            soil_to_fertilizer: ferts,
            fertilizer_to_water: waters,
            water_to_light: lights,
            light_to_temperature: temps,
            temperature_to_humidity: humids,
            humidity_to_location: locs,
        },
    )(input)
    .unwrap()
    .1
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn implementation(input: &str) -> String {
        let almanac = parse_input(input);
        almanac
            .seeds
            .iter()
            .map(|seed| almanac.seed_to_location(*seed))
            .min()
            .unwrap()
            .to_string()
    }
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn implementation(input: &str) -> String {
        let almanac = parse_input(input);
        almanac
            .seeds
            .chunks_exact(2)
            .flat_map(|s| {
                println!("chunk starting at {}", s[0]);
                s[0]..(s[0] + s[1])
            })
            .map(|seed| almanac.seed_to_location(seed))
            .min()
            .unwrap()
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
