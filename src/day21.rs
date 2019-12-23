use crate::common::*;
use crate::intcode::*;
use itertools::Itertools;

fn launch_springdroid(mut program: Program, code: &str) -> Result<Option<i64>> {
    let input = code
        .lines()
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .interleave_shortest(cycle("\n"))
        .collect::<String>();

    print!("CODE: {}", input);

    let mut iter = input.chars().map(|c| c as i64);

    while let ExecState::Output(c) = program.resume(&mut iter)? {
        if c < 128 {
            print!("{}", c as u8 as char);
        } else {
            return Ok(Some(c));
        }
    }

    Ok(None)
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let program = parse_program("day21")?;

    // !(A && B && C) && D
    // Jump if D is true and any of [A, B, C] is false
    let code = "
        OR A T
        AND B T
        AND C T
        NOT T J
        AND D J
        WALK
    ";

    let answer = launch_springdroid(program.clone(), code)?;
    println!("answer A: {:?}", answer);

    // !(A && B && C) && (E || H) && D
    // Jump if D is true, any of [A, B, C] is false, E or H is true
    let code = "
        OR A T
        AND B T
        AND C T
        NOT T T

        OR E J
        OR H J

        AND T J
        AND D J
        RUN
    ";
    let answer = launch_springdroid(program, code)?;
    println!("answer B: {:?}", answer);

    Ok(())
}
