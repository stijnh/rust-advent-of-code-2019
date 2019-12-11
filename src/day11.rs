use crate::common::*;
use crate::intcode::*;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Copy, Clone, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West
}

use Direction::*;

impl Direction {
    fn turn_right(self) -> Self {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn turn_left(self) -> Self {
        self.turn_right().turn_right().turn_right()
    }

    fn delta(self) -> (isize, isize) {
        match self {
            North => (0, -1),
            East => (1, 0),
            South => (0, 1),
            West => (-1, 0),
        }
    }
}

fn paint(mut program: Program, start_tile: i64) -> Result<HashMap<(isize, isize), i64>> {
    let (mut x, mut y) = (0, 0);
    let mut direction = North;
    let mut panels = HashMap::new();
    panels.insert((x, y), start_tile);

    loop {
        let color = panels
            .get(&(x, y))
            .copied()
            .unwrap_or(0);

        let new_color = match program.resume(Some(color))? {
            ExecState::Halted => break,
            ExecState::Output(c) => c,
            ExecState::Input => Err(ExecError::InputExhausted)?,
        };

        panels.insert((x, y), new_color);

        let turn = match program.resume(None)? {
            ExecState::Halted => break,
            ExecState::Output(c) => c,
            ExecState::Input => Err(ExecError::InputExhausted)?,
        };

        direction = match turn {
            0 => direction.turn_left(),
            _ => direction.turn_right(),
        };

        let (dx, dy) = direction.delta();
        x += dx;
        y += dy;
    }

    Ok(panels)
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let program = parse_program("day11")?;

    let panels = paint(program.clone(), 0)?;
    println!("answer A: {:?}", panels.len());

    let panels = paint(program.clone(), 1)?;
    let (min_x, max_x) = (-50, 50);
    let (min_y, max_y) = (-10, 10);

    println!("answer B:");
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", iff!(panels.get(&(x, y)) == Some(&1), '#', ' '));
        }
        println!();
    }

    Ok(())
}
