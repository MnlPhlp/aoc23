use crate::types::*;

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = &'a str;

    fn parse_input(input: &'a str) -> Self::Input {
        input
    }

    fn solve1(&self, input: &Self::Input, test: bool) -> String {
        todo!()
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        todo!()
    }
}
