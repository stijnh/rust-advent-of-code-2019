use crate::common::*;
use std::collections::{HashMap, VecDeque};
use binary_heap_plus::BinaryHeap;
use ndarray::prelude::*;
use std::fmt;
use itertools::Itertools;

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
struct KeySet(u32);

impl fmt::Display for KeySet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("KeySet(")?;

        for i in 0..26 {
            let x = (('a' as u8) + i) as char;
            let c = iff!(self.contains_key(x), x, '.');
            write!(f, "{}", c)?;
        }

        f.write_str(")")
    }
}

impl KeySet {
    fn new() -> Self {
        Self(0)
    }

    fn all() -> Self {
        Self((1 << 26) - 1)
    }

    fn from(c: char) -> Self {
        if c >= 'a' && c <= 'z' {
            let i = (c as usize) - ('a' as usize);
            let mask = 1 << i;
            Self(mask)
        } else {
            Self(0)
        }
    }

    fn opens_door(self, c: char) -> bool {
        if c >= 'A' && c <= 'Z' {
            let i = (c as usize) - ('A' as usize);
            let mask = 1 << i;
            self.0 & mask != 0
        } else {
            false
        }
    }

    fn contains_key(self, c: char) -> bool {
        self.union(KeySet::from(c)) == self
    }

    fn union(self, other: KeySet) -> Self {
        KeySet(self.0 | other.0)
    }
}

fn build_distance_matrix(grid: ArrayView2<char>, characters: &[char]) -> Array2<Option<usize>> {
    let n = characters.len();
    let mut dists = Array2::from_elem((n, n), None);

    for (index, &target) in enumerate(characters) {
        let (x, y) = grid
            .indexed_iter()
            .find(move |(_, &v)| v == target)
            .expect("to find character")
            .0;

        let mut queue = VecDeque::new();
        let mut visited = grid.map(|_| false);

        queue.push_back((x, y, 0));
        visited[[x, y]] = true;

        while let Some((cx, cy, dist)) = queue.pop_front() {
            let c = grid[[cx, cy]];

            if let Some(offset) = characters.iter().position(|&v| v == c) {
                dists[[index, offset]] = Some(dist);
            }

            if c != '.' && c != target {
                continue;
            }

            for &[x, y] in &[[cx -1, cy], [cx + 1, cy], [cx, cy - 1], [cx, cy + 1]] {
                if !visited[[x, y]] {
                    visited[[x, y]] = true;
                    queue.push_back((x, y, dist + 1));
                }
            }
        }
    }

    dists
}

fn explore_grid(grid: ArrayView2<char>) -> Option<usize> {
    let nodes = "@abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect_vec();

    let dists = build_distance_matrix(grid.view(), &nodes);

    let mut queue = VecDeque::new();
    let mut states = HashMap::new();

    states.insert((0, KeySet::new()), 0);
    queue.push_back((0, KeySet::new()));

    while let Some((index, keys)) = queue.pop_front() {
        let d = states[&(index, keys)];

        for (next_index, (&c, &l)) in enumerate(zip(&nodes, &dists.row(index))) {
            if let Some(l) = l {
                if (c >= 'A' && c <= 'Z') && !keys.opens_door(c) {
                    continue;
                }

                let next_keys = KeySet::union(keys, KeySet::from(c));
                let entry = states
                    .entry((next_index, next_keys))
                    .or_insert(std::usize::MAX);

                if *entry > d + l {
                    *entry = d + l;
                    queue.push_back((next_index, next_keys));
                }
            }
        }
    }

    states
        .into_iter()
        .filter_map(|((_, keys), dist)| iff!(keys == KeySet::all(), Some(dist), None))
        .min()
}


fn explore_grid_four(grid: ArrayView2<char>) -> Option<usize> {
    let nodes = "0123abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .collect_vec();

    let dists = build_distance_matrix(grid.view(), &nodes);

    let mut queue = BinaryHeap::new_by_key(|p: &([usize; 4], KeySet, usize)| -(p.2 as isize));
    let mut states = HashMap::new();

    let k = ([0, 1, 2, 3], KeySet::new());
    states.insert(k, 0);
    queue.push((k.0, k.1, 0));
    let mut waiting = 0;

    while let Some((indices, keys, d)) = queue.pop() {
        if waiting == 0 {
            println!("{} {} {}", d, states.len(), queue.len());
            waiting = 1000000;
        }
        waiting -= 1;

        if keys == KeySet::all() {
            return Some(d);
        }

        for robot in 0..4 {
            let index = indices[robot];

            for (next_index, (&c, &l)) in enumerate(zip(&nodes, &dists.row(index))) {
                if let Some(l) = l {
                    if (c >= 'A' && c <= 'Z') && !keys.opens_door(c) {
                        continue;
                    }

                    let next_keys = KeySet::union(keys, KeySet::from(c));
                    let mut next_indices = indices;
                    next_indices[robot] = next_index;


                    let entry = states
                        .entry((next_indices, next_keys))
                        .or_insert(std::usize::MAX);

                    if *entry > d + l {
                        *entry = d + l;
                        queue.push((next_indices, next_keys, d + l));
                    }
                }
            }
        }
    }

    None
}


pub(crate) fn run(_args: &[&str]) -> Result {
    let lines = read_input("day18")?;
    let (w, h) = (lines.len(), lines[0].len());
    let mut grid = Array2::from_elem((w, h), '#');

    for (x, line) in enumerate(lines) {
        for (y, c) in enumerate(line.chars()) {
            grid[[x, y]] = c;
        }
    }

    let answer = explore_grid(grid.view());
    println!("answer A: {:?}", answer);

    let mut grid = grid.clone();
    let (x, y) = (w / 2, h / 2);
    grid[[x - 1, y - 1]] = '0';
    grid[[x - 1, y    ]] = '#';
    grid[[x - 1, y + 1]] = '1';
    grid[[x    , y - 1]] = '#';
    grid[[x    , y    ]] = '#';
    grid[[x    , y + 1]] = '#';
    grid[[x + 1, y - 1]] = '2';
    grid[[x + 1, y    ]] = '#';
    grid[[x + 1, y + 1]] = '3';

    let answer = explore_grid_four(grid.view());
    println!("answer B: {:?}", answer);



    Ok(())
}
