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
    pub x: i64,
    pub y: i64,
}

impl Direction {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl PartialEq<(i64, i64)> for Direction {
    fn eq(&self, other: &(i64, i64)) -> bool {
        self.x == other.0 && self.y == other.1
    }
}

impl<T: Into<i64> + Copy> std::ops::Mul<T> for Direction {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs.into(),
            y: self.y * rhs.into(),
        }
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
    pub x: i64,
    pub y: i64,
}

impl Position {
    pub fn new<T: Into<i64>>(x: T, y: T) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

    pub fn distance(&self, other: &Self) -> usize {
        (self.x - other.x).unsigned_abs() as usize + (self.y - other.y).unsigned_abs() as usize
    }

    pub(crate) fn neighbors(&self) -> [Position; 4] {
        [
            Position::new(self.x, self.y - 1),
            Position::new(self.x + 1, self.y),
            Position::new(self.x, self.y + 1),
            Position::new(self.x - 1, self.y),
        ]
    }
}

impl std::ops::Add<Direction> for Position {
    type Output = Self;

    fn add(self, rhs: Direction) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
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
