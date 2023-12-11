use nom::{
    bytes::complete::{is_not, tag, take_until, take_while1},
    character::complete::{self, newline},
    multi::separated_list1,
    sequence::{delimited, terminated, tuple},
    IResult,
};

use crate::types::*;

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = &'a str;

    fn parse_input(input: &'a str, _test: bool) -> Self::Input {
        input
    }

    fn solve1(&self, input: &Self::Input, test: bool) -> String {
        let (_, races) = nom_parse_1(input).unwrap();
        test_print!(test, "races: {races:?}");
        let mut res = 1;
        for (time, max_dist) in races {
            let min = (1..time).find(|t| t * (time - t) > max_dist).unwrap();
            let max = (min..time).find(|t| t * (time - t) <= max_dist).unwrap();
            test_print!(test, "min: {min}, max: {max}");
            res *= max - min;
        }
        res.to_string()
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        let (_, (time, max_dist)) = nom_parse_2(input).unwrap();
        let min = binary_search(1, time, |t| t * (time - t) > max_dist);
        let max = binary_search_right(time, time, |t| t * (time - t) >= max_dist);
        test_print!(test, "min: {min}, max: {max}");
        (max - min + 1).to_string()
    }
}

fn binary_search(mut min: u64, mut max: u64, valid: impl Fn(u64) -> bool) -> u64 {
    while min < max {
        let mid = (max + min) / 2;
        if valid(mid) {
            max = mid;
        } else {
            min = mid + 1;
        }
    }
    min
}

fn binary_search_right(mut min: u64, mut max: u64, valid: impl Fn(u64) -> bool) -> u64 {
    let mut mid = (max + min) / 2;
    while min < max {
        mid = (max + min) / 2;
        if valid(mid) {
            min = mid + 1;
        } else {
            max = mid - 1;
        }
    }
    mid
}

fn nom_parse_2(input: &str) -> IResult<&str, (u64, u64)> {
    let (_, (time, distance)) = tuple((num("Time:"), num("Distance:")))(input)?;
    Ok(("", (time, distance)))
}

fn num(key: &'static str) -> impl Fn(&str) -> IResult<&str, u64> {
    move |input| {
        let (rest, numbers) = delimited(tag(key), is_not("\n"), complete::newline)(input)?;
        let num = numbers.replace(' ', "").parse().unwrap();
        Ok((rest, num))
    }
}

fn nom_parse_1(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (_, (times, _, distances)) =
        tuple((num_list("Time:"), newline, num_list("Distance:")))(input)?;
    Ok(("", times.into_iter().zip(distances).collect()))
}

fn num_list(key: &'static str) -> impl Fn(&str) -> IResult<&str, Vec<u32>> {
    move |input| {
        let (input, _) = tuple((tag(key), complete::space0))(input)?;
        let (rest, seeds) = separated_list1(complete::space1, complete::u32)(input)?;
        Ok((rest, seeds))
    }
}
