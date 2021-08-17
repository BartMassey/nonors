mod board;
mod clues;
mod brute;

use std::path::PathBuf;
use structopt::StructOpt;

pub use board::*;
pub use clues::*;

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
    let solved = brute::solve_brute(&clues, &mut board, 0, 0, opt.find_all);
    std::process::exit((!solved) as i32);
}
