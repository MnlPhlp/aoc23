use nom::{
    bytes::complete::{tag, take_until, take_while1},
    character::complete::{self, newline},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, terminated, tuple},
    IResult,
};

use crate::types::*;

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<(u32, u32)>;

    fn parse_input(input: &'a str) -> Self::Input {
        let (rest, races) = nom_parse(input).unwrap();
        dbg!(&races);
        races
    }

    fn solve1(&self, input: &Self::Input, test: bool) -> String {
        todo!()
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        todo!()
    }
}

fn nom_parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (rest, (times, _, distances)) =
        tuple((num_list("Time:"), newline, num_list("Distance:")))(input)?;
    Ok(("", times.into_iter().zip(distances).collect()))
}

fn num_list(key: &'static str) -> impl Fn(&str) -> IResult<&str, Vec<u32>> {
    move |input| {
        let (input, _) = tuple((tag(key), complete::space0))(input)?;
        println!("input: {input}");
        let (rest, seeds) = delimited(
            complete::space0,
            separated_list0(complete::space0, complete::u32),
            complete::newline,
        )(input)?;
        Ok((rest, seeds))
    }
}
