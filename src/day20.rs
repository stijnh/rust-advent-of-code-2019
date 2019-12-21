use crate::common::*;
use std::collections::{HashMap, HashSet, VecDeque};

type Grid = HashMap<(i32, i32), char>;

fn read_grid() -> Result<Grid> {
    let mut grid = Grid::new();

    for (x, line) in enumerate(read_input("day20")?) {
        for (y, c) in enumerate(line.chars()) {
            if c != ' ' {
                grid.insert((x as i32, y as i32), c);
            }
        }
    }

    Ok(grid)
}

fn find_portals(grid: &Grid) -> HashMap<String, Vec<(i32, i32)>> {
    let mut output: HashMap<_, Vec<_>> = HashMap::new();

    for (&(x, y), &c) in grid {
        if c == '.' {
            for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let a = grid.get(&(x - dx, y - dy)).unwrap_or(&' ');
                let b = grid.get(&(x - 2 * dx, y - 2 * dy)).unwrap_or(&' ');

                if a.is_ascii_alphabetic() && b.is_ascii_alphabetic() {
                    let name = iff!(
                        dx > 0 || dy > 0,
                        format!("{}{}", a, b),
                        format!("{}{}", b, a)
                    );

                    output.entry(name).or_default().push((x, y));
                }
            }
        }
    }

    output
}

fn distance_to_center((x, y): (i32, i32)) -> i32 {
    (x - 50) * (x - 50) + (y - 50) * (y - 50)
}

fn find_path_length(start: &str, end: &str, grid: &Grid, recur_space: bool) -> Option<usize> {
    let portals = find_portals(&grid);
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut options = vec![];

    let (sx, sy) = portals[start][0];
    let (ex, ey) = portals[end][0];
    queue.push_back(((sx, sy, 0), 0));

    let mut teleports = HashMap::new();
    for (_, list) in portals {
        if list.len() == 2 {
            let (a, b) = (list[0], list[1]);

            let delta = if !recur_space {
                0
            } else if distance_to_center(a) < distance_to_center(b) {
                1
            } else {
                -1
            };

            teleports.insert(a, (b.0, b.1, delta));
            teleports.insert(b, (a.0, a.1, -delta));
        }
    }

    while let Some(((x, y, ring), dist)) = queue.pop_front() {
        if (x, y, ring) == (ex, ey, 0) {
            return Some(dist);
        }

        for &(dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if grid.get(&(x + dx, y + dy)) == Some(&'.') {
                options.push((x + dx, y + dy, ring));
            }
        }

        if let Some(&(nx, ny, delta)) = teleports.get(&(x, y)) {
            if ring + delta >= 0 {
                options.push((nx, ny, ring + delta));
            }
        }

        for p in options.drain(..) {
            if visited.insert(p) {
                queue.push_back((p, dist + 1));
            }
        }
    }

    None
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let grid = read_grid()?;

    let answer_a = find_path_length("AA", "ZZ", &grid, false);
    println!("answer A: {:?}", answer_a);

    let answer_b = find_path_length("AA", "ZZ", &grid, true);
    println!("answer B: {:?}", answer_b);

    Ok(())
}
