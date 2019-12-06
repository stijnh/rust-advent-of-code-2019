use crate::common::*;

const OP_ADD: i64 = 1;
const OP_MUL: i64 = 2;
const OP_INPUT: i64 = 3;
const OP_OUTPUT: i64 = 4;
const OP_BT: i64 = 5; // branch true
const OP_BF: i64 = 6; // branch false
const OP_LT: i64 = 7;
const OP_EQ: i64 = 8;
const OP_HALT: i64 = 99;

#[derive(Error, Debug)]
enum ExecError {
    #[error("index out of bounds: {0}")]
    InvalidIndex(i64),

    #[error("invalid opcode: {0}")]
    InvalidOpcode(i64),
}

fn run_program(program: &mut [i64], mut inputs: &[i64]) -> Result<Vec<i64>, ExecError> {
    use ExecError::*;

    fn next(program: &[i64], index: &mut i64) -> Result<i64, ExecError> {
        let result = get(program, *index)?;
        *index += 1;
        Ok(result)
    }

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
        let instr = next(program, &mut index)?;
        let opcode = instr % 100;
        let limm = (instr / 100) % 10 != 0;
        let rimm = (instr / 1000) % 10 != 0;

        if [OP_ADD, OP_MUL, OP_LT, OP_EQ].contains(&opcode) {
            let lhs = next(program, &mut index)?;
            let rhs = next(program, &mut index)?;
            let dst = next(program, &mut index)?;

            let a = iff!(limm, lhs, get(program, lhs)?);
            let b = iff!(rimm, rhs, get(program, rhs)?);
            let c = match opcode {
                OP_ADD => a + b,
                OP_MUL => a * b,
                OP_LT => (a < b) as i64,
                OP_EQ => (a == b) as i64,
                _ => panic!("invalid opcode"),
            };

            set(program, dst, c)?;
        } else if opcode == OP_BT || opcode == OP_BF {
            let param = next(program, &mut index)?;
            let target = next(program, &mut index)?;

            let a = iff!(limm, param, get(program, param)?);
            let b = iff!(rimm, target, get(program, target)?);

            if (opcode == OP_BT && a != 0) || (opcode == OP_BF && a == 0) {
                index = b;
            }
        } else if opcode == OP_INPUT {
            let value = inputs[0];
            let dst = next(program, &mut index)?;
            set(program, dst, value)?;

            inputs = &inputs[1..];
        } else if opcode == OP_OUTPUT {
            let src = next(program, &mut index)?;
            let value = get(program, src)?;
            output.push(value);
        } else if opcode == OP_HALT {
            break Ok(output);
        } else {
            break Err(InvalidOpcode(opcode));
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
