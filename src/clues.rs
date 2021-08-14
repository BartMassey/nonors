use std::str::FromStr;

pub type Clue = Vec<usize>;

#[derive(Debug)]
pub struct ParseError;

#[derive(Debug, PartialEq, Eq)]
pub struct Clues {
    pub rows: Vec<Clue>,
    pub cols: Vec<Clue>,
}

impl Clues {

    pub fn parse(desc: &str) -> Result<Self, ParseError> {
        let mut width: Option<usize> = None;
        let mut height: Option<usize> = None;
        let mut rows: Option<Vec<Clue>> = None;
        let mut cols: Option<Vec<Clue>> = None;

        let lines: Vec<_> = desc
            .split('\n')
            .map(|line| {
                line
                    .trim()
                    .split_whitespace()
                    .collect::<Vec<&str>>()
            })
            .collect();
        let nlines = lines.len();

        fn set_field<T>(field: &mut Option<T>, word: &str) -> Result<(), ParseError>
            where T: FromStr
        {
            if field.is_some() {
                return Err(ParseError);
            }
            let value: T = word.parse().map_err(|_| ParseError)?;
            *field = Some(value);
            Ok(())
        }

        fn parse_clue(nclues: Option<usize>, lines: &[Vec<&str>]) -> Result<(usize, Vec<Clue>), ParseError>
        {
            let ncs = if let Some(nclues) = nclues {
                nclues
            } else {
                return Err(ParseError);
            };

            let mut cs = vec![];
            for line in &lines[..ncs] {
                match line.len() {
                    0 => cs.push(vec![]),
                    1 => {
                        let clues = line[0]
                            .split(',')
                            .map(|clue| {
                                clue.parse().map_err(|_| ParseError)
                            })
                            .collect::<Result<_, _>>()?;
                        cs.push(clues);
                    }
                    _ => return Err(ParseError),
                }
            }
            Ok((ncs, cs))
        }

        let mut pos = 0;
        while pos < nlines {
            
            let words = &lines[pos];
            if words.is_empty() {
                pos += 1;
                continue;
            }

            match words[0] {
                "width" => {
                    set_field(&mut width, words[1])?;
                }
                "height" => {
                    set_field(&mut height, words[1])?;
                }
                "rows" => {
                    pos += 1;
                    let (nrs, rs) = parse_clue(height, &lines[pos..])?;
                    pos += nrs;
                    rows = Some(rs);
                    continue;
                }
                "columns" => {
                    pos += 1;
                    let (ncs, cs) = parse_clue(width, &lines[pos..])?;
                    pos += ncs;
                    cols = Some(cs);
                    continue;
                }
                _ => (),
            }

            pos += 1;
        }

        match (rows, cols) {
            (Some(rows), Some(cols)) => Ok(Self { rows, cols }),
            _ => Err(ParseError),
        }
    }

    pub fn dims(&self) -> (usize, usize) {
        (self.rows.len(), self.cols.len())
    }
}
