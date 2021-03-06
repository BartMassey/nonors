use crate::*;

pub fn solve(
    clues: &Clues,
    board: &mut Board,
    r: usize,
    c: usize,
    find_all: bool,
) -> bool {
    let (nr, nc) = clues.dims();

    if r > 0 && c == 0 {
        if clues.rows[r - 1] != board.gen_row_clue(r - 1) {
            return false;
        }
        if board.cols_inconsistent(clues) {
            return false;
        }
    }

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
    let solved = solve(clues, board, next_r, next_c, find_all);
    if !find_all && solved {
        return true;
    }
    board.set(r, c, true);
    let solved = solve(clues, board, next_r, next_c, find_all);
    if !find_all && solved {
        return true;
    }
    board.set(r, c, false);
    solved
}
