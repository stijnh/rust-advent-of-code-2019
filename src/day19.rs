use crate::common::*;
use crate::intcode::*;
use ndarray::prelude::*;

pub(crate) fn is_affected(program: &Program, x: usize, y: usize) -> Result<bool> {
    let mut p = program.clone();
    if let ExecState::Output(i) = p.resume(vec![x as i64, y as i64])? {
        Ok(i > 0)
    } else {
        Ok(false)
    }
}

pub(crate) fn scan_grid(program: &Program, dim: usize) -> Result<Array2<bool>> {
    let mut grid = Array2::from_elem((dim, dim), false);

    for ((x, y), entry) in grid.indexed_iter_mut() {
        *entry = is_affected(program, x, y)?;
    }

    Ok(grid)
}

pub(crate) fn fit_ship(program: &Program) -> Result<(usize, usize)> {
    let dim = 1500;
    let ship_size = 100;
    let grid = scan_grid(program, dim)?;
    let mut result = (0, 0);
    let mut min_dist = std::usize::MAX;

    for x in 0..(dim - ship_size) {
        for y in 0..(dim - ship_size) {
            let mut is_valid = true;
            let dist = x * x + y * y;

            if dist > min_dist {
                continue;
            }

            for dx in 0..ship_size {
                for dy in 0..ship_size {
                    if !grid[[x + dx, y + dy]] {
                        is_valid = false;
                        break;
                    }
                }
            }

            if is_valid {
                min_dist = dist;
                result = (x, y);
            }
        }
    }

    Ok(result)
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let program = parse_program("day19")?;

    let count = scan_grid(&program, 50)?
        .iter()
        .map(|&x| x as usize)
        .sum::<usize>();
    println!("answer A: {}", count);

    let answer = fit_ship(&program);
    println!("answer B: {:?}", answer);

    Ok(())
}
