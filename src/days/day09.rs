use nom::{
    character::complete::{self, newline, space1},
    multi::separated_list1,
    IResult,
};

use crate::types::*;

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<Vec<i32>>;

    fn parse_input(input: &'a str, test: bool) -> Self::Input {
        let (_, list) = nom_parse(input).unwrap();
        test_print!(test, "list: {list:?}");
        list
    }

    fn solve1(&self, input: &Self::Input, test: bool) -> String {
        let mut sum = 0;
        for row in input {
            let mut diff_sum = 0;
            let mut diffs = row.to_owned();
            loop {
                diffs = find_diffs(&diffs);
                if diffs.iter().all(|&x| x == 0) {
                    break;
                }
                diff_sum += diffs[diffs.len() - 1];
            }
            let prediction = row[row.len() - 1] + diff_sum;
            test_print!(test, "prediction: {prediction}");
            sum += prediction;
        }
        sum.to_string()
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        let mut sum = 0;
        for row in input {
            let mut diffs_stack = vec![row.to_owned()];
            loop {
                let diffs = find_diffs(&diffs_stack[diffs_stack.len() - 1]);
                if diffs.iter().all(|&x| x == 0) {
                    break;
                }
                diffs_stack.push(diffs);
            }
            let mut prediction = 0;
            for diff in diffs_stack.iter().rev() {
                prediction = diff[0] - prediction;
            }
            test_print!(test, "prediction: {prediction}");
            sum += prediction;
        }
        sum.to_string()
    }
}

fn find_diffs(row: &[i32]) -> Vec<i32> {
    let mut last = row[0];
    let mut diffs = Vec::with_capacity(row.len() - 1);
    for i in 1..row.len() {
        diffs.push(row[i] - last);
        last = row[i];
    }
    diffs
}

fn nom_parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    separated_list1(newline, separated_list1(space1, complete::i32))(input)
}
