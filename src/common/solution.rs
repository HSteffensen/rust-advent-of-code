use super::data::{get_examples, get_input, submit_answer};

pub trait AocSolution {
    const YEAR: u32;
    const DAY: u32;
    const PART: u32;

    fn implementation(input: &str) -> String;

    fn solve() {
        let examples = Self::get_examples();
        for (i, (example, expected)) in examples.iter().enumerate() {
            let actual = Self::implementation(example);
            if &actual == expected {
                println!("{}: Example {} passed.", Self::ydp(), i);
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
        let answer = Self::implementation(&input);
        if Self::do_post_answer() {
            submit_answer(Self::YEAR, Self::DAY, Self::PART, &answer).unwrap();
        } else {
            println!(
                "Printing answer rather than posting to the AoC website:\n{}",
                answer
            );
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
