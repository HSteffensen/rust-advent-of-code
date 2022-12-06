use std::{error::Error, fs};

use crate::common::data::answers::check_answer;
use crate::common::data::{examples::fetch_examples, req::aoc_request};

use super::SimpleResult;

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

pub fn get_input(year: u32, day: u32) -> SimpleResult<String> {
    assert!((2015..3000).contains(&year));
    assert!((1..=25).contains(&day));
    let data_folder = format!("./data/{}/{}", year, day);
    let input_filename = format!("{}/input.txt", data_folder);
    fs::read_to_string(&input_filename).or_else(|_| -> Result<String, Box<dyn Error>> {
        println!(
            "Couldn't find input file {}, fetching from adventofcode.com",
            input_filename
        );
        let fetched_input = fetch_input(year, day)?;
        fs::create_dir_all(&data_folder)?;
        fs::write(input_filename, &fetched_input)?;
        Ok(fetched_input)
    })
}

pub fn get_examples(year: u32, day: u32, part: u32) -> SimpleResult<Vec<(String, String)>> {
    assert!((2015..3000).contains(&year));
    assert!((1..=25).contains(&day));
    assert!((1..=2).contains(&part));
    let data_folder = format!("./data/{}/{}", year, day);
    let examples_filename = format!("{}/examples_part{}.json", data_folder, part);
    let examples_json =
        fs::read_to_string(&examples_filename).or_else(|_| -> Result<String, Box<dyn Error>> {
            println!(
                "Couldn't find examples file {}, fetching from adventofcode.com",
                examples_filename
            );
            let fetched_examples = fetch_examples(year, day, part)?;
            let examples_str = serde_json::to_string(&fetched_examples)?;
            fs::create_dir_all(&data_folder)?;
            fs::write(examples_filename, &examples_str)?;
            Ok(examples_str)
        })?;
    Ok(serde_json::from_str::<Vec<(String, String)>>(
        &examples_json,
    )?)
}

pub fn submit_answer(year: u32, day: u32, part: u32, answer: &str) -> SimpleResult<()> {
    assert!((2015..3000).contains(&year));
    assert!((1..=25).contains(&day));
    assert!((1..=2).contains(&part));

    if check_answer(year, day, part, answer)? {
        println!(
            "Correct answer submitted for {} day {} part {}: `{}`!",
            year, day, part, answer
        );
    } else {
        println!(
            "Incorrect answer submitted for {} day {} part {}: `{}`",
            year, day, part, answer
        );
    }
    Ok(())
}

fn fetch_input(year: u32, day: u32) -> SimpleResult<String> {
    let url_path = format!("{}/day/{}/input", year, day);
    let response = aoc_request(url_path)?;
    assert!(
        !response.starts_with("Please don't"),
        "{} day {} has no input",
        year,
        day
    );
    Ok(response)
}

#[test]
fn test_fetch_input() {
    fetch_input(2018, 1).unwrap();
    get_input(2018, 1).unwrap();
}
