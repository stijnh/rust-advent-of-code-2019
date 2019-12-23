#[macro_use]
mod common;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;
mod day23;
mod intcode;

use std::env;

fn main() {
    let funs = [
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
        day07::run,
        day08::run,
        day09::run,
        day10::run,
        day11::run,
        day12::run,
        day13::run,
        day14::run,
        day15::run,
        day16::run,
        day17::run,
        day18::run,
        day19::run,
        day20::run,
        day21::run,
        day22::run,
        day23::run,
    ];

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
