use crate::types::DaySolver;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;

pub const DAY_COUNT: usize = 3;

pub(crate) fn solve(
    day: usize,
    input: &str,
    test: bool,
    task: crate::types::Task,
) -> (String, String) {
    match day {
        1 => day01::Solver.solve(input, test, task),
        2 => day02::Solver.solve(input, test, task),
        3 => day03::Solver.solve(input, test, task),
        4 => day04::Solver.solve(input, test, task),
        _ => panic!("invalid day"),
    }
}
