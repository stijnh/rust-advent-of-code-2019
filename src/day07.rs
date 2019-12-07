use crate::common::*;
use crate::intcode::*;

fn permutations(n: usize) -> Vec<Vec<usize>> {
    if n == 1 {
        return vec![vec![0]];
    }

    let mut output = vec![];
    let others = permutations(n - 1);
    for i in 0..n {
        for rest in &others {
            let mut p = vec![i];

            for &v in rest {
                p.push(iff!(v >= i, v + 1, v));
            }

            output.push(p);
        }
    }

    output
}

pub(crate) fn run(_args: &[&str]) -> Result {
    let original = parse_program("day07")?;
    let mut max_value = 0;

    for order in permutations(5) {
        let mut value = 0;

        for i in order {
            let output = original.clone().run(&[i as _, value])?;
            value = output[0];
        }

        max_value = i64::max(value, max_value);
    }

    println!("answer A: {}", max_value);

    max_value = 0;

    for order in permutations(5) {
        let mut programs = vec![];
        let mut value = 0;

        for i in order {
            let mut program = original.clone();
            program.resume(Some(i as i64 + 5))?;
            programs.push(program);
        }

        let mut halted = false;
        while !halted {
            for program in &mut programs {
                match program.resume(Some(value))? {
                    ExecState::Output(v) => value = v,
                    ExecState::Input => panic!("need input?"),
                    ExecState::Halted => halted = true,
                }
            }
        }

        max_value = i64::max(value, max_value);
    }

    println!("answer B: {}", max_value);

    Ok(())
}
