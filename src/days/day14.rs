use std::{
    collections::{hash_map::DefaultHasher, HashMap},
    hash::{Hash, Hasher},
};

use crate::types::*;

pub struct Solver;

impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<Vec<char>>;

    fn parse_input(input: &'a str, _test: bool) -> Self::Input {
        input.lines().map(|l| l.chars().collect()).collect()
    }

    fn solve1(&self, map: &Self::Input, test: bool) -> String {
        // roll rocks north
        let map = map.clone();
        let map = roll(map.clone(), (0, -1));
        let load = count_load(&map);
        if test {
            println!("map after rolling:");
            for row in &map {
                println!("{}", row.iter().collect::<String>());
            }
        }
        // add load for rocks on top
        load.to_string()
    }

    fn solve2(&self, input: &Self::Input, _test: bool) -> String {
        let mut map = input.clone();
        let mut hashed_maps = HashMap::new();
        let mut cycles = 0;
        loop {
            for dir in &[(0, -1), (-1, 0), (0, 1), (1, 0)] {
                map = roll(map, *dir);
            }
            cycles += 1;
            let mut hasher = DefaultHasher::new();
            map.hash(&mut hasher);
            let hash = hasher.finish();
            let hashed_at = hashed_maps.entry(hash).or_insert(cycles);
            if *hashed_at < cycles {
                // found a cycle
                let cycle_len = cycles - *hashed_at;
                let remaining = (1000000000 - cycles) % cycle_len;
                for _ in 0..remaining {
                    for dir in &[(0, -1), (-1, 0), (0, 1), (1, 0)] {
                        map = roll(map, *dir);
                    }
                }
                break;
            }
        }
        count_load(&map).to_string()
    }
}

fn count_load(map: &[Vec<char>]) -> usize {
    let mut load = 0;
    for (y, row) in map.iter().enumerate() {
        for c in row {
            if *c == 'O' {
                load += map.len() - y;
            }
        }
    }
    load
}

fn roll(mut map: Vec<Vec<char>>, dir: (i32, i32)) -> Vec<Vec<char>> {
    let (x_start, x_end, x_loop) = if dir.0 < 0 {
        (0, map[0].len() as i32, 1)
    } else {
        (map[0].len() as i32 - 1, -1, -1)
    };
    let (mut y, y_end, y_loop) = if dir.1 < 0 {
        (0, map.len() as i32, 1)
    } else {
        (map.len() as i32 - 1, -1, -1)
    };
    while y != y_end {
        let mut x = x_start;
        while x != x_end {
            if map[y as usize][x as usize] == 'O' {
                // move rock as far as possible
                let mut new_x = x + dir.0;
                let mut new_y = y + dir.1;
                while new_y >= 0
                    && new_y < map.len() as i32
                    && new_x >= 0
                    && new_x < map[0].len() as i32
                    && map[new_y as usize][new_x as usize] == '.'
                {
                    new_x += dir.0;
                    new_y += dir.1;
                }
                if x != new_x || y != new_y {
                    map[y as usize][x as usize] = '.';
                    map[(new_y - dir.1) as usize][(new_x - dir.0) as usize] = 'O';
                }
            }
            x += x_loop;
        }
        y += y_loop;
    }
    map
}
