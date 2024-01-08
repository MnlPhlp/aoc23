use std::{collections::HashMap, fmt::Display};

use nom::{
    character::complete::{newline, one_of},
    multi::{many1, separated_list1},
    IResult,
};

use crate::types::*;

pub struct Solver;

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PipeSegment {
    Vertical,
    Horizontal,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
    Ground,
    Start,
}

impl PipeSegment {
    /// Get the direction of the pipe segment, given the incoming direction.
    /// Returns None if the pipe segment is not connected to the incoming direction.
    fn get_direction(&self, incoming: Direction) -> Option<Direction> {
        match self {
            Self::Vertical if incoming.x == 0 => Some(Direction::new(0, incoming.y)),
            Self::Horizontal if incoming.y == 0 => Some(Direction::new(incoming.x, 0)),
            Self::NorthWest if incoming.x == 1 => Some(Direction::new(0, -1)),
            Self::NorthWest if incoming.y == 1 => Some(Direction::new(-1, 0)),
            Self::NorthEast if incoming.x == -1 => Some(Direction::new(0, -1)),
            Self::NorthEast if incoming.y == 1 => Some(Direction::new(1, 0)),
            Self::SouthWest if incoming.x == 1 => Some(Direction::new(0, 1)),
            Self::SouthWest if incoming.y == -1 => Some(Direction::new(-1, 0)),
            Self::SouthEast if incoming.x == -1 => Some(Direction::new(0, 1)),
            Self::SouthEast if incoming.y == -1 => Some(Direction::new(1, 0)),
            _ => None,
        }
    }
}

impl From<char> for PipeSegment {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::Vertical,
            '-' => Self::Horizontal,
            'L' => Self::NorthEast,
            'J' => Self::NorthWest,
            '7' => Self::SouthWest,
            'F' => Self::SouthEast,
            '.' => Self::Ground,
            'S' => Self::Start,
            c => panic!("invalid pipe character: '{c}'"),
        }
    }
}

impl Display for PipeSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Vertical => '|',
            Self::Horizontal => '-',
            Self::NorthWest => 'J',
            Self::NorthEast => 'L',
            Self::SouthWest => '7',
            Self::SouthEast => 'F',
            Self::Ground => '.',
            Self::Start => 'S',
        };
        write!(f, "{}", c)
    }
}

impl<'a> DaySolver<'a> for Solver {
    type Input = HashMap<Position, PipeSegment>;

    fn parse_input(input: &'a str, test: bool) -> Self::Input {
        let (_, segments) = nom_parse(input).unwrap();

        find_loop(test, segments)
    }

    fn solve1(&self, path: &Self::Input, _test: bool) -> String {
        (path.len() / 2).to_string()
    }

    fn solve2(&self, path: &Self::Input, test: bool) -> String {
        if test {
            print!("Path: [");
            for pos in path.keys() {
                print!("{}, ", pos);
            }
            println!("]");
        }
        // find outermost positions
        let mut min_x = i32::MAX;
        let mut min_y = i32::MAX;
        let mut max_x = i32::MIN;
        let mut max_y = i32::MIN;
        for pos in path.keys() {
            if pos.x < min_x {
                min_x = pos.x;
            }
            if pos.y < min_y {
                min_y = pos.y;
            }
            if pos.x > max_x {
                max_x = pos.x;
            }
            if pos.y > max_y {
                max_y = pos.y;
            }
        }
        let mut enclosed = 0;
        for y in min_y..=max_y {
            let mut inside = false;
            for x in min_x..=max_x {
                if let Some(&segment) = path.get(&Position::new(x, y)) {
                    if segment == PipeSegment::Vertical
                        || segment == PipeSegment::NorthEast
                        || segment == PipeSegment::NorthWest
                    {
                        inside = !inside;
                    }
                } else if inside {
                    enclosed += 1;
                }
            }
        }
        // check if enclosed by walls for each position
        enclosed.to_string()
    }
}

fn find_loop(test: bool, input: Vec<Vec<PipeSegment>>) -> HashMap<Position, PipeSegment> {
    test_print!(test, "{input:?}]");
    // find starting position
    let mut pos1 = Position::new(0, 0);
    let mut pos2 = Position::new(0, 0);
    for (y, row) in input.iter().enumerate() {
        for (x, segment) in row.iter().enumerate() {
            if *segment == PipeSegment::Start {
                pos1 = Position::new(x as i32, y as i32);
                pos2 = Position::new(x as i32, y as i32);
            }
        }
    }
    let mut path = HashMap::new();
    // check where tiles are connected to the starting position
    // and move there to start
    let mut dir1 = Direction::new(0, 0);
    let mut dir2 = Direction::new(0, 0);
    let start_x = pos1.x;
    let start_y = pos1.y;
    let mut start_dir = (0, 0, 0, 0);
    for &(dir_x, dir_y) in &[(0, -1), (0, 1), (1, 0), (-1, 0)] {
        let x = start_x + dir_x;
        let y = start_y + dir_y;
        if x < 0 || y < 0 || x >= input[0].len() as i32 || y >= input.len() as i32 {
            continue;
        }
        let segment = input[y as usize][x as usize];
        if let Some(dir) = segment.get_direction(Direction::new(dir_x, dir_y)) {
            if dir1 == (0, 0) {
                pos1 = Position::new(x, y);
                dir1 = dir;
                start_dir.0 = dir_x;
                start_dir.1 = dir_y;
                path.insert(pos1, segment);
            } else {
                pos2 = Position::new(x, y);
                dir2 = dir;
                start_dir.2 = dir_x;
                start_dir.3 = dir_y;
                path.insert(pos2, segment);
                break;
            }
        }
    }
    test_print!(
        test,
        "Starting:\n pos1: {pos1:?}, dir1: {dir1:?}\n pos2: {pos2:?}, dir1: {dir2:?}\n"
    );
    let start_segment = match start_dir {
        (0, -1, 0, 1) => PipeSegment::Vertical,
        (0, -1, -1, 0) => PipeSegment::NorthWest,
        (0, -1, 1, 0) => PipeSegment::NorthEast,
        (0, 1, 1, 0) => PipeSegment::SouthEast,
        (0, 1, -1, 0) => PipeSegment::SouthWest,
        (1, 0, -1, 0) => PipeSegment::Horizontal,
        _ => panic!("invalid starting direction"),
    };
    // add Starting segment
    path.insert(Position::new(start_x, start_y), start_segment);
    // find loop
    while pos1 != pos2 {
        test_print!(test, "pos1: {pos1} {dir1}; pos2: {pos2:?} {dir2}");
        pos1 += dir1;
        let segment = input[pos1.y as usize][pos1.x as usize];
        dir1 = segment.get_direction(dir1).unwrap();
        path.insert(pos1, segment);
        pos2 += dir2;
        let segment = input[pos2.y as usize][pos2.x as usize];
        dir2 = input[pos2.y as usize][pos2.x as usize]
            .get_direction(dir2)
            .unwrap();
        path.insert(pos2, segment);
    }
    path
}

fn nom_parse(input: &str) -> IResult<&str, Vec<Vec<PipeSegment>>> {
    separated_list1(newline, many1(pipe_segment))(input)
}

fn pipe_segment(input: &str) -> IResult<&str, PipeSegment> {
    let (rest, segment) = one_of("|-LJ7F.S")(input)?;
    let segment = PipeSegment::from(segment);
    Ok((rest, segment))
}
