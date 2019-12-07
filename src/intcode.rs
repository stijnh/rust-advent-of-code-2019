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
pub(crate) enum ExecError {
    #[error("index out of bounds: {0}")]
    InvalidIndex(i64),

    #[error("invalid opcode: {0}")]
    InvalidOpcode(i64),

    #[error("insufficient number of inputs provided")]
    InputExhausted,
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ExecState {
    Halted,
    Input,
    Output(i64),
}

pub(crate) fn parse_program(filename: &str) -> Result<Program> {
    let program = read_input(filename)?[0]
        .split(",")
        .map(|s| s.parse::<i64>())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Program::new(program))
}

#[derive(Debug, Clone)]
pub(crate) struct Program {
    data: Vec<i64>,
    index: i64,
}

impl Program {
    pub(crate) fn new(data: Vec<i64>) -> Self {
        Self { 
            data,
            index: 0 
        }
    }

    #[inline(always)]
    fn next(&mut self) -> Result<i64, ExecError> {
        let result = self.get(self.index)?;
        self.index += 1;
        Ok(result)
    }

    #[inline(always)]
    fn get(&self, index: i64) -> Result<i64, ExecError> {
        self.data
            .get(index as usize)
            .copied()
            .ok_or(ExecError::InvalidIndex(index))
    }

    #[inline(always)]
    fn set(&mut self, index: i64, value: i64) -> Result<(), ExecError> {
        self.data
            .get_mut(index as usize)
            .map(|p| *p = value)
            .ok_or(ExecError::InvalidIndex(index))
    }

    pub(crate) fn run(&mut self, inputs: &[i64]) -> Result<Vec<i64>, ExecError> {
        let mut output = vec![];
        let mut inputs = inputs.into_iter().copied();

        loop {
            match self.resume(&mut inputs)? {
                ExecState::Halted => {
                    return Ok(output);
                },
                ExecState::Output(v) => {
                    output.push(v);
                },
                ExecState::Input => {
                    return Err(ExecError::InputExhausted);
                }
            }
        }
    }

    pub(crate) fn resume<I: IntoIterator<Item=i64>>(&mut self, inputs: I) -> Result<ExecState, ExecError> {
        self.resume_internal(&mut inputs.into_iter())
    }

    fn resume_internal(&mut self, inputs: &mut dyn Iterator<Item=i64>) -> Result<ExecState, ExecError> {
        use ExecError::*;

        loop {
            let instr = self.next()?;
            let opcode = instr % 100;
            let limm = (instr / 100) % 10 != 0;
            let rimm = (instr / 1000) % 10 != 0;

            if [OP_ADD, OP_MUL, OP_LT, OP_EQ].contains(&opcode) {
                let lhs = self.next()?;
                let rhs = self.next()?;
                let dst = self.next()?;

                let a = iff!(limm, lhs, self.get(lhs)?);
                let b = iff!(rimm, rhs, self.get(rhs)?);
                let c = match opcode {
                    OP_ADD => a + b,
                    OP_MUL => a * b,
                    OP_LT => (a < b) as i64,
                    OP_EQ => (a == b) as i64,
                    _ => panic!("invalid opcode"),
                };

                self.set(dst, c)?;
            } else if opcode == OP_BT || opcode == OP_BF {
                let param = self.next()?;
                let target = self.next()?;

                let a = iff!(limm, param, self.get(param)?);
                let b = iff!(rimm, target, self.get(target)?);

                if (opcode == OP_BT && a != 0) || (opcode == OP_BF && a == 0) {
                    self.index = b;
                }
            } else if opcode == OP_INPUT {
                if let Some(value) = inputs.next() {
                    let dst = self.next()?;
                    self.set(dst, value)?;
                } else {
                    self.index -= 1;
                    break Ok(ExecState::Input);
                }
            } else if opcode == OP_OUTPUT {
                let src = self.next()?;
                let value = self.get(src)?;
                break Ok(ExecState::Output(value));
            } else if opcode == OP_HALT {
                break Ok(ExecState::Halted);
            } else {
                break Err(InvalidOpcode(opcode));
            }
        }
    }
}
