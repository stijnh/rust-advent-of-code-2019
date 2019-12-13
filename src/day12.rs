use crate::common::*;
use std::collections::HashMap;

fn gcd(a: i64, b: i64) -> i64 {
    match (a, b) {
        (a, 0) => a,
        (a, b) => gcd(b, a % b),
    }
}

fn lcd(a: i64, b: i64) -> i64 {
    (a * b) / gcd(a, b)
}

fn iterate_timestep(pos: &mut [[i64; 3]], vel: &mut [[i64; 3]]) {
    assert_eq!(pos.len(), vel.len());
    let n = pos.len();
    let mut gravity = vec![[0, 0, 0]; n];

    // calculate gravity
    for (i, p) in enumerate(&*pos) {
        for (j, q) in enumerate(&*pos) {
            if i != j {
                for k in 0..3 {
                    gravity[i][k] += (q[k] - p[k]).signum();
                }
            }
        }
    }

    // apply gravity
    for (v, g) in zip(&mut *vel, gravity) {
        *v = [v[0] + g[0], v[1] + g[1], v[2] + g[2]];
    }

    // apply velocity
    for (p, v) in zip(pos, vel) {
        *p = [p[0] + v[0], p[1] + v[1], p[2] + v[2]];
    }
}

fn calculate_energy(pos: &[[i64; 3]], vel: &[[i64; 3]]) -> i64 {
    let mut energy = 0;

    for (p, v) in zip(pos, vel) {
        energy += (p[0].abs() + p[1].abs() + p[2].abs()) * (v[0].abs() + v[1].abs() + v[2].abs());
    }

    energy
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let pattern = regex::Regex::new("<x=(-?[0-9]+), y=(-?[0-9]+), z=(-?[0-9]+)>")?;
    let mut pos = vec![];
    let mut vel = vec![];

    for line in read_input("day12")? {
        let m = pattern.captures(&line).unwrap();

        pos.push([
            m[1].parse::<i64>().unwrap(),
            m[2].parse::<i64>().unwrap(),
            m[3].parse::<i64>().unwrap(),
        ]);

        vel.push([0, 0, 0]);
    }

    {
        let (mut pos, mut vel) = (pos.clone(), vel.clone());
        for _ in 0..1000 {
            iterate_timestep(&mut pos, &mut vel);
        }
        println!("answer A: {}", calculate_energy(&pos, &vel));
    }

    // find the cycles length along each axis
    let mut cycle_length = [0; 3];
    for k in 0..3 {
        let mut states = HashMap::new();
        let (mut pos, mut vel) = (pos.clone(), vel.clone());

        for curr in 0.. {
            // state is vector of coordinate along k-th axis for each planet
            let state = zip(&pos, &vel)
                .map(|(p, v)| (p[k], v[k]))
                .collect::<Vec<_>>();

            // if state has already been seen, we have hit a cycle
            if let Some(prev) = states.insert(state, curr) {
                cycle_length[k] = curr - prev;
                println!("cycle along {}-th axis: {}", k, curr - prev);
                break;
            }

            iterate_timestep(&mut pos, &mut vel);
        }
    }

    let mut cycle = 1;
    cycle = lcd(cycle, cycle_length[0]);
    cycle = lcd(cycle, cycle_length[1]);
    cycle = lcd(cycle, cycle_length[2]);
    println!("answer B: {:?}", cycle);

    Ok(())
}
