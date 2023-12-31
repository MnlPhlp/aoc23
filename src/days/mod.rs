use crate::types::DaySolver;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;

pub const DAY_COUNT: usize = 19;

pub(crate) fn solve(
    day: usize,
    input: &str,
    test: bool,
    task: crate::types::Task,
    print_times: bool,
) -> (String, String) {
    match day {
        1 => day01::Solver.solve(day, input, test, task, print_times),
        3 => day03::Solver.solve(day, input, test, task, print_times),
        4 => day04::Solver.solve(day, input, test, task, print_times),
        2 => day02::Solver.solve(day, input, test, task, print_times),
        5 => day05::Solver.solve(day, input, test, task, print_times),
        6 => day06::Solver.solve(day, input, test, task, print_times),
        7 => day07::Solver.solve(day, input, test, task, print_times),
        8 => day08::Solver.solve(day, input, test, task, print_times),
        9 => day09::Solver.solve(day, input, test, task, print_times),
        10 => day10::Solver.solve(day, input, test, task, print_times),
        11 => day11::Solver.solve(day, input, test, task, print_times),
        12 => day12::Solver.solve(day, input, test, task, print_times),
        13 => day13::Solver.solve(day, input, test, task, print_times),
        14 => day14::Solver.solve(day, input, test, task, print_times),
        15 => day15::Solver.solve(day, input, test, task, print_times),
        16 => day16::Solver.solve(day, input, test, task, print_times),
        17 => day17::Solver.solve(day, input, test, task, print_times),
        18 => day18::Solver.solve(day, input, test, task, print_times),
        19 => day19::Solver.solve(day, input, test, task, print_times),
        _ => panic!("invalid day"),
    }
}
