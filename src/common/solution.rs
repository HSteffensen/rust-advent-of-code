use std::time::Instant;

use super::data::{get_examples, get_input, submit_answer};

pub trait AocSolution {
    const PART: u32;

    fn solution_path() -> String {
        "".to_string()
    }

    fn year_day() -> (u32, u32) {
        let year = Self::solution_path()
            .split("::")
            .find(|part| part.starts_with('y'))
            .map(|part| part.replace('y', "").parse().unwrap())
            .unwrap();
        let day = Self::solution_path()
            .split("::")
            .find(|part| part.starts_with('d'))
            .map(|part| part.replace('d', "").parse().unwrap())
            .unwrap();
        (year, day)
    }

    fn implementation(input: &str) -> String;

    fn solve() {
        let examples = Self::get_examples();
        for (i, (example, expected)) in examples.iter().enumerate() {
            let start = Instant::now();
            let actual = Self::implementation(example);
            let elapsed = start.elapsed();
            if &actual == expected {
                println!("{}: Example {} passed in {:?}", Self::ydp(), i, elapsed);
            } else {
                panic!(
                    "\n{}: Example {} failed.\nExample input:\n{}\nExpected: `{}`\nGot: `{}`\n",
                    Self::ydp(),
                    i,
                    example,
                    expected,
                    actual,
                )
            }
        }
        let (year, day) = Self::year_day();
        let input = get_input(year, day).unwrap();
        let start = Instant::now();
        let answer = Self::implementation(&input);
        let elapsed = start.elapsed();
        println!("Answer `{}`; Solution ran in {:?}", answer, elapsed);
        if Self::do_post_answer() {
            submit_answer(year, day, Self::PART, &answer).unwrap();
        }
    }

    fn do_post_answer() -> bool {
        true
    }

    fn ydp() -> String {
        let (year, day) = Self::year_day();
        format!("y{}d{}p{}", year, day, Self::PART)
    }

    fn map_example_input(example: &str) -> String {
        example.to_string()
    }

    fn get_examples() -> Vec<(String, String)> {
        let (year, day) = Self::year_day();
        get_examples(year, day, Self::PART)
            .unwrap()
            .iter()
            .map(|(example, expected)| -> (String, String) {
                (Self::map_example_input(example), expected.to_string())
            })
            .collect()
    }
}
