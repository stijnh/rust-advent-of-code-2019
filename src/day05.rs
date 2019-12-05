use crate::common::*;

#[derive(Error, Debug)]
enum ExecError {
    #[error("index out of bounds: {0}")]
    InvalidIndex(i64),

    #[error("invalid opcode: {0}")]
    InvalidOpcode(i64),
}

fn run_program(program: &mut [i64], mut inputs: &[i64]) -> Result<Vec<i64>, ExecError> {
    use ExecError::*;

    fn get(program: &[i64], index: i64) -> Result<i64, ExecError> {
        program
            .get(index as usize)
            .copied()
            .ok_or(InvalidIndex(index))
    }

    fn set(program: &mut [i64], index: i64, value: i64) -> Result<(), ExecError> {
        program
            .get_mut(index as usize)
            .map(|p| *p = value)
            .ok_or(InvalidIndex(index))
    }

    let mut index = 0;
    let mut output = vec![];

    loop {
        let instr = get(program, index)?;
        let opcode = instr % 100;

        if opcode == 1 || opcode == 2 || opcode == 7 || opcode == 8 {
            let lhs = get(program, index + 1)?;
            let rhs = get(program, index + 2)?;
            let dst = get(program, index + 3)?;

            let limm = (instr / 100) % 10 != 0;
            let rimm = (instr / 1000) % 10 != 0;

            let a = iff!(limm, lhs, get(program, lhs)?);
            let b = iff!(rimm, rhs, get(program, rhs)?);
            let c = match opcode {
                1 => a + b,
                2 => a * b,
                7 => iff!(a < b, 1, 0),
                8 => iff!(a == b, 1, 0),
                _ => panic!("invalid opcode"),
            };

            set(program, dst, c)?;
            index += 4;

        } else if opcode == 5 || opcode == 6 {
            let param = get(program, index + 1)?;
            let target = get(program, index + 2)?;

            let pimm = (instr / 100) % 10 != 0;
            let timm = (instr / 1000) % 10 != 0;

            let a = iff!(pimm, param, get(program, param)?);
            let b = iff!(timm, target, get(program, target)?);

            if (opcode == 5 && a != 0) || (opcode == 6 && a == 0) {
                index = b;
            } else {
                index += 3;
            }

        } else if opcode == 3 {
            let value = inputs[0];
            let dst = get(program, index + 1)?;
            set(program, dst, value)?;

            index += 2;
            inputs = &inputs[1..];

        } else if opcode == 4 {
            let src = get(program, index + 1)?;
            let value = get(program, src)?;
            output.push(value);

            index += 2;

        } else if opcode == 99 {
            break Ok(output)

        } else {
            break Err(InvalidOpcode(opcode))
        }
    }
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let original = read_input("day05")?[0]
        .split(",")
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    let mut program = original.clone();
    let outputs = run_program(&mut program, &[1])?;
    println!("answer A: {:?}", outputs);

    let mut program = original.clone();
    let outputs = run_program(&mut program, &[5])?;
    println!("answer B: {:?}", outputs);

    Ok(())
}
