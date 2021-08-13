#![allow(unused)]

mod board;
mod clues;

use board::*;
use clues::*;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let desc = std::fs::read_to_string(path).unwrap();
    let clues = Clues::parse(&desc).unwrap();
    println!("{:?}", clues);
}
