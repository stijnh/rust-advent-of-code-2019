use crate::common::*;
use crate::intcode::*;
use ndarray::prelude::*;
use std::thread::sleep;
use std::cmp::Ordering;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum Tile {
    Unknown,
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}

use Tile::*;

fn run_timestep(input: i64, program: &mut Program, board: &mut Array2<Tile>, score: &mut i64) -> Result {
    let mut input = Some(input).into_iter();
    let mut outputs = vec![];

    loop {
        match program.resume(&mut input)? {
            ExecState::Output(x) => outputs.push(x),
            _ => break,
        }
    };

    for slice in outputs.chunks_exact(3) {
        let (x, y, id) = (slice[0], slice[1], slice[2]);

        if x >= 0 && y >= 0 {
            let tile = match id {
                0 => Empty,
                1 => Wall,
                2 => Block,
                3 => Paddle,
                4 => Ball,
                _ => Unknown,
            };

            board[(x as usize, y as usize)] = tile;
        } else {
            *score = id;
        }
    }

    Ok(())
}

fn count_block_tiles(board: &Array2<Tile>) -> usize {
    board.iter().filter(|&&v| v == Block).count()
}

fn find_tile(board: &Array2<Tile>, tile: Tile) -> Option<(usize, usize)> {
    board
        .indexed_iter()
        .filter(move |(_, &t)| t == tile)
        .map(|(idx, _)| idx)
        .next()
}

fn print_board(board: &Array2<Tile>) {
    for vec in board.gencolumns() {
        for val in vec {
            let c = match val {
                Block => '=',
                Wall => '#',
                Paddle => '-',
                Ball => 'O',
                _ => ' ',
            };

            print!("{}", c);
        }
        println!();
    }
}

pub(crate) fn run(args: &[&str]) -> Result {
    let mut program = parse_program("day13")?;
    let mut board = Array2::from_elem((50, 25), Unknown);
    let mut score = 0;

    // insert quarter
    program.set(0, 2)?;

    // Run game once and count number of blocks
    run_timestep(0, &mut program, &mut board, &mut score)?;
    println!("answer A: {}", count_block_tiles(&board));

    // Run game until all blocks are gone
    while count_block_tiles(&board) > 0 {

        // Print visualization if argument given
        if !args.is_empty() {
            print_board(&board);
            sleep(Duration::from_millis(50));
        }

        // Get ball and paddle location
        let ball_x = find_tile(&board, Ball).unwrap_or_default().0;
        let paddle_x = find_tile(&board, Paddle).unwrap_or_default().0;

        // Move paddle in direction of ball
        let input = match Ord::cmp(&paddle_x, &ball_x) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        };

        run_timestep(input, &mut program, &mut board, &mut score)?;
    }

    println!("answer B: {}", score);

    Ok(())
}
