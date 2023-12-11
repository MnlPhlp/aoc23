use std::{
    collections::{hash_map::Entry, HashMap},
    time::Instant,
};

use nom::{
    bytes::complete::{is_a, tag},
    character::complete::{alphanumeric1, multispace1, newline},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::types::*;

pub struct Solver;

#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

#[derive(Debug)]
pub struct Map<'a> {
    directions: Vec<u8>,
    nodes: HashMap<&'a str, Node<'a>>,
}

fn node(input: &str) -> IResult<&str, Node> {
    let (rest, (name, _, left, _, right, _)) = tuple((
        alphanumeric1,
        tag(" = ("),
        alphanumeric1,
        tag(", "),
        alphanumeric1,
        tag(")"),
    ))(input)?;
    Ok((rest, Node { name, left, right }))
}

fn nom_parse(input: &str) -> IResult<&str, Map> {
    let (rest, (directions, _, mut nodes)) =
        tuple((is_a("RL"), multispace1, separated_list1(newline, node)))(input)?;
    let directions = directions.bytes().collect();
    let mut node_map = HashMap::with_capacity(nodes.len());
    nodes.drain(0..nodes.len()).for_each(|node| {
        node_map.insert(node.name, node);
    });
    Ok((
        rest,
        Map {
            directions,
            nodes: node_map,
        },
    ))
}

impl<'a> DaySolver<'a> for Solver {
    type Input = Map<'a>;

    fn parse_input(input: &'a str, test: bool) -> Self::Input {
        let (_, map) = nom_parse(input).unwrap();
        test_print!(test, "map: {map:#?}");
        map
    }

    fn solve1(&self, map: &Self::Input, test: bool) -> String {
        count_moves("AAA", map, |p| p == "ZZZ").to_string()
    }

    // this solution assumes that the count of moves is the same after reaching the first Z
    // this is true for the input, but not necessarily for other inputs
    fn solve2(&self, map: &Self::Input, test: bool) -> String {
        // get starting positions
        let pos = map.nodes.keys().filter(|k| k.ends_with('A'));

        // calculate moves per starting position
        let move_counts = pos
            .map(|p| count_moves(p, map, |p| p.ends_with('Z')))
            .collect::<Vec<_>>();

        // find smallest common multiple
        let mut steps = move_counts[0];
        for i in 1..move_counts.len() {
            steps = num::integer::lcm(steps, move_counts[i]);
        }

        steps.to_string()
    }
}

fn count_moves(starting_pos: &str, map: &Map<'_>, end_check: impl Fn(&str) -> bool) -> usize {
    let mut moves = 0;
    let mut pos = starting_pos;
    while !end_check(pos) {
        if map.directions[moves % map.directions.len()] == b'L' {
            pos = map.nodes[pos].left;
        } else {
            pos = map.nodes[pos].right;
        }
        moves += 1;
    }
    moves
}
