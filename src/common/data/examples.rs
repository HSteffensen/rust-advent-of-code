use std::io::{self, BufRead};

use kuchiki::{parse_html, traits::TendrilSink};

use crate::common::data::req::aoc_request;

pub fn fetch_examples(year: u32, day: u32, part: u32) -> Vec<(String, String)> {
    let url_path = format!("{}/day/{}", year, day);
    let response = aoc_request(url_path).unwrap();
    let html = parse_html().one(response);
    let mut examples = Vec::new();
    let min_example_length = 5;
    let example_candidates = html
        .select("article.day-desc")
        .unwrap()
        .nth((part - 1) as usize)
        .unwrap_or_else(|| panic!("Day {} part {} is not available", day, part))
        .as_node()
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
        "Found {} <code> tags, of which {} are longer than {} characters.",
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
    examples
}
