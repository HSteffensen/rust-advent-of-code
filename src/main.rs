use crate::common::data::{get_examples, get_input};

mod common;
mod day01;
mod day02;

fn main() {
    println!("Hello, world!");
    let input = get_input(2018, 1);
    println!("{}", input);
    let examples = get_examples(2018, 1, 1);
    println!("{:?}", examples);
}
