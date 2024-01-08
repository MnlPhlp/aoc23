use std::collections::HashSet;

use nom::{
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, newline, one_of, space1},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

use crate::types::*;

pub struct Solver;
impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<Step>;

    fn parse_input(input: &'a str, test: bool) -> Self::Input {
        let (rem, parsed) = nom_parse(input).unwrap();
        test_print!(test, "rem: {rem}");
        test_print!(test, "{parsed:?}");
        parsed
    }

    fn solve1(&self, plan: &Self::Input, _test: bool) -> String {
        let steps = plan
            .iter()
            .map(|step| (step.direction_1, step.distance_1 as u32))
            .collect();
        let area = calculate_area(steps);
        area.to_string()
    }

    fn solve2(&self, plan: &Self::Input, _test: bool) -> String {
        let steps = plan
            .iter()
            .map(|step| (step.direction_2, step.distance_2))
            .collect();
        let area = calculate_area(steps);
        area.to_string()
    }
}

// calculate area of given hole
// fixed some errors by looking at https://github.com/Tom-the-Bomb/aoc-2023/blob/main/aoc-py/solutions/day18.py
fn calculate_area(steps: Vec<(Direction, u32)>) -> i64 {
    let mut position = Position::new(0, 0);
    let mut vertices = vec![];
    vertices.push(position);
    let mut border = 0;
    for (dir, dist) in steps {
        position += dir * dist;
        border += dist;
        vertices.push(position);
    }
    // compute area using shoelace formula for inner polygon area
    let mut area = 0;
    for a in 0..vertices.len() {
        let b = (a + 1) % vertices.len();
        area += vertices[a].x * vertices[b].y - vertices[a].y * vertices[b].x
    }
    area /= 2;
    // adding on the border
    area + border as i64 / 2 + 1
}

fn nom_parse(input: &str) -> IResult<&str, Vec<Step>> {
    separated_list1(newline, step)(input)
}

fn step(input: &str) -> IResult<&str, Step> {
    let (remaining, (dir, _, distance_1, _, hex_code)) = tuple((
        one_of("URDL"),
        space1,
        complete::u8,
        space1,
        delimited(tag("(#"), alphanumeric1, complete::char(')')),
    ))(input)?;
    let direction_1 = match dir {
        'U' => Direction::new(0, -1),
        'R' => Direction::new(1, 0),
        'D' => Direction::new(0, 1),
        'L' => Direction::new(-1, 0),
        _ => unreachable!(),
    };
    let (direction_2, distance_2) = parse_hex_code(hex_code);
    Ok((
        remaining,
        Step {
            direction_1,
            distance_1,
            direction_2,
            distance_2,
        },
    ))
}

fn parse_hex_code(hex_code: &str) -> (Direction, u32) {
    let (dist, dir) = hex_code.split_at(5);
    let direction = match dir {
        "0" => Direction::new(1, 0),
        "1" => Direction::new(0, 1),
        "2" => Direction::new(-1, 0),
        "3" => Direction::new(0, -1),
        _ => unreachable!(),
    };
    let distance = u32::from_str_radix(dist, 16).unwrap();
    (direction, distance)
}

#[derive(Debug)]
pub struct Step {
    direction_1: Direction,
    distance_1: u8,
    direction_2: Direction,
    distance_2: u32,
}
