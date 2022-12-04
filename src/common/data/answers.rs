use std::{error::Error, fs};

use kuchiki::{parse_html, traits::TendrilSink};

use crate::common::{data::req::aoc_request, SimpleResult};

use super::req::post_answer;

pub fn check_answer(year: u32, day: u32, part: u32, answer: &str) -> SimpleResult<bool> {
    Ok(if answer_is_known_incorrect(year, day, part, answer)? {
        false
    } else if let Some(correct_answer) = fetch_correct_answer(year, day, part)? {
        println!(
            "Puzzle already solved, correct answer was `{}`",
            correct_answer
        );
        correct_answer == answer
    } else if post_answer(year, day, part, answer)? {
        true
    } else {
        write_answer_incorrect(year, day, part, answer)?;
        false
    })
}

fn answer_is_known_incorrect(year: u32, day: u32, part: u32, answer: &str) -> SimpleResult<bool> {
    let known_incorrect_answers = read_incorrect_answers(year, day, part)?;
    Ok(known_incorrect_answers.contains(&answer.to_owned()))
}

fn fetch_correct_answer(year: u32, day: u32, part: u32) -> SimpleResult<Option<String>> {
    let url_path = format!("{}/day/{}", year, day);
    let response = aoc_request(url_path)?;
    let html = parse_html().one(response);
    let answer = html
        .select("main > p")
        .unwrap()
        .filter(|element| {
            element
                .text_contents()
                .starts_with("Your puzzle answer was")
        })
        .nth((part - 1) as usize)
        .map(|p| {
            let code = p.as_node().select("code").unwrap().next().unwrap();
            code.text_contents().trim().to_owned()
        });
    Ok(answer)
}

fn read_incorrect_answers(year: u32, day: u32, part: u32) -> SimpleResult<Vec<String>> {
    let data_folder = data_folder(year, day);
    let incorrects_filename = incorrect_answers_filename(year, day, part);
    let contents = fs::read_to_string(&incorrects_filename).or_else(
        |_| -> Result<String, Box<dyn Error>> {
            fs::create_dir_all(&data_folder)?;
            let no_incorrects: Vec<String> = vec![];
            let contents = serde_json::to_string(&no_incorrects)?;
            fs::write(&incorrects_filename, &contents)?;
            Ok(contents)
        },
    )?;
    Ok(serde_json::from_str::<Vec<String>>(&contents)?)
}

fn write_answer_incorrect(year: u32, day: u32, part: u32, answer: &str) -> SimpleResult<()> {
    let mut incorrect_answers = read_incorrect_answers(year, day, part)?;
    incorrect_answers.push(answer.to_owned());
    fs::write(
        incorrect_answers_filename(year, day, part),
        serde_json::to_string(&incorrect_answers)?,
    )?;
    Ok(())
}

fn data_folder(year: u32, day: u32) -> String {
    format!("./data/{}/{}", year, day)
}

fn incorrect_answers_filename(year: u32, day: u32, part: u32) -> String {
    format!("{}/incorrect_part{}.json", data_folder(year, day), part)
}

#[test]
fn test_incorrect_answer() -> SimpleResult<()> {
    write_answer_incorrect(2018, 1, 1, "0")?;
    assert!(answer_is_known_incorrect(2018, 1, 1, "0")?);
    Ok(())
}

#[test]
fn test_correct_answer() -> SimpleResult<()> {
    // Note: this test assumes 2018 is complete up until day 3 part 1
    assert_eq!(fetch_correct_answer(2018, 1, 1)?, Some("582".to_owned()));
    assert_eq!(fetch_correct_answer(2018, 1, 2)?, Some("488".to_owned()));
    assert_eq!(fetch_correct_answer(2018, 2, 1)?, Some("4980".to_owned()));
    assert_eq!(
        fetch_correct_answer(2018, 2, 2)?,
        Some("qysdtrkloagnfozuwujmhrbvx".to_owned())
    );
    assert_eq!(fetch_correct_answer(2018, 3, 1)?, Some("101469".to_owned()));
    assert_eq!(fetch_correct_answer(2018, 3, 2)?, None);
    assert_eq!(fetch_correct_answer(2018, 4, 1)?, None);
    Ok(())
}
