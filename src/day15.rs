use crate::common::*;
use crate::intcode::*;
use ndarray::prelude::*;
use std::collections::VecDeque;
use std::mem::replace;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tile {
    Unknown,
    Wall,
    Goal,
    Empty,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

use Direction::*;
use Tile::*;

impl Direction {
    fn command(self) -> i64 {
        match self {
            North => 1,
            South => 2,
            West => 3,
            East => 4,
        }
    }

    fn apply_delta(self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            North => (x, y-1),
            South => (x, y+1),
            West => (x-1, y),
            East => (x+1, y),
        }
    }
}

fn build_grid(dim: usize, start: (usize, usize), program: Program) -> Result<Array2<Tile>> {
    let mut grid = Array2::from_elem((dim, dim), Unknown);
    let mut queue = VecDeque::new();

    grid[start] = Empty;
    queue.push_back((start, program));

    while let Some((index, program)) = queue.pop_back() {
        for &d in &[North, South, East, West] {
            let index = d.apply_delta(index);

            let mut mirror = program.clone();
            let tile = match mirror.resume(Some(d.command()))? {
                ExecState::Output(0) => Wall,
                ExecState::Output(1) => Empty,
                ExecState::Output(2) => Goal,
                _ => panic!("invalid program"),
            };

            if replace(&mut grid[index], tile) == Unknown && tile != Wall {
                queue.push_back((index, mirror));
            }
        }
    }

    Ok(grid)
}

fn print_grid(grid: ArrayView2<Tile>) {
    for col in grid.gencolumns() {
        for v in col {
            let c = match v {
                Unknown => '?',
                Wall => '#',
                Empty => ' ',
                Goal => '!',
            };

            print!("{}", c);
        }
        println!();
    }
}

fn calculate_dist(start: (usize, usize), grid: ArrayView2<Tile>) -> Array2<i64> {
    let mut dist = grid.map(|_| std::i64::MAX);
    let mut queue = VecDeque::new();
    queue.push_front((start, 0));

    while let Some((index, d)) = queue.pop_front() {
        for &dir in &[North, East, South, West] {
            let index = dir.apply_delta(index);

            if grid[index] != Wall && dist[index] > d + 1 {
                dist[index] = d + 1;
                queue.push_back((index, d + 1));
            }
        }
    }

    dist
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let program = parse_program("day15")?;

    let dim = 50;
    let start = ((dim / 2), (dim / 2));
    let grid = build_grid(dim, start, program)?;

    print_grid(grid.view());

    let goal = grid
        .indexed_iter()
        .filter(|(_, &val)| val == Goal)
        .next()
        .unwrap().0;

    let dist = calculate_dist(goal, grid.view());
    println!("answer A: {}", dist[start]);

    let max_dist = dist
        .iter()
        .filter(|&&d| d != std::i64::MAX)
        .max();

    println!("answer B: {:?}", max_dist);



    Ok(())
}
