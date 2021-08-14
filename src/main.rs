mod board;
mod clues;

use board::*;
use clues::*;

fn solve(clues: &Clues, board: &mut Board, r: usize, c: usize) {
    let (nr, nc) = clues.dims();
    if r >= nr {
        let board_clues = board.gen_clues();
        if &board_clues == clues {
            println!("{}", board);
        }
        return;
    }

    let (mut next_r, mut next_c) = (r, c);
    next_c += 1;
    if next_c >= nc {
        next_r += 1;
        next_c = 0;
    }

    board.set(r, c, false);
    solve(clues, board, next_r, next_c);
    board.set(r, c, true);
    solve(clues, board, next_r, next_c);
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let desc = std::fs::read_to_string(path).unwrap();
    let clues = Clues::parse(&desc).unwrap();
    let (nrows, ncols) = clues.dims();
    let mut board = Board::new(nrows, ncols);

    solve(&clues, &mut board, 0, 0);
}
