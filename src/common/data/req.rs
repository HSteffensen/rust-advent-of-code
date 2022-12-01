use std::{
    error::Error,
    fs,
    io::{self, BufRead},
    thread::sleep,
};

use chrono::{DateTime, Duration, Utc};
use ureq::{Cookie, Request};

pub fn aoc_request(path: String) -> String {
    let url = format!("https://adventofcode.com/{}", path);
    set_headers(ureq::get(&url))
        .call()
        .unwrap()
        .into_string()
        .unwrap()
}

pub fn post_answer(year: u32, day: u32, part: u32, answer: &str) -> bool {
    check_one_minute_between_submissions();
    let url = format!("https://adventofcode.com/{}/day/{}/answer", year, day);
    let level = part.to_string();
    let form_body: Vec<(&str, &str)> = vec![("level", &level), ("answer", answer)];
    println!("Posting answer {} to {}", answer, url);
    let response = set_headers(ureq::post(&url))
        .send_form(&form_body)
        .unwrap()
        .into_string()
        .unwrap();
    if response.contains("That's the right answer") {
        true
    } else if response.contains("That's not the right answer") {
        let message = response
            .split("That's not the right answer")
            .collect::<Vec<&str>>()[1]
            .split('.')
            .collect::<Vec<&str>>()[0]
            .trim_start_matches("; ");
        if !message.is_empty() {
            println!("Wrong: '{}'", message);
        }
        false
    } else {
        panic!(
            "{}\n----\nSomething went wrong when submitting the answer. See above HTML output.",
            response
        );
    }
}

fn set_headers(request: Request) -> Request {
    let cookie = Cookie::new("session", load_session_cookie());
    request.set("Cookie", &cookie.to_string()).set(
        "User-Agent",
        "https://github.com/HSteffensen/rust-advent-of-code by henry@steffensenfamily.com",
    )
}

fn check_one_minute_between_submissions() {
    let path = &"./data/last_submission_time.txt";
    let time_since_last_submission = fs::read_to_string(path)
        .map(|contents| -> Duration {
            Utc::now().signed_duration_since(DateTime::parse_from_rfc3339(&contents).unwrap())
        })
        .or_else(|_| -> Result<Duration, Box<dyn Error>> { Ok(Duration::hours(1)) })
        .unwrap();
    if time_since_last_submission < Duration::minutes(1) {
        let remaining_time =
            Duration::minutes(1) - time_since_last_submission + Duration::seconds(1);
        println!(
            "Too short time between submissions, sleeping for {} seconds before next submission",
            remaining_time.num_seconds()
        );
        sleep(remaining_time.to_std().unwrap());
    }
    fs::write(path, Utc::now().to_rfc3339()).unwrap();
}

fn load_session_cookie() -> String {
    let cookie_file_name = "./data/.session_cookie";
    fs::read_to_string(cookie_file_name)
        .map(|s| s.trim().to_string())
        .or_else(|_err| -> Result<String, Box<dyn Error>> {
            println!("No session cookie found. Please log in to https://adventofcode.com/ in your browser, open the browser console, copy the value of the 'session' cookie, and paste it here:");
            let mut line = String::new();
            io::stdin().lock().read_line(&mut line)?;
            fs::create_dir("./data/").unwrap();
            fs::write(cookie_file_name, &line)?;
            Ok(line.trim().to_string())
        }).unwrap()
}

#[test]
fn check_one_minute() {
    check_one_minute_between_submissions()
}
