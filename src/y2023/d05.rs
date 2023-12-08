use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, space1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, separated_pair, tuple},
    IResult,
};

use crate::common::solution::AocSolution;

struct Part1 {}
struct Part2 {}

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

    fn contains_destination(&self, destination: &u64) -> bool {
        let destination_end = self.destination_start + self.length;
        (self.destination_start..destination_end).contains(destination)
    }

    fn map_destination_to_source(&self, destination: u64) -> Option<u64> {
        Some(destination)
            .filter(|s| self.contains_destination(s))
            .map(|s| s + self.source_start - self.destination_start)
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
    value: u64,
}

impl AlmanacItem {
    fn map_source(&self, mappings: &[AlmanacMapping]) -> AlmanacItem {
        let mapped = mappings
            .iter()
            .find_map(|mapping| mapping.map_source(self.value))
            .unwrap_or(self.value);
        AlmanacItem { value: mapped }
    }

    fn map_destination_to_source(&self, mappings: &[AlmanacMapping]) -> AlmanacItem {
        let mapped = mappings
            .iter()
            .find_map(|mapping| mapping.map_destination_to_source(self.value))
            .unwrap_or(self.value);
        AlmanacItem { value: mapped }
    }
}

impl Almanac {
    fn seed_to_location(&self, seed: u64) -> u64 {
        AlmanacItem { value: seed }
            .map_source(&self.seed_to_soil)
            .map_source(&self.soil_to_fertilizer)
            .map_source(&self.fertilizer_to_water)
            .map_source(&self.water_to_light)
            .map_source(&self.light_to_temperature)
            .map_source(&self.temperature_to_humidity)
            .map_source(&self.humidity_to_location)
            .value
    }

    fn location_to_seed(&self, location: u64) -> u64 {
        AlmanacItem { value: location }
            .map_destination_to_source(&self.humidity_to_location)
            .map_destination_to_source(&self.temperature_to_humidity)
            .map_destination_to_source(&self.light_to_temperature)
            .map_destination_to_source(&self.water_to_light)
            .map_destination_to_source(&self.fertilizer_to_water)
            .map_destination_to_source(&self.soil_to_fertilizer)
            .map_destination_to_source(&self.seed_to_soil)
            .value
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
    const PART: u32 = 1;
    fn solution_path() -> String {
        module_path!().to_string()
    }

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
    const PART: u32 = 2;
    fn solution_path() -> String {
        module_path!().to_string()
    }

    fn implementation(input: &str) -> String {
        let almanac = parse_input(input);
        let seed_ranges = almanac
            .seeds
            .chunks(2)
            .map(|seed| seed[0]..(seed[0] + seed[1]))
            .collect_vec();
        let some_location = almanac.seed_to_location(almanac.seeds[0]);
        (0..some_location)
            .find(|loc| {
                let seed = almanac.location_to_seed(*loc);
                seed_ranges.iter().any(|r| r.contains(&seed))
            })
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
