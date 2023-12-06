use crate::types::*;

pub struct Solver;

impl DaySolver for Solver {
    fn solve1(&self, input: &str, _test: bool) -> String {
        let sum = input.lines().fold(0u32, |acc, line| {
            if line.is_empty() {
                return acc;
            }
            let start = line.chars().find(|c| c.is_ascii_digit()).unwrap() as u8 - b'0';
            let end = line.chars().rfind(|c| c.is_ascii_digit()).unwrap() as u8 - b'0';
            let num = start * 10 + end;
            acc + num as u32
        });
        sum.to_string()
    }

    fn solve2(&self, input: &str, test: bool) -> String {
        let sum = input.lines().fold(0u32, |acc, line| {
            if line.is_empty() {
                return acc;
            }
            let start = find_first_number(line);
            let end = find_last_number(line);
            test_print!(test, "line: {line} start: {start} end: {end}");
            let num = start * 10 + end;
            acc + num
        });
        sum.to_string()
    }
}

fn find_first_number(mut line: &str) -> u32 {
    let mut digit = 0u32;
    let digit_pos = line.chars().position(|c| c.is_ascii_digit());
    if let Some(pos) = digit_pos {
        digit = (line.as_bytes()[pos] - b'0') as u32;
        if pos == 0 {
            return digit;
        }
        line = &line[..=pos];
    }
    for num in [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ] {
        if let Some(pos) = line.find(num.0) {
            line = &line[..=pos];
            digit = num.1
        }
    }
    digit
}

fn find_last_number(mut line: &str) -> u32 {
    let mut digit = 0u32;
    let digit_pos = line
        .chars()
        .rev()
        .position(|c| c.is_ascii_digit())
        .map(|i| line.len() - i - 1);
    if let Some(pos) = digit_pos {
        digit = (line.as_bytes()[pos] - b'0') as u32;
        if pos == line.len() {
            return digit;
        }
        line = &line[pos..];
    }
    for num in [
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ] {
        if let Some(pos) = line.rfind(num.0) {
            line = &line[pos..];
            digit = num.1
        }
    }
    digit
}
