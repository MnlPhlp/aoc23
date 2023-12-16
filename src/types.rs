#[derive(Clone, Copy)]
pub enum Task {
    One,
    Two,
    Both,
}

pub trait DaySolver<'a> {
    type Input;

    fn parse_input(input: &'a str, test: bool) -> Self::Input;

    fn solve(
        &self,
        day: usize,
        input: &'a str,
        test: bool,
        task: Task,
        print_times: bool,
    ) -> (String, String) {
        let mut res1 = "".into();
        let mut res2 = "".into();
        let start = Instant::now();
        let input = Self::parse_input(input, test);
        let parsing = start.elapsed();
        let start = Instant::now();
        if !matches!(task, Task::Two) {
            res1 = self.solve1(&input, test);
        }
        let t1 = start.elapsed();
        let start = Instant::now();
        if !matches!(task, Task::One) {
            res2 = self.solve2(&input, test);
        }
        let t2 = start.elapsed();
        if print_times {
            println!(
                "day: {day:>2} parsing: {parsing:>8.2?}, task 1: {t1:>8.2?}, task 2: {t2:>8.2?}"
            );
        }
        (res1, res2)
    }

    fn solve1(&self, input: &Self::Input, test: bool) -> String;
    fn solve2(&self, input: &Self::Input, test: bool) -> String;
}

macro_rules! test_print {
    ($test:expr,$($arg:tt)*) => {
        if $test {
            println!($($arg)*)
        }
    };
}
use std::time::Instant;

pub(crate) use test_print;
