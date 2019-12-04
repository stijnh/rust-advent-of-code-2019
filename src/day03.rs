use crate::common::*;

#[derive(Copy, Clone, Debug)]
enum Line {
    Horz(i64, i64, i64), // x, y, length along x axis
    Vert(i64, i64, i64), // x, y, length along y axis
}

impl Line {
    fn length(&self) -> i64 {
        match *self {
            Line::Horz(.., length) => length.abs(),
            Line::Vert(.., length) => length.abs(),
        }
    }

    fn distance_to(&self, x: i64, y: i64) -> i64 {
        match *self {
            Line::Horz(x0, y0, _) => (x - x0).abs() + (y - y0).abs(),
            Line::Vert(x0, y0, _) => (x - x0).abs() + (y - y0).abs(),
        }
    }
}

fn parse_wire(string: &str) -> Result<Vec<Line>> {
    let mut wire = vec![];
    let (mut x, mut y) = (0, 0);

    for part in string.split(",") {
        let letter = &part[..1];
        let number = part[1..].parse::<i64>()?;

        if letter == "R" {
            wire.push(Line::Horz(x, y, number));
            x += number;
        } else if letter == "L" {
            wire.push(Line::Horz(x, y, -number));
            x -= number;
        } else if letter == "U" {
            wire.push(Line::Vert(x, y, number));
            y += number;
        } else if letter == "D" {
            wire.push(Line::Vert(x, y, -number));
            y -= number;
        } else {
            println!("warning: unknown direction {}", letter);
        }
    }

    Ok(wire)
}

fn calculate_intersection(p: Line, q: Line) -> Option<(i64, i64)> {
    match (p, q) {
        (Line::Horz(x0, y, h), Line::Vert(x, y0, v)) => {
            let (x0, x1) = iff!(h > 0, (x0, x0 + h), (x0 + h, x0));
            let (y0, y1) = iff!(v > 0, (y0, y0 + v), (y0 + v, y0));

            if x >= x0 && x <= x1 && y >= y0 && y <= y1 {
                Some((x, y))
            } else {
                None
            }
        }
        (Line::Vert(..), Line::Horz(..)) => calculate_intersection(q, p),
        _ => None,
    }
}

pub fn run(_args: &[&str]) -> Result {
    let input = read_input("day03")?;
    let a = parse_wire(&input[0])?;
    let b = parse_wire(&input[1])?;

    let mut closest_dist = std::i64::MAX;
    let mut fastest_time = std::i64::MAX;

    let mut time_a = 0;
    for &p in &a {
        let mut time_b = 0;

        for &q in &b {
            if let Some((x, y)) = calculate_intersection(p, q) {
                let dist = x.abs() + y.abs();
                closest_dist = i64::min(dist, closest_dist);

                let time = time_a + time_b + p.distance_to(x, y) + q.distance_to(x, y);
                fastest_time = i64::min(time, fastest_time);
            }

            time_b += q.length();
        }

        time_a += p.length();
    }

    println!("answer A: {}", closest_dist);
    println!("answer B: {}", fastest_time);

    Ok(())
}
