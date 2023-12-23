use std::collections::{hash_map::Entry, HashMap};

use nom::{bytes::complete::is_not, character::complete, multi::separated_list1, IResult};
use rayon::vec;

use crate::types::*;

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<&'a str>;

    fn parse_input(input: &'a str, test: bool) -> Self::Input {
        let (_, parsed) = nom_parse(input).unwrap();
        test_print!(test, "{:?}", parsed);
        parsed
    }

    fn solve1(&self, input: &Self::Input, test: bool) -> String {
        test_print!(test, "hashing {} items", input.len());
        input
            .iter()
            .fold(0, |acc, input| acc + hash(input) as u64)
            .to_string()
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        let mut boxes: HashMap<u8, Vec<(&str, u8)>> = HashMap::new();
        for input in input {
            if input.ends_with('-') {
                let lbl = &input[..input.len() - 1];
                if let Entry::Occupied(mut b) = boxes.entry(hash(&lbl)) {
                    let b = b.get_mut();
                    if let Some(idx) = b.iter().position(|(lense, _)| lense == &lbl) {
                        b.remove(idx);
                    }
                }
            } else {
                let (lbl, val) = input.split_once('=').unwrap();
                let val = val.parse::<u8>().unwrap();
                let b = boxes.entry(hash(lbl)).or_insert(vec![]);
                if let Some(e) = b.iter_mut().find(|(lense, _)| lense == &lbl) {
                    e.1 = val;
                } else {
                    b.push((lbl, val));
                }
            }
        }
        test_print!(test, "boxes: {:?}", boxes);
        boxes
            .iter()
            .fold(0, |acc, (box_num, b)| {
                acc + b.iter().enumerate().fold(0, |acc, (pos, (_, focal_len))| {
                    acc + (*box_num as u64 + 1) * (pos as u64 + 1) * (*focal_len as u64)
                })
            })
            .to_string()
    }
}

fn nom_parse(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(complete::char(','), is_not(","))(input)
}

fn hash(input: &str) -> u8 {
    let mut hash = 0u8;
    for c in input.bytes() {
        hash = hash.wrapping_add(c).wrapping_mul(17);
    }
    hash
}
