use crate::types::*;

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<Vec<Vec<char>>>;

    fn parse_input(input: &'a str, test: bool) -> Self::Input {
        let groups = input
            .split("\n\n")
            .map(|g| g.lines().map(|l| l.chars().collect()).collect())
            .collect();
        test_print!(test, "groups: {:#?}", groups);
        groups
    }

    fn solve1(&self, groups: &Self::Input, test: bool) -> String {
        groups
            .iter()
            .fold(0, |acc, g| {
                let (x, y) = mirror_line(g, 0);
                test_print!(test, "mirrored at ({}, {})", x, y);
                acc + x + y * 100
            })
            .to_string()
    }

    fn solve2(&self, groups: &Self::Input, test: bool) -> String {
        groups
            .iter()
            .fold(0, |acc, g| {
                let (x, y) = mirror_line(g, 1);
                test_print!(test, "mirrored at ({}, {})", x, y);
                acc + x + y * 100
            })
            .to_string()
    }
}

fn mirror_line(g: &[Vec<char>], errors: usize) -> (u32, u32) {
    // check for horizontal reflection
    'rows: for y in 0..g.len() - 1 {
        let mut smudges = 0;
        for offset in 0..=y.min(g.len() - 2 - y) {
            smudges += g[y - offset]
                .iter()
                .zip(g[y + offset + 1].iter())
                .filter(|(a, b)| a != b)
                .count();
            if smudges > errors {
                continue 'rows;
            }
        }
        if smudges != errors {
            continue 'rows;
        }
        return (0, (y + 1) as u32);
    }
    // check for vertical reflection
    'cols: for x in 0..g[0].len() - 1 {
        let mut smudges = 0;
        for offset in 0..=x.min(g[0].len() - 2 - x) {
            smudges += g
                .iter()
                .map(|row| (row[x - offset], row[x + offset + 1]))
                .filter(|(a, b)| a != b)
                .count();
            if smudges > errors {
                continue 'cols;
            }
        }
        if smudges != errors {
            continue 'cols;
        }
        return ((x + 1) as u32, 0);
    }
    let mut pattern = String::new();
    for row in g {
        pattern.push_str(&row.iter().collect::<String>());
        pattern.push('\n');
    }
    panic!("no symmetry found in pattern:\n{}", pattern);
}
