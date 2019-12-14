use crate::common::*;
use std::collections::HashMap;

#[derive(Clone, Default, Debug)]
struct Reaction {
    input: Vec<(String, i64)>,
    output: (String, i64),
}

fn parse_input() -> Result<Vec<Reaction>> {
    let mut reactions = vec![];

    fn parse_element(line: &str) -> Result<(String, i64)> {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        let count = parts[0].parse()?;
        let name = parts[1].to_string();

        Ok((name, count))
    }

    for line in read_input("day14")? {
        let parts = line.splitn(2, " => ").collect::<Vec<_>>();
        let mut reaction: Reaction = default();

        for input in parts[0].split(",") {
            reaction.input.push(parse_element(input)?);
        }

        reaction.output = parse_element(parts[1])?;
        reactions.push(reaction);
    }

    Ok(reactions)
}

fn toposort(reactions: &mut [Reaction]) {
    let n = reactions.len();

    for i in 0..n {
        let mut count = HashMap::<_, usize>::new();
        let mut j = None;

        for r in &reactions[i..] {
            for input in &r.input {
                *count.entry(&input.0).or_default() += 1;
            }
        }

        for (k, r) in enumerate(&reactions[i..]) {
            if count.get(&r.output.0).copied().unwrap_or(0) == 0 {
                j = Some(i + k);
            }
        }

        reactions.swap(i, j.unwrap());
    }
}

fn ceil_div(a: i64, b: i64) -> i64 {
    (a / b) + iff!(a % b != 0, 1, 0)
}

fn find_ore_for_fuel(reactions: &[Reaction], fuel: i64) -> i64 {
    let mut pending = HashMap::new();
    pending.insert("FUEL", fuel);

    for r in reactions {
        let k = *pending.get(&*r.output.0).unwrap_or(&0);
        let n = ceil_div(k, r.output.1);

        if n > 0 {
            for input in &r.input {
                *pending.entry(&input.0).or_default() += input.1 * n;
            }
        }
    }

    pending["ORE"]
}

fn find_fuel_for_ore(reactions: &[Reaction], ore: i64) -> i64 {
    let mut lbnd = 0i64;
    let mut ubnd = 1_000_000_000i64;

    while ubnd - lbnd > 1 {
        let mid = (lbnd + ubnd) / 2;
        let found = find_ore_for_fuel(reactions, mid);

        if found <= ore {
            lbnd = mid;
        } else {
            ubnd = mid;
        }
    }

    lbnd
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let mut reactions = parse_input()?;
    toposort(&mut reactions);

    println!("answer A: {}", find_ore_for_fuel(&reactions, 1));
    println!("answer B: {}", find_fuel_for_ore(&reactions, 1000000000000));

    Ok(())
}
