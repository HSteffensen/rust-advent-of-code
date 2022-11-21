use chrono::{DateTime, Duration, Utc};
use kuchiki::{parse_html, traits::TendrilSink};
use std::{
    error::Error,
    fs,
    io::{self, BufRead},
    thread::sleep,
};
use ureq::{self, Cookie};

trait AocSolution {
    const YEAR: u32;
    const DAY: u32;
    const PART: u32;

    fn implementation(&self) -> String;

    fn solve(&self) {}

    fn input(&self) {}
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
    let examples_filename = format!("{}/examples_part{}.txt", data_folder, part);
    fs::read_to_string(&examples_filename)
        .map(|contents| serde_json::from_str::<Vec<(String, String)>>(&contents).unwrap())
        .or_else(|_| -> Result<Vec<(String, String)>, Box<dyn Error>> {
            println!(
                "Couldn't find examples file {}, fetching from adventofcode.com",
                examples_filename
            );
            let fetched_input = fetch_examples(year, day);
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

    let data_folder = format!("./data/{}/{}", year, day);
    let incorrects_filename = format!("{}/incorrect.txt", data_folder);
    let mut known_incorrect_answers = fs::read_to_string(&incorrects_filename)
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
        .unwrap();
    let answer_owned = answer.to_string();
    if known_incorrect_answers.contains(&answer_owned) {
        println!(
            "Already known to be incorrect for {} day {} part {}: `{}`",
            year, day, part, answer
        );
        return false;
    };
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
        known_incorrect_answers.push(answer_owned);
        fs::write(
            incorrects_filename,
            serde_json::to_string(&known_incorrect_answers).unwrap(),
        )
        .unwrap();
        false
    }
}

fn check_one_minute_between_submissions() {
    let last_incorrect_submission_filename = format!("./data/last_incorrect_submission.txt");
    let time_since_last_fail = fs::read_to_string(&last_incorrect_submission_filename)
        .map(|contents| -> Duration {
            Utc::now().signed_duration_since(DateTime::parse_from_rfc3339(&contents).unwrap())
        })
        .or_else(|_| -> Result<Duration, Box<dyn Error>> { Ok(Duration::hours(1)) })
        .unwrap();
    if time_since_last_fail < Duration::minutes(1) {
        let remaining_time = Duration::minutes(1) - time_since_last_fail;
        println!(
            "Too short time between submissions, sleeping for {} seconds before next submission",
            remaining_time.num_seconds()
        );
        sleep(remaining_time.to_std().unwrap());
    }
}

fn load_session_cookie() -> String {
    let cookie_file_name = "./data/.session_cookie";
    fs::read_to_string(cookie_file_name)
        .map(|s| s.trim().to_string())
        .or_else(|_err| -> Result<String, Box<dyn Error>> {
            println!("No session cookie found. Please log in to https://adventofcode.com/ in your browser, open the browser console, copy the value of the 'session' cookie, and paste it here:");
            let mut line = String::new();
            io::stdin().lock().read_line(&mut line)?;
            fs::write(cookie_file_name, &line)?;
            Ok(line.trim().to_string())
        }).unwrap()
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

fn fetch_examples(year: u32, day: u32) -> Vec<(String, String)> {
    let url_path = format!("{}/day/{}", year, day);
    let response = aoc_request(url_path);
    println!("{}", &response);
    let html = parse_html().one(response);
    let mut examples = Vec::new();
    let min_example_length = 5;
    let example_candidates = html
        .select("code")
        .unwrap()
        .map(|node| node.text_contents().trim().into())
        .collect::<Vec<String>>();
    let pre_tag_count = example_candidates.len();
    let example_candidates = example_candidates
        .into_iter()
        .filter(|text| text.len() >= min_example_length)
        .collect::<Vec<String>>();
    println!(
        "Found {} <pre> tags, of which {} are longer than {} characters.",
        pre_tag_count,
        example_candidates.len(),
        min_example_length
    );
    for content in example_candidates {
        println!("Possible example found:\n{}\nIf this is an example, paste the corresponding correct answer. Else, press 'Enter':", content);
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).unwrap();
        let line = line.trim().to_owned();
        if !line.is_empty() {
            examples.push((content, line));
        }
    }
    // let ongoing = true;
    // while ongoing {
    //     println!("Manual example input. 'Enter' to end. '```' to begin multiline example. Otherwise, single-line example:");
    //     let mut line = String::new();
    //     io::stdin().lock().read_line(&mut line).unwrap();
    //     let line = line.trim().to_owned();
    //     if !line.is_empty() {
    //         examples.push((content, line));
    //     }
    // }

    examples
}

fn aoc_request(path: String) -> String {
    let url = format!("https://adventofcode.com/{}", path);
    let cookie = Cookie::new("session", load_session_cookie());
    ureq::get(&url)
        .set("Cookie", &cookie.to_string())
        .call()
        .unwrap()
        .into_string()
        .unwrap()
}

fn post_answer(year: u32, day: u32, part: u32, answer: &str) -> bool {
    let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
    let cookie = Cookie::new("session", load_session_cookie());
    let level = ((day - 1) * 2 + part).to_string();
    let form_body: Vec<(&str, &str)> = vec![("level", &level), ("answer", answer)];
    let response = ureq::post(&url)
        .set("Cookie", &cookie.to_string())
        .send_form(&form_body)
        .unwrap()
        .into_string()
        .unwrap();
    println!("{}", response);
    !response.contains("That's not the right answer.")
}

#[test]
fn test_fetch_input() {
    fetch_input(2018, 1);
    get_input(2018, 1);
}
