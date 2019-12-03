#[macro_use]
mod common;
mod day01;
mod day02;
mod day03;

use std::env;

fn main() {
    let funs = [day01::run, day02::run, day03::run];

    let args: Vec<_> = env::args().collect();
    let args: Vec<_> = args.iter().map(String::as_ref).collect();
    let binary = args.get(0).unwrap_or(&"");
    let day = args.get(1).unwrap_or(&"");
    let rest = args.get(2..).unwrap_or(&[]);

    if let Ok(x) = day.parse::<usize>() {
        if x > 0 && x <= funs.len() {
            if let Err(msg) = (funs[x - 1])(&rest) {
                eprintln!("error occurred: {:?}", msg);
            }
        } else {
            eprintln!("day must be between 1 and {}", funs.len() + 1);
        }
    } else {
        eprintln!("usage: {} [day]", binary);
    }
}
