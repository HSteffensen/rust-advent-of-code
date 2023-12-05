use std::time::Instant;

use super::data::{get_examples, get_input, submit_answer};

pub trait AocSolution {
    const YEAR: u32;
    const DAY: u32;
    const PART: u32;

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
        let input = get_input(Self::YEAR, Self::DAY).unwrap();
        let start = Instant::now();
        let answer = Self::implementation(&input);
        let elapsed = start.elapsed();
        println!("Answer `{}`; Solution ran in {:?}", answer, elapsed);
        if Self::do_post_answer() {
            submit_answer(Self::YEAR, Self::DAY, Self::PART, &answer).unwrap();
        }
    }

    fn do_post_answer() -> bool {
        true
    }

    fn ydp() -> String {
        format!("y{}d{}p{}", Self::YEAR, Self::DAY, Self::PART)
    }

    fn map_example_input(example: &str) -> String {
        example.to_string()
    }

    fn get_examples() -> Vec<(String, String)> {
        get_examples(Self::YEAR, Self::DAY, Self::PART)
            .unwrap()
            .iter()
            .map(|(example, expected)| -> (String, String) {
                (Self::map_example_input(example), expected.to_string())
            })
            .collect()
    }
}
