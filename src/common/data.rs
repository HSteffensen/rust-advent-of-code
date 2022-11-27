use std::{error::Error, fs};

use crate::common::data::answers::{answer_is_known_incorrect, check_answer};
use crate::common::data::{examples::fetch_examples, req::aoc_request};

mod answers;
mod examples;
mod req;

pub fn input_to_ints(input: &str) -> Vec<i64> {
    let mut numbers: Vec<i64> = Vec::new();
    for line in input.lines() {
        if let Ok(line_as_number) = line.parse() {
            numbers.push(line_as_number);
        }
    }
    numbers
}

pub fn get_input(year: u32, day: u32) -> String {
    assert!((2015..3000).contains(&year));
    assert!((1..=25).contains(&day));
    let data_folder = format!("./data/{}/{}", year, day);
    let input_filename = format!("{}/input.txt", data_folder);
    fs::read_to_string(&input_filename)
        .or_else(|_| -> Result<String, Box<dyn Error>> {
            println!(
                "Couldn't find input file {}, fetching from adventofcode.com",
                input_filename
            );
            let fetched_input = fetch_input(year, day);
            fs::create_dir_all(&data_folder).unwrap();
            fs::write(input_filename, &fetched_input).unwrap();
            Ok(fetched_input)
        })
        .unwrap()
}

pub fn get_examples(year: u32, day: u32, part: u32) -> Vec<(String, String)> {
    assert!((2015..3000).contains(&year));
    assert!((1..=25).contains(&day));
    assert!((1..=2).contains(&part));
    let data_folder = format!("./data/{}/{}", year, day);
    let examples_filename = format!("{}/examples_part{}.json", data_folder, part);
    fs::read_to_string(&examples_filename)
        .map(|contents| serde_json::from_str::<Vec<(String, String)>>(&contents).unwrap())
        .or_else(|_| -> Result<Vec<(String, String)>, Box<dyn Error>> {
            println!(
                "Couldn't find examples file {}, fetching from adventofcode.com",
                examples_filename
            );
            let fetched_input = fetch_examples(year, day, part);
            let examples_str = serde_json::to_string(&fetched_input).unwrap();
            fs::create_dir_all(&data_folder).unwrap();
            fs::write(examples_filename, &examples_str).unwrap();
            Ok(fetched_input)
        })
        .unwrap()
}

pub fn submit_answer(year: u32, day: u32, part: u32, answer: &str) -> bool {
    assert!((2015..3000).contains(&year));
    assert!((1..=25).contains(&day));
    assert!((1..=2).contains(&part));

    if answer_is_known_incorrect(year, day, part, answer) {
        println!(
            "Already known to be incorrect for {} day {} part {}: `{}`",
            year, day, part, answer
        );
        return false;
    }

    if check_answer(year, day, part, answer) {
        println!(
            "Correct answer submitted for {} day {} part {}: `{}`!",
            year, day, part, answer
        );
        true
    } else {
        println!(
            "Incorrect answer submitted for {} day {} part {}: `{}`",
            year, day, part, answer
        );
        false
    }
}

fn fetch_input(year: u32, day: u32) -> String {
    let url_path = format!("{}/day/{}/input", year, day);
    let response = aoc_request(url_path);
    assert!(
        !response.starts_with("Please don't"),
        "{} day {} has no input",
        year,
        day
    );
    response
}

#[test]
fn test_fetch_input() {
    fetch_input(2018, 1);
    get_input(2018, 1);
}
