mod tokenizer;
mod parser;

use std::fs;
use crate::tokenizer::Tokenizer;

fn main() {
    let input: Vec<_> = fs::read("res/script.he").expect("Unknown file!")
        .iter()
        .map(|char| *char as char)
        .collect();

    let tokens = Tokenizer::new(input).tokenize();

    tokens.iter()
        .for_each(|token| println!("{}", token))
}
