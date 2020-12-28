use crate::common::*;
use defaultmap::DefaultHashMap;
use std::collections::HashSet;

const DIM: i32 = 5;
const MID: i32 = 2;
type Pos = (i32, i32);
type PosLevel = (i32, i32, i32);

fn parse_input(lines: &[String]) -> Result<HashSet<Pos>> {
    let mut bugs: HashSet<Pos> = default();

    for (i, line) in enumerate(lines) {
        for (j, c) in enumerate(line.chars()) {
            if c == '#' {
                bugs.insert((i as _, j as _));
            }
        }
    }

    Ok(bugs)
}

fn evolve(mut bugs: HashSet<Pos>) -> HashSet<Pos> {
    let mut counts = DefaultHashMap::<Pos, usize>::new(0);

    for &(i, j) in &bugs {
        counts[(i, j)] += 0;

        for &index in &[(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
            counts[index] += 1;
        }
    }

    for (&(i, j), &c) in counts.iter() {
        if (i >= 0 && j >= 0 && i < DIM && j < DIM)
            && (c == 1 || (c == 2 && !bugs.contains(&(i, j))))
        {
            bugs.insert((i, j));
        } else {
            bugs.remove(&(i, j));
        }
    }

    bugs
}

fn evolve_until_repeats(mut bugs: HashSet<Pos>) -> i32 {
    let mut scores: HashSet<_> = default();

    loop {
        let score = sum(map(|&(i, j)| 1 << (i * DIM + j), &bugs));
        if !scores.insert(score) {
            break score;
        }

        bugs = evolve(bugs);
    }
}

fn evolve_recur(mut bugs: HashSet<PosLevel>) -> HashSet<PosLevel> {
    let mut counts = DefaultHashMap::<PosLevel, usize>::new(0);

    for &(i, j, l) in &bugs {
        counts[(i, j, l)] += 0;

        for &(p, q) in &[(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)] {
            if p < 0 {
                counts[(MID - 1, MID, l + 1)] += 1; // top row
            } else if p >= DIM {
                counts[(MID + 1, MID, l + 1)] += 1; // bot row
            } else if q < 0 {
                counts[(MID, MID - 1, l + 1)] += 1; // left col
            } else if q >= DIM {
                counts[(MID, MID + 1, l + 1)] += 1; // right col
            } else if (p, q) != (MID, MID) {
                counts[(p, q, l)] += 1;
            } else {
                for r in 0..DIM {
                    if i == MID - 1 {
                        counts[(0, r, l - 1)] += 1; // top row
                    } else if i == MID + 1 {
                        counts[(DIM - 1, r, l - 1)] += 1; // bot row
                    } else if j == MID - 1 {
                        counts[(r, 0, l - 1)] += 1; // left col
                    } else if j == MID + 1 {
                        counts[(r, DIM - 1, l - 1)] += 1; // right col
                    } else {
                        unreachable!();
                    }
                }
            }
        }
    }

    for (&index, &c) in counts.iter() {
        if c == 1 || (c == 2 && !bugs.contains(&index)) {
            bugs.insert(index);
        } else {
            bugs.remove(&index);
        }
    }

    bugs
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let bugs = parse_input(&read_input("day24")?)?;

    let answer = evolve_until_repeats(bugs.clone());
    println!("part A: {:?}", answer);

    let mut bugs = map(|(i, j)| (i, j, 0), bugs).collect();
    for _ in 0..200 {
        bugs = evolve_recur(bugs);
    }
    println!("part B: {:?}", bugs.len());

    Ok(())
}
