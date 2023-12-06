use crate::types::*;

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = (&'a str, usize, Vec<Number>);

    fn parse_input(input: &'a str) -> Self::Input {
        let line_length = input.find('\n').unwrap() + 1;
        (input, line_length, find_numbers(input, line_length))
    }

    fn solve1(&self, inp: &Self::Input, test: bool) -> String {
        let (input, line_length, numbers) = inp;
        test_print!(test, "line_length: {line_length}");
        let mut sum = 0;
        for num in numbers {
            if touches_symbol(input, *line_length, num.x, num.y, num.len) {
                sum += num.val;
            }
        }
        sum.to_string()
    }

    fn solve2(&self, inp: &Self::Input, test: bool) -> String {
        let (input, line_length, numbers) = inp;
        test_print!(test, "line_length: {line_length}");
        let mut sum = 0;
        test_print!(test, "numbers: {numbers:?}");
        for (i, c) in input.bytes().enumerate() {
            if c == b'*' {
                let x = i % line_length;
                let y = i / line_length;
                let numbers = touching_numbers(numbers, x as isize, y as isize);
                test_print!(test, "x: {x} y: {y} touching: {numbers:?}");
                if numbers.len() == 2 {
                    sum += numbers[0] * numbers[1];
                }
            }
        }
        sum.to_string()
    }
}

fn touching_numbers(numbers: &[Number], x: isize, y: isize) -> Vec<u32> {
    let mut touching = vec![];
    for num in numbers {
        // to far up
        if y - num.y > 1 {
            continue;
        }
        // to far down
        if num.y - y > 1 {
            // if to far down no more matches can happen
            return touching;
        }
        // to far right
        if num.x - num.len as isize > x {
            continue;
        }
        // to far left
        if num.x < x - 1 {
            continue;
        }
        touching.push(num.val)
    }
    touching
}

#[derive(Debug)]
pub struct Number {
    x: isize,
    y: isize,
    len: u8,
    val: u32,
}

fn find_numbers(input: &str, line_length: usize) -> Vec<Number> {
    let mut numbers = vec![];
    let mut num_length = 0;
    let mut number = 0;
    for (i, c) in input.bytes().enumerate() {
        if c.is_ascii_digit() {
            num_length += 1;
            number = number * 10 + (c - b'0') as u32;
        } else {
            if num_length == 0 {
                continue;
            }
            numbers.push(Number {
                x: ((i - 1) % line_length) as isize,
                y: ((i - 1) / line_length) as isize,
                len: num_length,
                val: number,
            });
            num_length = 0;
            number = 0;
        }
    }
    numbers
}

/// check if number ending at i touches a symbol
fn touches_symbol(
    input: &str,
    line_length: usize,
    num_x: isize,
    num_y: isize,
    num_length: u8,
) -> bool {
    // check left and right
    if is_symbol(input, line_length, num_x + 1, num_y)
        || is_symbol(input, line_length, num_x - num_length as isize, num_y)
    {
        return true;
    }
    for x in (num_x - num_length as isize)..=(num_x + 1) {
        // check row above
        if is_symbol(input, line_length, x, num_y - 1) {
            return true;
        }
        // check row below
        if is_symbol(input, line_length, x, num_y + 1) {
            return true;
        }
    }
    false
}

fn is_symbol(input: &str, line_length: usize, x: isize, y: isize) -> bool {
    // check left or right outside
    if x < 0 || x as usize > line_length - 2 {
        return false;
    }
    // check top or bottom outside
    if y < 0 || y as usize > line_length - 2 {
        return false;
    }
    let char = input.as_bytes()[line_length * y as usize + x as usize];
    !char.is_ascii_digit() && char != b'.'
}
