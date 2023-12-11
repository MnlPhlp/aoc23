use crate::types::DaySolver;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;

pub const DAY_COUNT: usize = 8;

pub(crate) fn solve(
    day: usize,
    input: &str,
    test: bool,
    task: crate::types::Task,
) -> (String, String) {
    match day {
        1 => day01::Solver.solve(day, input, test, task),
        2 => day02::Solver.solve(day, input, test, task),
        3 => day03::Solver.solve(day, input, test, task),
        4 => day04::Solver.solve(day, input, test, task),
        5 => day05::Solver.solve(day, input, test, task),
        6 => day06::Solver.solve(day, input, test, task),
        7 => day07::Solver.solve(day, input, test, task),
        8 => day08::Solver.solve(day, input, test, task),
        _ => panic!("invalid day"),
    }
}
