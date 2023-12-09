use crate::types::*;

pub struct Solver;

pub struct Card {
    winning_numbers: Vec<u8>,
    your_numbers: Vec<u8>,
}

impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<Card>;

    fn parse_input(input: &'a str) -> Self::Input {
        let mut cards = Vec::with_capacity(input.lines().count());
        for line in input.lines() {
            if line.is_empty() {
                continue;
            }
            let numbers = line.split_once(':').unwrap().1.trim();
            let (winning, yours) = numbers.split_once('|').unwrap();
            let winning = winning.trim().split(' ').filter(|s| !s.is_empty());
            let yours = yours.trim().split(' ').filter(|s| !s.is_empty());
            let winning = winning.map(|n| n.trim().parse::<u8>().unwrap()).collect();
            let yours = yours.map(|n| n.trim().parse::<u8>().unwrap()).collect();
            cards.push(Card {
                winning_numbers: winning,
                your_numbers: yours,
            })
        }
        cards
    }

    fn solve1(&self, cards: &Self::Input, test: bool) -> String {
        let mut points = 0;
        for card in cards {
            let matches = count_matches(card);
            if matches == 0 {
                continue;
            }
            points += 2u32.pow(matches - 1);
        }
        points.to_string()
    }

    fn solve2(&self, cards: &Self::Input, test: bool) -> String {
        let mut card_count = vec![1; cards.len()];
        for (i, card) in cards.iter().take(cards.len() - 1).enumerate() {
            let matches = count_matches(card);
            for pos in i + 1..=std::cmp::min(i + matches as usize, cards.len() - 1) {
                card_count[pos] += card_count[i]
            }
        }
        test_print!(test, "{card_count:?}");
        card_count.iter().sum::<u32>().to_string()
    }
}

fn count_matches(card: &Card) -> u32 {
    card.your_numbers
        .iter()
        .filter(|n| card.winning_numbers.contains(n))
        .count() as u32
}
