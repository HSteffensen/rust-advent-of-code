use super::{
    data::{get_examples, get_input, submit_answer},
    SimpleResult,
};

pub trait AocSolution {
    const YEAR: u32;
    const DAY: u32;
    const PART: u32;

    fn implementation(input: &str) -> String;

    fn solve() -> SimpleResult<()> {
        let examples = Self::get_examples()?;
        for (i, (example, expected)) in examples.iter().enumerate() {
            let actual = Self::implementation(example);
            if &actual == expected {
                println!("{}: Example {} passed.", Self::ydp(), i);
            } else {
                panic!(
                    "{}: Example {} failed.\nExample input:\n{}\nExpected: '{}'\nGot: '{}'",
                    Self::ydp(),
                    i,
                    example,
                    expected,
                    actual,
                )
            }
        }
        let input = get_input(Self::YEAR, Self::DAY)?;
        let answer = Self::implementation(&input);
        submit_answer(Self::YEAR, Self::DAY, Self::PART, &answer)?;
        Ok(())
    }

    fn ydp() -> String {
        format!("y{}d{}p{}", Self::YEAR, Self::DAY, Self::PART)
    }

    fn map_example_input(example: &str) -> String {
        example.to_string()
    }

    fn get_examples() -> SimpleResult<Vec<(String, String)>> {
        Ok(get_examples(Self::YEAR, Self::DAY, Self::PART)?
            .iter()
            .map(|(example, expected)| -> (String, String) {
                (Self::map_example_input(example), expected.to_string())
            })
            .collect())
    }
}
