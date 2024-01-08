use std::collections::HashMap;

use nom::{
    bytes::complete::{is_not, tag},
    character::complete::{self, alphanumeric1, newline},
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

use crate::types::*;

pub struct Solver;
impl<'a> DaySolver<'a> for Solver {
    type Input = (HashMap<String, Workflow>, Vec<Part>);

    fn parse_input(input: &'a str, test: bool) -> Self::Input {
        let (input_workflows, input_parts) = input.split_once("\n\n").unwrap();
        let (_, workflows) = parse_workflows(input_workflows).unwrap();
        test_print!(test, "workflows: {workflows:?}");
        let (_, parts) = parse_parts(input_parts).unwrap();
        test_print!(test, "parts: {parts:?}");
        let mut workflow_map = HashMap::new();
        for (k, v) in workflows {
            workflow_map.insert(String::from(k), v);
        }
        (workflow_map, parts)
    }

    fn solve1(&self, input: &Self::Input, test: bool) -> String {
        let (workflows, parts) = input;
        let mut accepted = vec![];
        for part in parts {
            let mut w = String::from("in");
            while w != "R" {
                w = workflows[&w].process(part);
                if w == "A" {
                    accepted.push(part);
                    break;
                }
            }
        }
        let sum = accepted.iter().map(|p| p.x + p.m + p.a + p.s).sum::<u32>();
        sum.to_string()
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        todo!()
    }
}

fn parse_parts(input: &str) -> IResult<&str, Vec<Part>> {
    separated_list1(newline, part)(input)
}
fn part(input: &str) -> IResult<&str, Part> {
    let (rem, (_, x, _, m, _, a, _, s, _)) = tuple((
        tag("{x="),
        complete::u32,
        tag(",m="),
        complete::u32,
        tag(",a="),
        complete::u32,
        tag(",s="),
        complete::u32,
        tag("}"),
    ))(input)?;
    Ok((rem, Part { x, m, a, s }))
}

fn parse_workflows(input: &str) -> IResult<&str, Vec<(&str, Workflow)>> {
    separated_list1(
        newline,
        tuple((alphanumeric1, delimited(tag("{"), workflow, tag("}")))),
    )(input)
}
fn workflow(input: &str) -> IResult<&str, Workflow> {
    let (rest, wf) = is_not("}")(input)?;
    let mut default = String::from("");
    let mut checks = vec![];
    for part in wf.split(',') {
        if let Some((cond, dest)) = part.split_once(':') {
            // condition case
            if let Some((field, val)) = cond.split_once('<') {
                checks.push(Check {
                    cond: Condition::Lower,
                    field: Field::from(field),
                    val: val.parse().unwrap(),
                    dest: String::from(dest),
                })
            }
            if let Some((field, val)) = cond.split_once('>') {
                checks.push(Check {
                    cond: Condition::Higher,
                    field: Field::from(field),
                    val: val.parse().unwrap(),
                    dest: String::from(dest),
                })
            }
        } else {
            // default case
            default = String::from(part);
        }
    }
    Ok((rest, Workflow { checks, default }))
}

#[derive(Debug)]
enum Condition {
    Higher,
    Lower,
}
#[derive(Debug)]
enum Field {
    X,
    M,
    A,
    S,
}
impl From<&str> for Field {
    fn from(value: &str) -> Self {
        match value {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub struct Workflow {
    checks: Vec<Check>,
    default: String,
}
impl Workflow {
    fn process(&self, part: &Part) -> String {
        for c in &self.checks {
            let success = match c.cond {
                Condition::Higher => part.get_val(&c.field) > c.val,
                Condition::Lower => part.get_val(&c.field) < c.val,
            };
            if success {
                return c.dest.clone();
            }
        }
        self.default.clone()
    }
}

#[derive(Debug)]
struct Check {
    cond: Condition,
    field: Field,
    val: u32,
    dest: String,
}

#[derive(Debug)]
pub struct Part {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl Part {
    fn get_val(&self, f: &Field) -> u32 {
        match f {
            Field::X => self.x,
            Field::M => self.m,
            Field::A => self.a,
            Field::S => self.s,
        }
    }
}
