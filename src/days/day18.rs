use std::{
    cmp::{max, min},
    collections::HashSet,
};

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

    fn solve1(&self, plan: &Self::Input, test: bool) -> String {
        let mut position = Position::new(0, 0);
        let mut visited = HashSet::new();
        visited.insert(position);
        let mut min_x = 0;
        let mut min_y = 0;
        let mut max_x = 0;
        let mut max_y = 0;
        for step in plan {
            for _ in 0..step.distance_1 {
                position += step.direction_1;
                visited.insert(position);
            }
            min_x = min(min_x, position.x);
            min_y = min(min_y, position.y);
            max_x = max(max_x, position.x);
            max_y = max(max_y, position.y);
        }
        print_grid(test, min_y, max_y, min_x, max_x, &visited);
        visited = fill(visited, min_x, max_x, min_y, max_y);
        test_print!(test, "Filled:");
        print_grid(test, min_y, max_y, min_x, max_x, &visited);
        visited.len().to_string()
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        todo!()
    }
}

fn print_grid(
    test: bool,
    min_y: i32,
    max_y: i32,
    min_x: i32,
    max_x: i32,
    visited: &HashSet<Position>,
) {
    if test {
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if visited.contains(&Position::new(x, y)) {
                    print!("#")
                } else {
                    print!(".")
                }
            }
            println!()
        }
    }
}

/// Fill space inside border
fn fill(
    mut visited: HashSet<Position>,
    min_x: i32,
    max_x: i32,
    min_y: i32,
    max_y: i32,
) -> HashSet<Position> {
    // find some starting point inside border
    let mut start = None;
    'y_loop: for y in min_y..=max_y {
        for x in min_x..=max_x {
            let pos = Position::new(x, y);
            if visited.contains(&pos) {
                continue;
            } else if visited.contains(&(pos + Direction::new(-1, 0)))
                && visited.contains(&(pos + Direction::new(0, -1)))
            {
                start = Some(pos);
                break 'y_loop;
            }
        }
    }
    let start = start.expect("start has to be found");
    // fill from starting point
    let mut queue = vec![start];
    while let Some(current) = queue.pop() {
        for pos in current.neighbors() {
            if !visited.contains(&pos) {
                queue.push(pos);
                visited.insert(pos);
            }
        }
    }
    visited
}

fn nom_parse(input: &str) -> IResult<&str, Vec<Step>> {
    separated_list1(newline, step)(input)
}

fn step(input: &str) -> IResult<&str, Step> {
    let (remaining, (dir, _, distance, _, color)) = tuple((
        one_of("URDL"),
        space1,
        complete::u8,
        space1,
        delimited(tag("(#"), alphanumeric1, complete::char(')')),
    ))(input)?;
    let direction = match dir {
        'U' => Direction::new(0, -1),
        'R' => Direction::new(1, 0),
        'D' => Direction::new(0, 1),
        'L' => Direction::new(-1, 0),
        _ => unreachable!(),
    };
    Ok((
        remaining,
        Step {
            direction_1: direction,
            distance_1: distance,
            color: color.into(),
        },
    ))
}

#[derive(Debug)]
pub struct Step {
    direction_1: Direction,
    distance_1: u8,
    color: String,
}
