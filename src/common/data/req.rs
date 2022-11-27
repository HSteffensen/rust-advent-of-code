use std::{
    error::Error,
    fs,
    io::{self, BufRead},
};

use ureq::Cookie;

pub fn aoc_request(path: String) -> String {
    let url = format!("https://adventofcode.com/{}", path);
    let cookie = Cookie::new("session", load_session_cookie());
    ureq::get(&url)
        .set("Cookie", &cookie.to_string())
        .call()
        .unwrap()
        .into_string()
        .unwrap()
}

pub fn post_answer(year: u32, day: u32, part: u32, answer: &str) -> bool {
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
    response.contains("That's the right answer!")
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
