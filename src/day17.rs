use crate::common::*;
use crate::intcode::*;
use itertools::Itertools;

type Grid = Vec<Vec<char>>;

pub(crate) fn read_grid(program: &mut Program) -> Result<Grid> {
    let mut grid = Vec::new();
    let (mut x, mut y) = (0, 0);

    while let ExecState::Output(c) = program.resume(None)? {
        if c == '\n' as i64 {
            x = 0;
            y += 1;
        } else {
            if x >= grid.len() {
                grid.push(vec![]);
            }

            if y >= grid[x].len() {
                grid[x].push('.');
            }

            grid[x][y] = c as u8 as char;
            x += 1;
        }
    }

    Ok(grid)
}

fn print_grid(grid: &Grid) {
    for y in 0..grid[0].len() {
        for x in 0..grid.len() {
            print!("{}", grid[x][y]);
        }
        println!();
    }
}

fn find_intersections(grid: &Grid) -> Vec<(usize, usize)> {
    let mut result = vec![];
    let (w, h) = (grid.len(), grid[0].len());

    for x in 1..(w - 1) {
        for y in 1..(h - 1) {
            if grid[x][y] == '#'
                && grid[x][y + 1] == '#'
                && grid[x][y - 1] == '#'
                && grid[x - 1][y] == '#'
                && grid[x + 1][y] == '#'
            {
                result.push((x, y))
            }
        }
    }

    result
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Ord, PartialOrd)]
enum Step {
    Left,
    Right,
    Forward,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

use Direction::*;
use Step::*;

impl Direction {
    fn turn_right(self) -> Direction {
        match self {
            North => East,
            East => South,
            South => West,
            West => North,
        }
    }

    fn turn_left(self) -> Direction {
        self.turn_right().turn_right().turn_right()
    }
}

fn find_path(grid: &Grid) -> Vec<Step> {
    let (w, h) = (grid.len(), grid[0].len());
    let mut robot = None;

    for x in 0..w {
        for y in 0..h {
            if let Some(dir) = match grid[x][y] {
                '^' => Some(North),
                '>' => Some(East),
                '<' => Some(West),
                'v' => Some(South),
                _ => None,
            } {
                robot = Some((x, y, dir));
            }
        }
    }

    let (mut x, mut y, mut dir) = robot.expect("no robot found");
    let mut path = vec![];

    let apply_delta = |x: usize, y: usize, d: Direction| -> (usize, usize) {
        match d {
            North => (x, y.wrapping_sub(1)),
            East => (x + 1, y),
            South => (x, y + 1),
            West => (x.wrapping_sub(1), y),
        }
    };

    let is_walkable = |x: usize, y: usize, d: Direction| -> bool {
        let (nx, ny) = apply_delta(x, y, d);
        nx < w && ny < h && grid[nx][ny] != '.'
    };

    loop {
        if !is_walkable(x, y, dir) {
            if is_walkable(x, y, dir.turn_left()) {
                path.push(Left);
                dir = dir.turn_left();
            } else if is_walkable(x, y, dir.turn_right()) {
                path.push(Right);
                dir = dir.turn_right();
            } else {
                break;
            }
        }

        path.push(Forward);
        let pos = apply_delta(x, y, dir);
        x = pos.0;
        y = pos.1;
    }

    path
}

fn path2command(path: &[Step]) -> String {
    let mut output = String::new();
    let mut n = 0;

    for &step in path {
        if step != Forward && n > 0 {
            output = format!("{}{},", output, n);
            n = 0;
        }

        match step {
            Left => output += "L,",
            Right => output += "R,",
            Forward => n += 1,
        }
    }

    if n > 0 {
        output = format!("{}{},", output, n);
    }

    output.pop();
    output
}

fn find_routines<'a>(
    path: &'a [Step],
    routines: &mut Vec<&'a [Step]>,
    trace: &mut Vec<usize>,
) -> bool {
    // we have reached the end, return true to indicate success
    if path.is_empty() {
        return true;
    }

    // trace cannot be longer than 10 characters
    if trace.len() > 10 {
        return false;
    }

    // check if path starts with any of the existing routines
    for index in 0..routines.len() {
        let routine = routines[index];
        let n = routine.len();

        if path.starts_with(routine) {
            trace.push(index);

            if find_routines(&path[n..], routines, trace) {
                return true;
            }

            trace.pop();
        }
    }

    // if less than 3 routines so far, check if each prefix of path
    // is a possible routine
    if routines.len() < 3 {
        for i in 2..path.len() {
            let slice = &path[..i];

            // routines cannot be longer than 20 characters
            if path2command(slice).len() > 20 {
                continue;
            }

            trace.push(routines.len());
            routines.push(slice);

            if find_routines(&path[i..], routines, trace) {
                return true;
            }

            trace.pop();
            routines.pop();
        }
    }

    false
}

fn send_commands(program: &mut Program, trace: &[usize], routines: &[&[Step]]) -> Result<i64> {
    let mut buffer = String::new();

    let main = trace
        .iter()
        .copied()
        .map(|i| ['A', 'B', 'C'][i])
        .intersperse(',');
    buffer.extend(main);
    buffer.push('\n');

    for routine in routines {
        buffer.push_str(&path2command(routine));
        buffer.push('\n');
    }

    buffer.push_str("y\n");

    let mut input = buffer.chars().map(|c| c as i64);
    let mut output = -1;

    while let ExecState::Output(c) = program.resume(&mut input)? {
        if c < 128 {
            print!("{}", c as u8 as char);
        } else {
            output = c;
            break;
        }
    }

    println!();
    Ok(output)
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let mut program = parse_program("day17")?;
    let grid = read_grid(&mut program)?;
    print_grid(&grid);

    let alignments = map(|(x, y)| x * y, find_intersections(&grid));
    println!("answer A: {}", sum(alignments));

    let path = find_path(&grid);

    let mut routines = vec![];
    let mut trace = vec![];
    find_routines(&path, &mut routines, &mut trace);

    println!("Path: {}", path2command(&path));
    println!("Main: {}", trace.iter().copied().join(","));

    for (index, routine) in enumerate(&routines) {
        println!("Function {}: {}", index, path2command(routine));
    }

    let output = send_commands(&mut program, &trace, &routines)?;
    println!("answer B: {}", output);

    Ok(())
}
