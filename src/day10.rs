use crate::common::*;
use std::collections::{HashSet, BTreeMap, BinaryHeap};
use float_ord::FloatOrd;

fn gcd(a: isize, b: isize) -> isize {
    iff!(b == 0, a, gcd(b, a % b))
}

fn count_detected_astroids(x: isize, y: isize, astroids: &[(isize, isize)]) -> usize {
    let mut angles = HashSet::new();

    for &(ax, ay) in astroids {
        if (ax, ay) != (x, y) {
            let dx = ax - x;
            let dy = ay - y;
            let common = gcd(dx.abs(), dy.abs());

            angles.insert((dx / common, dy / common));
        }
    }

    angles.len()
}

fn order_astroids_by_laser(x: isize, y: isize, astroids: &[(isize, isize)]) -> Vec<(isize, isize)> {
    let mut groups = BTreeMap::<_, BinaryHeap<(isize, isize, isize)>>::new();

    for &(ax, ay) in astroids {
        if (ax, ay) != (x, y) {
            let (dx, dy) = (ax - x, ay - y);
            let dist = dx.abs() + dy.abs();

            let angle = f64::atan2(dx as f64, -dy as f64);
            let angle = iff!(angle >= 0.0, angle, angle + 2.0 * std::f64::consts::PI);

            groups
                .entry(FloatOrd(angle))
                .or_default()
                .push((-dist, ax, ay));
        }
    }

    let mut output = vec![];

    while output.len() + 1 < astroids.len() {
        for group in groups.values_mut() {
            if let Some((_, ax, ay)) = group.pop() {
                output.push((ax, ay));
            }
        }
    }

    output
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let mut astroids = vec![];

    for (y, line) in enumerate(read_input("day10")?) {
        for (x, c) in enumerate(line.chars()) {
            if c == '#' {
                astroids.push((x as isize, y as isize));
            }
        }
    }

    let (x, y, count) = astroids
        .iter()
        .map(|&(x, y)| (x, y, count_detected_astroids(x, y, &astroids)))
        .max_by_key(|&(_, _, c)| c)
        .unwrap();

    println!("answer A: {}", count);

    let ordered = order_astroids_by_laser(x, y, &astroids);
    println!("answer B: {:?}", ordered[199]);

    Ok(())
}
