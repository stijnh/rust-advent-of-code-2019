use crate::common::*;
use crate::intcode::*;

pub(crate) fn run(_args: &[&str]) -> Result {
    let mut program = parse_program("day09")?;

    let output = program.run(&[1])?;
    println!("answer A: {:?}", output);

    let output = program.run(&[2])?;
    println!("answer B: {:?}", output);

    Ok(())
}
