use std::{fs::File, io::Read};

pub mod data;

pub fn read_input(filename: &str) -> Vec<String> {
    let mut file = File::open(filename).expect("file does not exist");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("problem reading file");
    let mut lines: Vec<String> = Vec::new();
    for line in contents.lines() {
        lines.push(line.into());
    }
    lines
}

pub fn strings_to_ints<I>(lines: I) -> Vec<i64>
where
    I: IntoIterator,
    I::Item: AsRef<str>,
{
    let mut numbers: Vec<i64> = Vec::new();
    for line in lines {
        if let Ok(line_as_number) = line.as_ref().parse() {
            numbers.push(line_as_number);
        }
    }
    numbers
}

pub fn string_to_ints(input: &str) -> Vec<i64> {
    strings_to_ints(input.lines().map(|line| line.trim()))
}

pub fn read_to_ints(filename: &str) -> Vec<i64> {
    strings_to_ints(read_input(filename))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_input_strings() {
        assert_eq!(
            vec!["1", "12", "", "123"],
            read_input("./src/common/read_input_test.txt")
        );
    }

    #[test]
    fn read_input_to_ints() {
        assert_eq!(
            vec![1, 12, 123],
            read_to_ints("./src/common/read_input_test.txt")
        );
    }

    #[test]
    fn test_string_to_ints() {
        assert_eq!(
            vec![1, 12, 123],
            string_to_ints(
                "
            1
            12
            123
            "
            )
        );
    }
}
