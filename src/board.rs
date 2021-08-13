#![allow(unused)]

use std::convert::TryInto;

type Line = u64;

#[derive(Debug, Clone)]
pub struct Board {
    rows: Vec<Line>,
    cols: Vec<Line>,
}

#[derive(Debug)]
pub struct ParseError;

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

    pub fn parse(desc: &str) -> Result<Self, ParseError> {
        let mut width = None;
        let mut height = None;
        for line in desc.split('\n').map(str::trim) {
            let words: Vec<&str> = line.split_whitespace().collect();
            if words.len() == 0 {
                continue;
            }

            fn set_field(field: &mut Option<usize>, word: &str) -> Result<(), ParseError> {
                if field.is_some() {
                    return Err(ParseError);
                }
                let value = word.parse().map_err(|_| ParseError)?;
                *field = Some(value);
                Ok(())
            }
            
            match words[0] {
                "width" => {
                    set_field(&mut width, words[1])?;
                }
                "height" => {
                    set_field(&mut height, words[1])?;
                }
                _ => (),
            }
        }

        match (width, height) {
            (Some(w), Some(h)) => Ok(Self::new(h, w)),
            _ => Err(ParseError),
        }
    }
}
