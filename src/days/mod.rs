use crate::types::DaySolver;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

pub const DAY_COUNT: usize = 5;

pub(crate) fn solve(
    day: usize,
    input: &str,
    test: bool,
    task: crate::types::Task,
) -> (String, String) {
    print!("day {day:>2}: ");
    match day {
        1 => day01::Solver.solve(input, test, task),
        2 => day02::Solver.solve(input, test, task),
        3 => day03::Solver.solve(input, test, task),
        4 => day04::Solver.solve(input, test, task),
        5 => day05::Solver.solve(input, test, task),
        6 => day06::Solver.solve(input, test, task),
        _ => panic!("invalid day"),
    }
}
