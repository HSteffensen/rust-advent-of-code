use crate::common::read_to_ints;

pub fn part1(input: &Vec<i64>) -> String {
    "".to_string()
}

pub fn part2(input: &Vec<i64>) -> String {
    "".to_string()
}

pub fn main() {
    let input_ints = read_to_ints("./src/day01/input.txt");
    print!("{}", part1(&input_ints));
    print!("{}", part2(&input_ints));
}

#[cfg(test)]
mod tests {
    use crate::common::string_to_ints;

    use super::*;
    static TEST_INPUT_1: &str = r#""#;

    #[test]
    fn part_1_test_a() {
        // TODO: do a trait thing to make string passable into &Vec<i64> and &[str]
        assert_eq!("", part1(&string_to_ints(TEST_INPUT_1)))
    }
}
