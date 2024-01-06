use std::{
    collections::{BinaryHeap, HashMap, HashSet},
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
        let res = (other.h + other.g).cmp(&(self.h + self.g));
        if res == std::cmp::Ordering::Equal {
            return other.g.cmp(&self.g);
        }
        res
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
        let end = find_path(grid);
        if test {
            // print path
            let path = get_path(end.clone());
            for (y, row) in grid.iter().enumerate() {
                for (x, cost) in row.iter().enumerate() {
                    if path.contains(&(x, y)) {
                        print!("#")
                    } else {
                        print!("{cost}")
                    }
                }
                println!()
            }
        }
        end.g.to_string()
    }

    fn solve2(&self, input: &Self::Input, test: bool) -> String {
        String::from("ToDo")
    }
}

fn get_path(end: Rc<Node>) -> Vec<(usize, usize)> {
    let mut path = vec![end.pos];
    let mut current = end;
    while let Some(node) = current.prev.clone() {
        path.push(node.pos);
        current = node;
    }
    path.reverse();
    path
}

fn find_path(grid: &[Vec<u8>]) -> Rc<Node> {
    let mut queue = BinaryHeap::new();
    let start = Rc::new(Node {
        g: 0,
        h: grid.len() + grid[0].len(),
        pos: (0, 0),
        prev: None,
        last_dir: 0,
        same_steps: 0,
    });
    queue.push(start.clone());
    let mut visited = HashMap::new();
    visited.insert((0, 0), start);
    while let Some(current) = queue.pop() {
        // check if goal reached
        if current.pos == (grid[0].len() - 1, grid.len() - 1) {
            return current;
        }
        // add neighbors
        for dir in 0..4 {
            // cant go back
            if dir == (current.last_dir + 2) % 4 {
                continue;
            }
            // cant go same dir more than 3 times
            if dir == current.last_dir && current.same_steps == 3 {
                continue;
            }
            let next_pos = match dir {
                0 if current.pos.1 > 0 => (current.pos.0, current.pos.1 - 1),
                1 if current.pos.0 < grid[0].len() - 1 => (current.pos.0 + 1, current.pos.1),
                2 if current.pos.1 < grid.len() - 1 => (current.pos.0, current.pos.1 + 1),
                3 if current.pos.0 > 0 => (current.pos.0 - 1, current.pos.1),
                _ => continue,
            };
            let cost = current.g + grid[next_pos.1][next_pos.0] as usize;
            // check if better path exists already
            if let Some(prev) = visited.get(&next_pos) {
                if prev.g <= cost {
                    continue;
                }
            }
            // create new node
            let node = Rc::new(Node {
                g: cost,
                h: grid.len() - next_pos.1 + grid[0].len() - next_pos.0,
                same_steps: if dir == current.last_dir {
                    current.same_steps + 1
                } else {
                    1
                },
                last_dir: dir,
                pos: next_pos,
                prev: Some(current.clone()),
            });
            queue.push(node.clone());
            visited.insert(next_pos, node);
        }
    }
    panic!("no path found");
}
