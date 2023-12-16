mod days;
mod types;
mod util;

use std::time::{Duration, Instant};

pub use days::DAY_COUNT;
use rayon::prelude::*;
pub use types::Task;

pub fn calc_day(
    day: usize,
    result1: &mut String,
    result2: &mut String,
    time: &mut Duration,
    test: bool,
    task: Task,
    print_times: bool,
) {
    if test {
        println!("\n##################\ncalculating day {day} \n##################\n");
    }
    let input = util::read_input(day, test);
    let start = Instant::now();
    let (res1, res2) = days::solve(day, &input, test, task, print_times);
    *time = Instant::now().duration_since(start);
    *result1 = res1;
    *result2 = res2;
}

pub fn run_serial(
    days: &[usize],
    results1: &mut [String],
    results2: &mut [String],
    times: &mut [Duration],
    test: bool,
    task: Task,
) {
    for (i, day) in days.iter().enumerate() {
        calc_day(
            *day,
            &mut results1[i],
            &mut results2[i],
            &mut times[i],
            test,
            task,
            true,
        );
    }
}

pub fn run_parallel(
    days: &[usize],
    results1: &mut Vec<String>,
    results2: &mut Vec<String>,
    times: &mut Vec<Duration>,
    test: bool,
    task: Task,
) {
    days.par_iter()
        .zip(results1)
        .zip(results2)
        .zip(times)
        .for_each(|(((day, result1), result2), time)| {
            calc_day(*day, result1, result2, time, test, task, true);
        });
}
