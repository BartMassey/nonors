mod board;
mod clues;
mod solvers;

use std::path::PathBuf;
use std::process::exit;

use structopt::StructOpt;

pub use board::*;
pub use clues::*;
pub use solvers::*;

#[derive(Debug, StructOpt)]
#[structopt(name = "nonors", about = "Nonogram solver.")]
struct Opt {
    #[structopt(short = "a", long = "all")]
    find_all: bool,
    #[structopt(short = "s", long = "solver", default_value = "brute")]
    solver: String,
    #[structopt(parse(from_os_str))]
    path: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let desc = std::fs::read_to_string(opt.path).unwrap();
    let clues = Clues::parse(&desc).unwrap();
    let (nrows, ncols) = clues.dims();
    let mut board = Board::new(nrows, ncols);
    let solved = match opt.solver.as_str() {
        "brute" => brute::solve(&clues, &mut board, 0, 0, opt.find_all),
        "rows" => rows::solve(&clues, &mut board, 0, 0, opt.find_all),
        _ => {
            eprintln!("unknown strategy {}", opt.solver);
            exit(-1);
        }
    };
    exit((!solved) as i32);
}
