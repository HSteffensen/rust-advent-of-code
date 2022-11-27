use chrono::{DateTime, Duration, Utc};
use std::{error::Error, fs, thread::sleep};

use crate::common::data::{
    examples::fetch_examples,
    req::{aoc_request, post_answer},
};

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
    check_one_minute_between_submissions();

    if answer_is_known_incorrect(year, day, part, answer) {
        println!(
            "Already known to be incorrect for {} day {} part {}: `{}`",
            year, day, part, answer
        );
        return false;
    }

    if post_answer(year, day, part, answer) {
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
        write_answer_incorrect(year, day, part, answer);
        false
    }
}

fn data_folder(year: u32, day: u32) -> String {
    format!("./data/{}/{}", year, day)
}

fn incorrect_answers_filename(year: u32, day: u32, part: u32) -> String {
    format!("{}/incorrect_part{}.json", data_folder(year, day), part)
}

fn check_one_minute_between_submissions() {
    let time_since_last_fail = fs::read_to_string("./data/last_incorrect_submission.txt")
        .map(|contents| -> Duration {
            Utc::now().signed_duration_since(DateTime::parse_from_rfc3339(&contents).unwrap())
        })
        .or_else(|_| -> Result<Duration, Box<dyn Error>> { Ok(Duration::hours(1)) })
        .unwrap();
    if time_since_last_fail < Duration::minutes(1) {
        let remaining_time = Duration::minutes(1) - time_since_last_fail + Duration::seconds(1);
        println!(
            "Too short time between submissions, sleeping for {} seconds before next submission",
            remaining_time.num_seconds()
        );
        sleep(remaining_time.to_std().unwrap());
    }
}

fn answer_is_known_incorrect(year: u32, day: u32, part: u32, answer: &str) -> bool {
    let known_incorrect_answers = read_incorrect_answers(year, day, part);
    known_incorrect_answers.contains(&answer.to_owned())
}

fn read_incorrect_answers(year: u32, day: u32, part: u32) -> Vec<String> {
    let data_folder = data_folder(year, day);
    let incorrects_filename = incorrect_answers_filename(year, day, part);
    fs::read_to_string(&incorrects_filename)
        .map(|contents| serde_json::from_str::<Vec<String>>(&contents).unwrap())
        .or_else(|_| -> Result<Vec<String>, Box<dyn Error>> {
            fs::create_dir_all(&data_folder).unwrap();
            let no_incorrects: Vec<String> = vec![];
            fs::write(
                &incorrects_filename,
                serde_json::to_string(&no_incorrects).unwrap(),
            )
            .unwrap();
            Ok(no_incorrects)
        })
        .unwrap()
}

fn write_answer_incorrect(year: u32, day: u32, part: u32, answer: &str) {
    let mut incorrect_answers = read_incorrect_answers(year, day, part);
    incorrect_answers.push(answer.to_owned());
    fs::write(
        incorrect_answers_filename(year, day, part),
        serde_json::to_string(&incorrect_answers).unwrap(),
    )
    .unwrap();
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

#[test]
fn test_incorrect_answer() {
    write_answer_incorrect(2018, 1, 1, "0");
    assert!(answer_is_known_incorrect(2018, 1, 1, "0"));
}
