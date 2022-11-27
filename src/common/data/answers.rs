use std::{error::Error, fs};

use super::req::post_answer;

pub fn answer_is_known_incorrect(year: u32, day: u32, part: u32, answer: &str) -> bool {
    let known_incorrect_answers = read_incorrect_answers(year, day, part);
    known_incorrect_answers.contains(&answer.to_owned())
}

pub fn check_answer(year: u32, day: u32, part: u32, answer: &str) -> bool {
    if let Some(correct_answer) = read_correct_answer(year, day, part) {
        correct_answer == answer
    } else if post_answer(year, day, part, answer) {
        write_correct_answer(year, day, part, answer);
        true
    } else {
        write_answer_incorrect(year, day, part, answer);
        false
    }
}

fn read_correct_answer(year: u32, day: u32, part: u32) -> Option<String> {
    let correct_filename = correct_answers_filename(year, day, part);
    fs::read_to_string(&correct_filename)
        .map(|contents| Some(contents.trim().to_owned()))
        .unwrap_or(None)
}

fn write_correct_answer(year: u32, day: u32, part: u32, answer: &str) {
    let data_folder = data_folder(year, day);
    let correct_filename = correct_answers_filename(year, day, part);
    fs::write(&correct_filename, answer)
        .or_else(|_| -> Result<(), Box<dyn Error>> {
            fs::create_dir_all(&data_folder).unwrap();
            fs::write(&correct_filename, answer).unwrap();
            Ok(())
        })
        .unwrap();
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

fn correct_answers_filename(year: u32, day: u32, part: u32) -> String {
    format!("{}/correct_part{}.txt", data_folder(year, day), part)
}

#[test]
fn test_incorrect_answer() {
    write_answer_incorrect(2018, 1, 1, "0");
    assert!(answer_is_known_incorrect(2018, 1, 1, "0"));
}
