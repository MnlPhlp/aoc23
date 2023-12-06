use crate::types::*;

pub struct Solver;

impl DaySolver for Solver {
    fn solve1(&self, input: &str, test: bool) -> String {
        let mut sum = 0;
        let mut game_id = 1;
        for line in input.lines() {
            if game_possible(line, 12, 13, 14, test) {
                sum += game_id;
            }
            game_id += 1;
        }
        sum.to_string()
    }

    fn solve2(&self, input: &str, _test: bool) -> String {
        let mut sum = 0;
        for line in input.lines() {
            let (a, b, c) = max_counts(line);
            sum += a * b * c;
        }
        sum.to_string()
    }
}

fn game_possible(line: &str, red: u32, green: u32, blue: u32, test: bool) -> bool {
    let (max_red, max_green, max_blue) = max_counts(line);
    test_print!(
        test,
        "{line}\n    max_red: {max_red}, max_green: {max_green}, max_blue: {max_blue}"
    );
    red >= max_red && green >= max_green && blue >= max_blue
}

fn max_counts(line: &str) -> (u32, u32, u32) {
    let draws = line.split(':').last().unwrap().split(';');
    let mut max_red = 0;
    let mut max_green = 0;
    let mut max_blue = 0;
    for draw in draws {
        for cubes in draw.split(',') {
            let (num, color) = cubes.trim().split_once(' ').unwrap();
            let num = num.parse().unwrap();
            match color {
                "red" => {
                    if num > max_red {
                        max_red = num;
                    }
                }
                "green" => {
                    if num > max_green {
                        max_green = num;
                    }
                }
                "blue" => {
                    if num > max_blue {
                        max_blue = num;
                    }
                }
                _ => {}
            }
        }
    }
    (max_red, max_green, max_blue)
}
