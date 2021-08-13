#![allow(unused)]

use std::convert::TryInto;

type Line = u64;

#[derive(Debug, Clone)]
pub struct Board {
    rows: Vec<Line>,
    cols: Vec<Line>,
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

    pub fn set(&mut self, r: usize, c: usize, v: bool) {
        self.rows[r] |= (v as Line) << c;
        self.cols[c] |= (v as Line) << r;
    }

    pub fn get(&mut self, r: usize, c: usize) -> bool {
        self.rows[r] & (1 << c) == 1
    }

}
