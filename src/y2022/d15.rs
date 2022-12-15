use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, newline},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::common::solution::AocSolution;

use super::Y;

struct Part1 {}
struct Part2 {}
const D: u32 = 15;

#[derive(Debug)]
struct Point2d(i32, i32);

impl Point2d {
    fn distance_manhattan(&self, other: &Point2d) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[derive(Debug)]
struct Sensor {
    position: Point2d,
    nearest_beacon: Point2d,
}

impl Sensor {
    fn beacon_distance(&self) -> u32 {
        self.position.distance_manhattan(&self.nearest_beacon)
    }

    fn covered_points_in_row(&self, row: i32) -> Option<(i32, i32)> {
        let row_distance = row.abs_diff(self.position.1);
        let beacon_distance = self.beacon_distance();
        if row_distance > beacon_distance {
            return None;
        }
        let row_width = (beacon_distance - row_distance) as i32;
        Some((self.position.0 - row_width, self.position.0 + row_width))
    }
}

#[test]
fn test_sensor_covered_points_in_row() {
    assert_eq!(
        Sensor {
            position: Point2d(0, 0),
            nearest_beacon: Point2d(1, 0)
        }
        .covered_points_in_row(1),
        Some((0, 0))
    );
    assert_eq!(
        Sensor {
            position: Point2d(8, 7),
            nearest_beacon: Point2d(2, 10)
        }
        .covered_points_in_row(1),
        Some((5, 11))
    );
    assert_eq!(
        Sensor {
            position: Point2d(0, 11),
            nearest_beacon: Point2d(2, 10)
        }
        .covered_points_in_row(13),
        Some((-2, 2))
    );
}

fn combine_overlapping_ranges(ranges: &[(i32, i32)]) -> Vec<(i32, i32)> {
    if ranges.is_empty() {
        return vec![];
    }
    let mut result = vec![];
    let mut ranges = ranges.iter().sorted_by(|(a, _), (b, _)| Ord::cmp(a, b));
    let current_range = ranges.next().unwrap();
    let mut current_range = (current_range.0, current_range.1);
    for range in ranges {
        if current_range.1 < (range.0 - 1) {
            result.push(current_range);
            current_range = (range.0, range.1);
        } else {
            current_range.1 = current_range.1.max(range.1);
        }
    }
    result.push(current_range);
    result
}

#[test]
fn test_combine_overlapping_ranges() {
    assert_eq!(combine_overlapping_ranges(&[]), vec![]);
    assert_eq!(combine_overlapping_ranges(&[(0, 8)]), vec![(0, 8)]);
    assert_eq!(
        combine_overlapping_ranges(&[(0, 8), (8, 10)]),
        vec![(0, 10)]
    );
    assert_eq!(
        combine_overlapping_ranges(&[(0, 7), (8, 10)]),
        vec![(0, 10)]
    );
    assert_eq!(
        combine_overlapping_ranges(&[(8, 11), (6, 14), (3, 4), (8, 10), (0, 2), (10, 15)]),
        vec![(0, 4), (6, 15)]
    );
}

fn parse_line(line: &str) -> IResult<&str, Sensor> {
    map(
        tuple((
            tag("Sensor at x="),
            complete::i32,
            tag(", y="),
            complete::i32,
            tag(": closest beacon is at x="),
            complete::i32,
            tag(", y="),
            complete::i32,
        )),
        |(_, sx, _, sy, _, bx, _, by)| Sensor {
            position: Point2d(sx, sy),
            nearest_beacon: Point2d(bx, by),
        },
    )(line)
}

fn parse_input(input: &str) -> Vec<Sensor> {
    let (_, sensors) = separated_list0(newline, parse_line)(input).unwrap();
    sensors
}

impl AocSolution for Part1 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 1;

    fn map_example_input(example: &str) -> String {
        format!("test\n{}", example)
    }

    fn implementation(input: &str) -> String {
        let row = if input.starts_with("test\n") {
            10
        } else {
            2000000
        };
        let input = input.trim_start_matches("test\n");
        let sensors = parse_input(input);
        let beacon_columns = sensors
            .iter()
            .filter_map(|s| {
                if s.nearest_beacon.1 == row {
                    Some(s.nearest_beacon.0)
                } else {
                    None
                }
            })
            .unique()
            .collect_vec();
        let ranges = sensors
            .iter()
            .filter_map(|s| s.covered_points_in_row(row))
            .collect_vec();
        let combined_ranges = combine_overlapping_ranges(&ranges);
        let beacons_within_combined_ranges = beacon_columns
            .iter()
            .filter(|b| combined_ranges.iter().any(|(r0, r1)| r0 <= b && *b <= r1))
            .count() as i32;
        let range_count = combined_ranges.iter().map(|(a, b)| b - a + 1).sum::<i32>();
        (range_count - beacons_within_combined_ranges).to_string()
    }
}

impl AocSolution for Part2 {
    const YEAR: u32 = Y;
    const DAY: u32 = D;
    const PART: u32 = 2;

    fn map_example_input(example: &str) -> String {
        format!("test\n{}", example)
    }

    fn implementation(input: &str) -> String {
        let grid_size = if input.starts_with("test\n") {
            20
        } else {
            4000000
        };
        let input = input.trim_start_matches("test\n");
        let sensors = parse_input(input);
        for row in 0..grid_size {
            let ranges = sensors
                .iter()
                .filter_map(|s| s.covered_points_in_row(row))
                .collect_vec();
            let combined_ranges = combine_overlapping_ranges(&ranges);
            if combined_ranges.len() > 1 {
                let col = (combined_ranges.first().unwrap().1 + 1) as u64;
                return (col * 4000000 + row as u64).to_string();
            }
        }
        unreachable!()
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
