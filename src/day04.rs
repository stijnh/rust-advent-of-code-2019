use crate::common::*;
use itertools::Itertools;

fn is_valid_password(code: i64, exact: bool) -> bool {
    let buffer = format!("{}", code).into_bytes();

    for (a, b) in buffer.iter().zip(&buffer[1..]) {
        if a > b {
            return false;
        }
    }

    for (_, group) in &buffer.iter().group_by(|&x| x) {
        if iff!(exact, group.count() == 2, group.count() >= 2) {
            return true;
        }
    }

    false
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let range = read_input("day04")?[0]
        .split('-')
        .map(|x| x.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let count = (range[0]..=range[1])
        .filter(|&p| is_valid_password(p, false))
        .count();

    println!("answer A: {}", count);

    let count = (range[0]..=range[1])
        .filter(|&p| is_valid_password(p, true))
        .count();

    println!("answer B: {}", count);

    Ok(())
}
