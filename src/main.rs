#![allow(unused)]

mod board;
use board::*;

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let desc = std::fs::read_to_string(path).unwrap();
    let board = Board::parse(&desc).unwrap();
    println!("{:?}", board);
}
