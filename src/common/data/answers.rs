use std::{error::Error, fs};

use kuchiki::{parse_html, traits::TendrilSink};

use crate::common::data::req::aoc_request;

use super::req::post_answer;

pub fn check_answer(year: u32, day: u32, part: u32, answer: &str) -> bool {
    if answer_is_known_incorrect(year, day, part, answer) {
        false
    } else if let Some(correct_answer) = fetch_correct_answer(year, day, part) {
        println!(
            "Puzzle already solved, correct answer was `{}`",
            correct_answer
        );
        correct_answer == answer
    } else if post_answer(year, day, part, answer) {
        true
    } else {
        write_answer_incorrect(year, day, part, answer);
        false
    }
}

fn answer_is_known_incorrect(year: u32, day: u32, part: u32, answer: &str) -> bool {
    let known_incorrect_answers = read_incorrect_answers(year, day, part);
    known_incorrect_answers.contains(&answer.to_owned())
}

fn fetch_correct_answer(year: u32, day: u32, part: u32) -> Option<String> {
    let url_path = format!("{}/day/{}", year, day);
    let response = aoc_request(url_path);
    let html = parse_html().one(response);
    let part_answer = html.select("main > p").unwrap().nth((part - 1) as usize);
    let p_contents = part_answer.as_ref().unwrap().text_contents();
    assert!(
        p_contents.starts_with("Your puzzle answer was"),
        "{}\nExpected puzzle answer, but got the above.",
        p_contents
    );
    part_answer.map(|p| {
        p.as_node()
            .select("code")
            .unwrap()
            .map(|node| node.text_contents().trim().to_owned())
            .next()
            .unwrap()
    })
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

fn data_folder(year: u32, day: u32) -> String {
    format!("./data/{}/{}", year, day)
}

fn incorrect_answers_filename(year: u32, day: u32, part: u32) -> String {
    format!("{}/incorrect_part{}.json", data_folder(year, day), part)
}

#[test]
fn test_incorrect_answer() {
    write_answer_incorrect(2018, 1, 1, "0");
    assert!(answer_is_known_incorrect(2018, 1, 1, "0"));
}

#[test]
fn test_correct_answer() {
    assert_eq!(fetch_correct_answer(2018, 1, 1), Some("582".to_owned()));
    assert_eq!(fetch_correct_answer(2018, 3, 1), Some("101469".to_owned()));
}
