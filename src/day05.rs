use crate::common::*;
use crate::intcode::*;

pub(crate) fn run(_args: &[&str]) -> Result {
    let original = parse_program("day05")?;

    let mut program = original.clone();
    let outputs = program.run(&[1])?;
    println!("answer A: {:?}", outputs);

    let mut program = original.clone();
    let outputs = program.run(&[5])?;
    println!("answer B: {:?}", outputs);

    Ok(())
}
