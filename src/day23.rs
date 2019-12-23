use crate::common::*;
use crate::intcode::*;
use std::mem::take;

pub(crate) fn run(_args: &[&str]) -> Result {
    let program = parse_program("day23")?;

    let mut computers = vec![];
    let mut queues = vec![];

    for i in 0..50 {
        let mut computer = program.clone();
        computer.resume(Some(i))?;

        computers.push(computer);
        queues.push(vec![]);
    }

    let mut nat = (0, 0);
    let mut history = vec![];
    let mut done = false;

    while !done {
        for i in 0..50 {
            queues[i].push(-1);

            let c = &mut computers[i];
            let mut input = take(&mut queues[i]).into_iter();
            let mut output = vec![];

            while let ExecState::Output(c) = c.resume(&mut input)? {
                output.push(c);
            }

            for chunk in output.chunks_exact(3) {
                let (addr, x, y) = (chunk[0], chunk[1], chunk[2]);

                if let Some(q) = queues.get_mut(addr as usize) {
                    q.push(x);
                    q.push(y);
                } else {
                    nat = (x, y);
                }
            }
        }

        if all(&queues, Vec::is_empty) {
            done = any(&history, |&p: &(i64, i64)| p.1 == nat.1);

            history.push(nat);
            queues[0].push(nat.0);
            queues[0].push(nat.1);
        }
    }

    println!("answer A: {:?}", history.first());
    println!("answer B: {:?}", history.last());

    Ok(())
}
