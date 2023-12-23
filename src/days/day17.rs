use crate::types::*;

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<Vec<u8>>;

    fn parse_input(input: &'a str, _test: bool) -> Self::Input {
        input.lines().map(|l| l.as_bytes().to_vec()).collect()
    }

    fn solve1(&self, input: &Self::Input, test: bool) -> String {
        String::from("ToDo")
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        String::from("ToDo")
    }
}
