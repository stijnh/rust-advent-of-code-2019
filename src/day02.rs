use crate::common::*;

#[derive(Error, Debug)]
enum ExecError {
    #[error("invalid opcode {0}")]
    UnknownOpcode(i64),

    #[error("index {0} is out of bounds")]
    InvalidIndex(i64),
}

#[inline(never)]
fn run_program(program: &mut [i64]) -> Result<(), ExecError> {
    use ExecError::*;
    let mut index = 0;

    #[inline(always)]
    fn get(program: &mut [i64], index: i64) -> Result<i64, ExecError> {
        program
            .get(index as usize)
            .copied()
            .ok_or(InvalidIndex(index))
    }

    #[inline(always)]
    fn set(program: &mut [i64], index: i64, value: i64) -> Result<(), ExecError> {
        program
            .get_mut(index as usize)
            .map(|p| *p = value)
            .ok_or(InvalidIndex(index))
    }

    loop {
        let opcode = get(program, index)?;

        if opcode == 1 || opcode == 2 {
            let lhs = get(program, index + 1)?;
            let rhs = get(program, index + 2)?;
            let dst = get(program, index + 3)?;

            let a = get(program, lhs)?;
            let b = get(program, rhs)?;
            let c = iff!(opcode == 1, a + b, a * b);
            set(program, dst, c)?;

            index += 4;
        } else if opcode == 99 {
            break Ok(());
        } else {
            break Err(ExecError::UnknownOpcode(opcode));
        }
    }
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let original = read_input("day02")?
        .remove(0)
        .split(",")
        .map(|s| s.parse::<i64>().context("invalid input file"))
        .collect::<Result<Vec<_>, _>>()?;

    let mut program = original.clone();
    program[1] = 12;
    program[2] = 2;
    run_program(&mut program)?;
    println!("answer A: {}", program[0]);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = original.clone();
            program[1] = noun;
            program[2] = verb;

            if run_program(&mut program).is_err() {
                continue;
            }

            if program[0] == 19690720 {
                println!("answer B: {}", 100 * noun + verb);
            }
        }
    }

    Ok(())
}
