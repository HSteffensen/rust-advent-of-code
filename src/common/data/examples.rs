use std::io::{self, BufRead};

use kuchiki::{parse_html, traits::TendrilSink};

use crate::common::{data::req::aoc_request, SimpleResult};

pub fn fetch_examples(year: u32, day: u32, part: u32) -> SimpleResult<Vec<(String, String)>> {
    let url_path = format!("{}/day/{}", year, day);
    let response = aoc_request(url_path)?;
    let html = parse_html().one(response);
    let mut examples = Vec::new();
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
    let mut example_candidates = example_candidates
        .into_iter()
        .filter(|text| text.contains('\n'))
        .collect::<Vec<String>>();
    println!(
        "Found {} <code> tags, of which {} contain a newline.",
        pre_tag_count,
        example_candidates.len()
    );
    if part == 2 {
        let part1_examples = fetch_examples(year, day, 1)?;
        let mut part1_example_inputs: Vec<String> =
            part1_examples.iter().map(|(e, _)| e.to_owned()).collect();
        example_candidates.append(&mut part1_example_inputs);
    }
    for content in example_candidates {
        println!("Possible example found:\n{}\nIf this is an example, paste the corresponding correct answer. Else, press 'Enter':", content);
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line)?;
        let line = line.trim().to_owned();
        if !line.is_empty() {
            examples.push((content, line));
        }
    }
    Ok(examples)
}
