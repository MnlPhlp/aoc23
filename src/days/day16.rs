use std::collections::HashMap;

use rayon::prelude::*;

use crate::types::*;

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<Vec<u8>>;

    fn parse_input(input: &'a str, _test: bool) -> Self::Input {
        input.lines().map(|l| l.as_bytes().to_vec()).collect()
    }

    fn solve1(&self, input: &Self::Input, _test: bool) -> String {
        count_energized_tiles(Position::new(0, 0), Direction::new(1, 0), input).to_string()
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        let max = (0..input.len())
            .collect::<Vec<_>>()
            .par_iter()
            .map(|&y| {
                let mut max = 0;
                for x in 0..input[0].len() {
                    if y == 0 {
                        //  check going down
                        let count = count_energized_tiles(
                            Position::new(x as i32, y as i32),
                            Direction::new(0, 1),
                            input,
                        );
                        if count > max {
                            max = count;
                            test_print!(test, "best_starting pos: ({x},{y})");
                        }
                    }
                    if x == 0 {
                        // check going right
                        let count = count_energized_tiles(
                            Position::new(x as i32, y as i32),
                            Direction::new(1, 0),
                            input,
                        );
                        if count > max {
                            max = count;
                            test_print!(test, "best_starting pos: ({x},{y})");
                        }
                    }
                    if x == input[0].len() - 1 {
                        // check going left
                        let count = count_energized_tiles(
                            Position::new(x as i32, y as i32),
                            Direction::new(-1, 0),
                            input,
                        );
                        if count > max {
                            max = count;
                            test_print!(test, "best_starting pos: ({x},{y})");
                        }
                    }
                    if y == input.len() - 1 {
                        // check going up
                        let count = count_energized_tiles(
                            Position::new(x as i32, y as i32),
                            Direction::new(0, -1),
                            input,
                        );
                        if count > max {
                            max = count;
                            test_print!(test, "best_starting pos: ({x},{y})");
                        }
                    }
                }
                max
            })
            .max()
            .unwrap();
        max.to_string()
    }
}

fn count_energized_tiles(start_pos: Position, start_dir: Direction, input: &[Vec<u8>]) -> usize {
    let mut positions = vec![start_pos];
    let mut directions = vec![start_dir];
    let mut energized = HashMap::new();
    while let Some(mut pos) = positions.pop() {
        // get last beam in queue
        let mut dir = directions.pop().unwrap();
        // follow bean until reaching existing beam or leaving the grid
        loop {
            // stop if out of grid
            if pos.x >= input[0].len() as i64 || pos.y >= input.len() as i64 {
                break;
            }
            // stop if beam already energized
            if let Some(energized_dir) = energized.get(&pos) {
                if energized_dir == &dir {
                    break;
                }
            }
            energized.insert(pos, dir);
            // split or redirect beam
            dir = match input[pos.y as usize][pos.x as usize] {
                b'/' => Direction::new(-dir.y, -dir.x),
                b'\\' => Direction::new(dir.y, dir.x),
                b'|' if dir.x != 0 => {
                    // add new beam going down
                    positions.push(pos + Direction::new(0, 1));
                    directions.push(Direction::new(0, 1));
                    // continue with beam going up
                    Direction::new(0, -1)
                }
                b'-' if dir.y != 0 => {
                    // add new beam going right
                    positions.push(pos + Direction::new(1, 0));
                    directions.push(Direction::new(1, 0));
                    // continue with beam going left
                    Direction::new(-1, 0)
                }
                _ => dir,
            };
            // stop if at edge and heading out
            if pos.x == 0 && dir.x == -1 || pos.y == 0 && dir.y == -1 {
                break;
            }
            // move beam
            pos += dir;
        }
    }
    energized.len()
}
