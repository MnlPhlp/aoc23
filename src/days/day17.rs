use std::{
    collections::{BinaryHeap, HashMap},
    rc::Rc,
};

use crate::types::*;

pub struct Solver;

#[derive(PartialEq, Eq, Debug, Clone)]
struct Node {
    g: usize,
    h: usize,
    /// number of steps done in same direction
    same_steps: u8,
    /// last step dir (0 = up, 1 = right, 2 = down, 3 = left)
    last_dir: u8,
    pos: (usize, usize),
    prev: Option<Rc<Node>>,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (other.h + other.g).cmp(&(self.h + self.g))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<'a> DaySolver<'a> for Solver {
    type Input = Vec<Vec<u8>>;

    fn parse_input(input: &'a str, _test: bool) -> Self::Input {
        input
            .lines()
            .map(|l| l.as_bytes().iter().map(|b| b - b'0').collect())
            .collect()
    }

    fn solve1(&self, grid: &Self::Input, test: bool) -> String {
        let (end, visited) = find_path(grid, false, test);
        if test {
            print_path(Some(end.clone()), grid, &visited);
        }
        end.g.to_string()
    }

    fn solve2(&self, grid: &Self::Input, test: bool) -> String {
        let (end, visited) = find_path(grid, true, test);
        if test {
            print_path(Some(end.clone()), grid, &visited);
        }
        end.g.to_string()
    }
}

fn print_path(
    end: Option<Rc<Node>>,
    grid: &[Vec<u8>],
    visited: &HashMap<(usize, usize, u8, u8), Rc<Node>>,
) {
    let mut path = vec![];
    if let Some(end) = end {
        path.push(end.pos);
        let mut current = end;
        while let Some(node) = current.prev.clone() {
            path.push(node.pos);
            current = node;
        }
        path.reverse();
    }
    let visited = visited
        .keys()
        .map(|&(x, y, _, _)| (x, y))
        .collect::<Vec<_>>();
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
    for (y, row) in grid.iter().enumerate() {
        for (x, cost) in row.iter().enumerate() {
            if path.contains(&(x, y)) {
                print!("O")
            } else if visited.contains(&(x, y)) {
                print!("#")
            } else {
                print!("{cost}")
            }
        }
        println!()
    }
}

fn find_path(
    grid: &[Vec<u8>],
    part_two: bool,
    test: bool,
) -> (Rc<Node>, HashMap<(usize, usize, u8, u8), Rc<Node>>) {
    let mut queue = BinaryHeap::new();
    // let mut queue = Vec::new();
    let start = Rc::new(Node {
        g: 0,
        h: (grid.len() + grid[0].len()),
        pos: (0, 0),
        prev: None,
        last_dir: 0,
        same_steps: 0,
    });
    queue.push(start.clone());
    let mut visited: HashMap<(usize, usize, u8, u8), Rc<Node>> = HashMap::new();
    let max_steps = if part_two { 10 } else { 3 };
    while let Some(current) = queue.pop() {
        if test {
            print_path(None, grid, &visited);
        }
        // check if goal reached
        if current.pos == (grid[0].len() - 1, grid.len() - 1)
            && (!part_two || current.same_steps >= 4)
        {
            return (current, visited);
        }
        let key = (
            current.pos.0,
            current.pos.1,
            current.last_dir,
            current.same_steps,
        );
        // check if better path exists already
        if let Some(prev) = visited.get(&key) {
            if prev.g <= current.g {
                continue;
            }
        }
        visited.insert(key, current.clone());
        // add neighbors
        for dir in 0..4 {
            // cant go back
            if dir == (current.last_dir + 2) % 4 {
                continue;
            }
            // cant go same dir more than n times
            if dir == current.last_dir && current.same_steps == max_steps {
                continue;
            }
            // part two can only turn after 4 moves
            if part_two
                && current.same_steps < 4
                && dir != current.last_dir
                && current.pos != (0, 0)
            {
                continue;
            }
            let next_pos = match dir {
                0 if current.pos.1 > 0 => (current.pos.0, current.pos.1 - 1),
                1 if current.pos.0 < grid[0].len() - 1 => (current.pos.0 + 1, current.pos.1),
                2 if current.pos.1 < grid.len() - 1 => (current.pos.0, current.pos.1 + 1),
                3 if current.pos.0 > 0 => (current.pos.0 - 1, current.pos.1),
                _ => continue,
            };
            let next_g = current.g + grid[next_pos.1][next_pos.0] as usize;
            let next_h = (grid.len() - next_pos.1 + grid[0].len() - next_pos.0) * 2;
            let next_steps = if dir == current.last_dir {
                current.same_steps + 1
            } else {
                1
            };
            // create new node
            let node = Rc::new(Node {
                g: next_g,
                h: next_h,
                same_steps: next_steps,
                last_dir: dir,
                pos: next_pos,
                prev: Some(current.clone()),
            });
            queue.push(node.clone());
        }
    }
    panic!("no path found");
}
