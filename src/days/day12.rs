// solution partly taken from https://github.com/ageron/aoc2023-rust/blob/main/src/day12.rs

use nom::{
    bytes::complete::is_not,
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::types::*;

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<Row>;

    fn parse_input(input: &'a str, test: bool) -> Self::Input {
        let (_, rows) = nom_parse(input).unwrap();
        test_print!(test, "rows: {:#?}", rows);
        rows
    }

    fn solve1(&self, rows: &Self::Input, _test: bool) -> String {
        rows.iter()
            .map(possible_solutions)
            .sum::<usize>()
            .to_string()
    }

    fn solve2(&self, rows: &Self::Input, test: bool) -> String {
        let rows = unfold(rows);
        self.solve1(&rows, test)
    }
}

#[derive(Debug, Clone)]
pub struct Row {
    line: String,
    groups: Vec<usize>,
}

fn nom_parse(input: &str) -> IResult<&str, Vec<Row>> {
    separated_list1(newline, row)(input)
}

fn row(input: &str) -> IResult<&str, Row> {
    let (rest, (line, _, groups)) = tuple((
        is_not(" "),
        complete::char(' '),
        separated_list1(complete::char(','), complete::u8),
    ))(input)?;
    let groups = groups.into_iter().map(|g| g as usize).collect();
    Ok((
        rest,
        Row {
            line: line.to_string(),
            groups,
        },
    ))
}

fn possible_solutions(row: &Row) -> usize {
    possible_solutions_rec(row.line.as_bytes(), &row.groups)
}

fn possible_solutions_rec(line: &[u8], groups: &[usize]) -> usize {
    if groups.is_empty() {
        if line.contains(&b'#') {
            return 0;
        } else {
            return 1;
        }
    }
    let center_group_index = groups.len() / 2;
    let center_group_length = groups[center_group_index];
    let left_groups = &groups[..center_group_index];
    let right_groups = &groups[center_group_index + 1..];
    if line.len() < (min_size(left_groups) + min_size(right_groups) + center_group_length) {
        return 0;
    }
    let mut num_arrangements = 0;
    for start_index in
        min_size(left_groups)..=line.len() - min_size(right_groups) - center_group_length
    {
        if start_index == 0 || b".?".contains(&line[start_index - 1]) {
            let after_index = start_index + center_group_length;
            if !line[start_index..after_index].contains(&b'.')
                && (after_index == line.len() || b".?".contains(&line[after_index]))
            {
                let left_arrangements =
                    possible_solutions_rec(&line[..start_index.max(1) - 1], left_groups);
                if left_arrangements > 0 {
                    let right_start_index = (after_index + 1).min(line.len());
                    let right_arrangements =
                        possible_solutions_rec(&line[right_start_index..], right_groups);
                    num_arrangements += left_arrangements * right_arrangements;
                }
            }
        }
    }
    num_arrangements
}

fn min_size(groups: &[usize]) -> usize {
    if groups.is_empty() {
        return 0;
    }
    groups.iter().sum::<usize>() + groups.len() - 1
}

fn unfold<'a>(rows: &Vec<Row>) -> Vec<Row> {
    rows.iter().map(|r| unfold_row(r)).collect()
}

fn unfold_row<'a>(r: &Row) -> Row {
    let mut line = String::with_capacity(r.line.len() * 5 + 4);
    for i in 0..5 {
        line += &r.line;
        if i < 4 {
            line += "?";
        }
    }
    let groups = r.groups.repeat(5);
    Row { line, groups }
}
