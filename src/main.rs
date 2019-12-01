#[macro_use]
mod common;
mod day01;

use std::env;

fn main() {
    let funs = [
        day01::run
    ];

    let args: Vec<_> = env::args().collect();
    let args: Vec<_> = args.iter().map(String::as_ref).collect();
    let binary = args.get(0).unwrap_or(&"");
    let day = args.get(1).unwrap_or(&"");
    let rest = args.get(2..).unwrap_or(&[]);

    match day.parse::<usize>() {
        Ok(x) if x > 0 && x <= funs.len() => {
            if let Err(x) = (funs[x - 1])(&rest) {
                println!("Error occurred:\n{:?}", x);
            }
        },
        _ => eprintln!("usage: {} [day]", binary),
    };
}
