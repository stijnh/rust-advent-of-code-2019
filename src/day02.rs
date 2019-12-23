use crate::common::*;
use crate::intcode::*;

pub(crate) fn run(_args: &[&str]) -> Result {
    let original = parse_program("day02")?;

    let mut program = original.clone();
    program.set(1, 12)?;
    program.set(2, 2)?;
    program.run(&[])?;
    println!("answer A: {}", program.get(0)?);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = original.clone();
            program.set(1, noun)?;
            program.set(2, verb)?;

            if program.run(&[]).is_err() {
                continue;
            }

            if program.get(0)? == 19_690_720 {
                println!("answer B: {}", 100 * noun + verb);
            }
        }
    }

    Ok(())
}
