use crate::common::*;

#[derive(Debug, Clone, Copy)]
enum Technique {
    Reverse,
    Cut(i128),
    Deal(i128),
}

use Technique::*;

fn parse_techniques() -> Result<Vec<Technique>> {
    let mut output = vec![];

    for line in read_input("day22")? {
        let t = if line == "deal into new stack" {
            Reverse
        } else if line.starts_with("cut") {
            let n = line[3..].trim().parse()?;
            Cut(n)
        } else if line.starts_with("deal with increment") {
            let n = line[19..].trim().parse()?;
            Deal(n)
        } else {
            bail!("invalid input line: {}", line);
        };

        output.push(t);
    }

    Ok(output)
}

fn apply_technique(t: Technique, index: i128, num: i128) -> i128 {
    assert!(index >= 0 && index < num);

    match t {
        Reverse => num - 1 - index,
        Deal(n) => (n * index) % num,
        Cut(n) => (index - n + num) % num,
    }
}

// Euclidean extended algorithm. From Wikipedia
fn mult_inverse(a: i128, n: i128) -> i128 {
    let mut t = 0;
    let mut r = n;
    let mut newt = 1;
    let mut newr = a;

    while newr != 0 {
        let q = r / newr;

        let oldt = t;
        t = newt;
        newt = oldt - q * newt;

        let oldr = r;
        r = newr;
        newr = oldr - q * newr
    }

    while t < 0 {
        t += n;
    }
    t
}

// applies "factor * x + constant" given "repeat" times
fn repeated_apply(factor: i128, constant: i128, repeat: i128, n: i128) -> (i128, i128) {
    // repeat == 0 so result is "1 * x + 0"
    if repeat == 0 {
        (1, 0)
    }
    // repeat == 2k so result "f (f x + c) + c" --> "f * f * x + f * c + c"
    else if repeat % 2 == 0 {
        let (f, c) = repeated_apply(factor, constant, repeat / 2, n);

        ((f * f).rem_euclid(n), (f * c + c).rem_euclid(n))
    }
    // repeat == 2k + 1
    else {
        let (f, c) = repeated_apply(factor, constant, repeat - 1, n);

        (
            (factor * f).rem_euclid(n),
            (factor * c + constant).rem_euclid(n),
        )
    }
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let input = parse_techniques()?;

    let n = 10007;
    let mut index = 2019;
    for &t in &input {
        index = apply_technique(t, index, n);
    }
    println!("answer A: {:?}", index);

    let n = 119_315_717_514_047;
    let repeats = 101_741_582_076_661;

    let (mut a, mut b) = (0, 1);
    for &t in &input {
        a = apply_technique(t, a, n);
        b = apply_technique(t, b, n);
    }

    // since "a == factor * 0 + constant" and "b == factor * 1 + constant"
    // we know that
    //   constant = a
    //   factor = b - constant = b - a
    let constant = a;
    let factor = (b - a).rem_euclid(n);
    println!("equation: {} x + {}", factor, constant);

    let inv_factor = mult_inverse(factor, n);
    let inv_constant = (-inv_factor * constant).rem_euclid(n);
    println!("inverse equation: {} x + {}", inv_factor, inv_constant);

    let (rep_factor, rep_constant) = repeated_apply(inv_factor, inv_constant, repeats, n);
    println!(
        "repeated inverse equation: {} x + {}",
        inv_factor, inv_constant
    );

    let index = 2020i128;
    let result = (rep_factor * index + rep_constant).rem_euclid(n);

    println!("answer B: {:?}", result);

    Ok(())
}
