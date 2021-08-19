use crate::*;

use std::iter;

fn gen_rows(r: Line, ir: usize, nr: usize, clue: &[usize]) -> Box<dyn Iterator<Item = Line>> {
    if clue.is_empty() {
        return Box::new(iter::once(r));
    }
    let blot = clue[0];
    let next_clue = &clue[1..];
    let space = (!next_clue.is_empty()) as usize;
    let mut result: Box<dyn Iterator<Item=Line>> = Box::new(iter::empty());
    for i in ir..=nr-blot-space {
        let next_r = r | (((1 << blot) - 1) << i);
        let next_ir = i + blot + space;
        result = Box::new(result.chain(gen_rows(next_r, next_ir, nr, next_clue)));
    }
    result
}

#[test]
fn test_gen_rows() {
    let clue = [1, 1, 1];
    let mut rows = [
        0b010101,
        0b100101,
        0b101001,
        0b101010,
    ];
    rows.sort();
    let mut gens: Vec<Line> = gen_rows(0, 0, 6, &clue).collect();
    gens.sort();
    assert_eq!(rows.to_vec(), gens);

    let clue = [4, 2];
    let rows = [
        0b1101111,
    ];
    let gens: Vec<Line> = gen_rows(0, 0, 7, &clue).collect();
    assert_eq!(rows.to_vec(), gens);
}

pub fn solve(
    clues: &Clues,
    board: &mut Board,
    r: usize,
    find_all: bool,
) -> bool {
    let (nr, nc) = clues.dims();

    if board.cols_inconsistent(clues) {
        return false;
    }

    if r >= nr {
        if board.solved(clues) {
            println!("{}", board);
            return true;
        }
        return false;
    }

    let mut solved = false;
    for row in gen_rows(0, 0, nc, &clues.rows[r]) {
        for c in 0..nc {
            board.set(r, c, (row & (1 << c)) != 0);
        }
        solved |= solve(clues, board, r + 1, find_all);
        if !find_all && solved {
            return true;
        }
    }
    for c in 0..nc {
        board.set(r, c, false);
    }
    solved
}
