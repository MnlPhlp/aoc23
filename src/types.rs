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
use std::{fmt::Display, time::Instant};

pub(crate) use test_print;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Direction {
    pub x: i32,
    pub y: i32,
}

impl Direction {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

impl PartialEq<(i32, i32)> for Direction {
    fn eq(&self, other: &(i32, i32)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match (self.x, self.y) {
            (0, -1) => '^',
            (-1, 0) => '<',
            (1, 0) => '>',
            (0, 1) => 'v',
            _ => '?',
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, PartialEq, Copy, Clone, Eq, Hash)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        Self {
            x: (self.x as i32 + rhs.x) as usize,
            y: (self.y as i32 + rhs.y) as usize,
        }
    }
}

impl std::ops::AddAssign<Direction> for Position {
    fn add_assign(&mut self, rhs: Direction) {
        *self = *self + rhs;
    }
}

impl Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}
