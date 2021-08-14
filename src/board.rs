use std::convert::TryInto;
use std::fmt;

use crate::*;

type Line = u64;

#[derive(Debug, Clone)]
pub struct Board {
    rows: Vec<Line>,
    cols: Vec<Line>,
}

fn gen_clue(nline: usize, line: Line) -> Clue {
    let mut clue = Clue::new();
    let mut count = 0;
    for i in 0..nline {
        let mark = line & (1 << i) != 0;
        if mark {
            count += 1;
        } else if count > 0 {
            clue.push(count);
            count = 0;
        }
    }
    if count > 0 {
        clue.push(count);
    }
    clue
}

impl Board {
    pub fn new(nrows: usize, ncols: usize) -> Self {
        assert!(nrows <= Line::max_value().try_into().unwrap());
        assert!(ncols <= Line::max_value().try_into().unwrap());

        fn gen(n: usize) -> Vec<Line> {
            std::iter::repeat(0).take(n).collect()
        }

        Self {
            rows: gen(nrows),
            cols: gen(ncols),
        }
    }

    pub fn get(&self, r: usize, c: usize) -> bool {
        self.rows[r] & (1 << c) != 0
    }

    pub fn set(&mut self, r: usize, c: usize, v: bool) {
        let cmask = 1 << c;
        if v {
            self.rows[r] |= cmask;
        } else {
            self.rows[r] &= !cmask;
        }

        let rmask = 1 << r;
        if v {
            self.cols[c] |= rmask;
        } else {
            self.cols[c] &= !rmask;
        }
    }

    pub fn gen_clues(&self) -> Clues {
        let nrows = self.rows.len();
        let ncols = self.cols.len();
        let rows = self.rows
            .iter()
            .map(|&r| gen_clue(ncols, r))
            .collect();
        let cols = self.cols
            .iter()
            .map(|&c| gen_clue(nrows, c))
            .collect();
        Clues { rows, cols }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let nr = self.rows.len();
        let nc = self.cols.len();
        for r in 0..nr {
            for c in 0..nc {
                if c > 0 {
                    write!(f, " ")?;
                }
                if self.get(r, c) {
                    write!(f, "*")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
