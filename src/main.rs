mod board;
mod clues;

use std::path::PathBuf;
use structopt::StructOpt;

use board::*;
use clues::*;

fn solve_brute(
    clues: &Clues,
    board: &mut Board,
    r: usize,
    c: usize,
    find_all: bool,
) -> bool {
    let (nr, nc) = clues.dims();
    if r >= nr {
        if board.solved(clues) {
            println!("{}", board);
            return true;
        }
        return false;
    }

    let (mut next_r, mut next_c) = (r, c);
    next_c += 1;
    if next_c >= nc {
        next_r += 1;
        next_c = 0;
    }

    board.set(r, c, false);
    let solved = solve_brute(clues, board, next_r, next_c, find_all);
    if !find_all && solved {
        return true;
    }
    board.set(r, c, true);
    let solved = solved | solve_brute(clues, board, next_r, next_c, find_all);
    solved
}

#[derive(Debug, StructOpt)]
#[structopt(name = "nonors", about = "Nonogram solver.")]
struct Opt {
    #[structopt(short = "a", long = "all")]
    find_all: bool,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let desc = std::fs::read_to_string(opt.path).unwrap();
    let clues = Clues::parse(&desc).unwrap();
    let (nrows, ncols) = clues.dims();
    let mut board = Board::new(nrows, ncols);
    let solved = solve_brute(&clues, &mut board, 0, 0, opt.find_all);
    std::process::exit((!solved) as i32);
}
