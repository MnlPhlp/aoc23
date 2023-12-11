use std::cmp::Ordering;

use strum::{EnumIter, IntoEnumIterator};

use crate::types::*;

pub struct Solver;

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd, EnumIter)]
enum CardValue {
    Joker,
    Num2,
    Num3,
    Num4,
    Num5,
    Num6,
    Num7,
    Num8,
    Num9,
    Num10,
    Jack,
    Queen,
    King,
    Ace,
}

impl CardValue {
    fn new(c: u8, joker: bool) -> CardValue {
        match c {
            b'A' => Self::Ace,
            b'K' => Self::King,
            b'Q' => Self::Queen,
            b'J' => {
                if joker {
                    Self::Joker
                } else {
                    Self::Jack
                }
            }
            b'T' => Self::Num10,
            b'9' => Self::Num9,
            b'8' => Self::Num8,
            b'7' => Self::Num7,
            b'6' => Self::Num6,
            b'5' => Self::Num5,
            b'4' => Self::Num4,
            b'3' => Self::Num3,
            b'2' => Self::Num2,
            _ => panic!("invalid card"),
        }
    }
}

#[derive(Debug, Ord, Eq, PartialEq, PartialOrd)]
enum Rank {
    High,
    Pair,
    TwoPairs,
    Three,
    FullHouse,
    Four,
    Five,
}

#[derive(Debug, Eq)]
pub struct Hand {
    cards: [CardValue; 5],
    rank: Rank,
    bid: u32,
}

impl PartialEq for Hand {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.rank.cmp(&other.rank) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => {
                for (a, b) in self.cards.iter().zip(&other.cards) {
                    match a.cmp(b) {
                        Ordering::Less => {
                            return Ordering::Less;
                        }
                        Ordering::Greater => {
                            return Ordering::Greater;
                        }
                        Ordering::Equal => {
                            continue;
                        }
                    }
                }
                Ordering::Less
            }
            Ordering::Greater => Ordering::Greater,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hand {
    fn new(cards: &str, bid: u32, joker: bool) -> Self {
        let cards = cards
            .bytes()
            .map(|c| CardValue::new(c, joker))
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        Self {
            rank: get_rank(&cards),
            cards,
            bid,
        }
    }
}

fn get_rank(cards: &[CardValue; 5]) -> Rank {
    let mut counts = Vec::with_capacity(5);
    for card in CardValue::iter() {
        if card == CardValue::Joker {
            continue;
        }
        let count = cards.iter().filter(|c| **c == card).count();
        counts.push(count);
    }
    let jokers = cards.iter().filter(|c| **c == CardValue::Joker).count();
    let max = *counts.iter().max().unwrap_or(&0);
    let max_pos = counts.iter().position(|n| *n == max).unwrap();
    counts[max_pos] = 0;
    let max = max + jokers;
    if max == 5 {
        return Rank::Five;
    }
    if max == 4 {
        return Rank::Four;
    }
    if max == 3 {
        if counts.contains(&2) {
            return Rank::FullHouse;
        }
        return Rank::Three;
    }
    if max == 2 {
        if counts.contains(&2) {
            return Rank::TwoPairs;
        }
        return Rank::Pair;
    }
    Rank::High
}

impl<'a> DaySolver<'a> for Solver {
    type Input = (Vec<Hand>, Vec<Hand>);

    fn parse_input(input: &'a str, _test: bool) -> Self::Input {
        let mut hands1 = Vec::with_capacity(input.lines().count());
        let mut hands2 = Vec::with_capacity(input.lines().count());
        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            let (cards, bid) = line.split_once(' ').unwrap();
            let bid = bid.trim().parse().unwrap();
            hands1.push(Hand::new(cards, bid, false));
            hands2.push(Hand::new(cards, bid, true));
        }
        (hands1, hands2)
    }

    fn solve1(&self, input: &Self::Input, _test: bool) -> String {
        let (hands, _) = input;
        sum_hands(hands).to_string()
    }

    fn solve2(&self, input: &Self::Input, _test: bool) -> String {
        let (_, hands) = input;
        sum_hands(hands).to_string()
    }
}

fn sum_hands(hands: &[Hand]) -> u32 {
    let mut hands = hands.iter().collect::<Vec<_>>();
    hands.sort_unstable();
    let mut sum = 0;
    for (rank, hand) in hands.iter().enumerate() {
        let rank = rank + 1;
        sum += rank as u32 * hand.bid;
    }
    sum
}
