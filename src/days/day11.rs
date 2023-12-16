use crate::types::*;

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<Vec<bool>>;

    fn parse_input(input: &'a str, test: bool) -> Self::Input {
        let mut map = Vec::new();
        for line in input.lines() {
            let row = line
                .chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("invalid input"),
                })
                .collect::<Vec<_>>();
            map.push(row);
        }
        map
    }

    fn solve1(&self, map: &Self::Input, test: bool) -> String {
        let galaxies = expand(map, 2);
        test_print!(test, "galaxies: {:#?}", galaxies);
        let mut distance = 0;
        for a in 0..galaxies.len() {
            for b in a + 1..galaxies.len() {
                distance += galaxies[a].distance(&galaxies[b]);
            }
        }
        distance.to_string()
    }

    fn solve2(&self, map: &Self::Input, test: bool) -> String {
        let galaxies = expand(map, 1000000);
        test_print!(test, "galaxies: {:#?}", galaxies);
        let mut distance = 0;
        for a in 0..galaxies.len() {
            for b in a + 1..galaxies.len() {
                distance += galaxies[a].distance(&galaxies[b]);
            }
        }
        distance.to_string()
    }
}

fn expand(map: &[Vec<bool>], expand: usize) -> Vec<Position> {
    let mut empty_columns = Vec::new();
    for x in 0..map[0].len() {
        let mut empty = true;
        for y in 0..map.len() {
            if map[y][x] {
                empty = false;
                break;
            }
        }
        if empty {
            empty_columns.push(x);
        }
    }
    let mut empty_rows = 0;
    let mut galaxies = Vec::new();
    for y in 0..map.len() {
        let mut empty = true;
        for x in 0..map[0].len() {
            if map[y][x] {
                empty = false;
                let y = y + empty_rows;
                let columns = empty_columns.iter().filter(|&&c| c < x).count();
                let x = x + columns * expand - columns;
                galaxies.push(Position::new(x, y));
            }
        }
        if empty {
            empty_rows += expand - 1;
        }
    }
    galaxies
}
