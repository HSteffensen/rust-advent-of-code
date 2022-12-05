use common::{solution::AocSolution, SimpleResult};

mod common;
mod y2018;
mod y2022;

fn main() -> SimpleResult<()> {
    y2018::d01::Part1::solve()
}
