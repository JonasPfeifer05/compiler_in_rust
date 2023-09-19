pub mod tokenizer;
pub mod parser;
pub mod semantic_analysis;


use std::fs;
use crate::parser::Parser;
use crate::semantic_analysis::symbol_table::SymbolTable;
use crate::tokenizer::Tokenizer;

fn main() {

    let input: Vec<_> = fs::read("res/script.he").expect("Unknown file!")
        .iter()
        .map(|char| *char as char)
        .collect();

    let tokens = Tokenizer::new(input).tokenize();

    tokens.iter()
        .for_each(|token| println!("{}", token));

    println!();

    let mut statements = Parser::new(tokens).parse_statements();

    statements.iter()
        .for_each(|statement| println!("{:?}", statement));

    println!();

    let mut symbol_table = SymbolTable::new();
    statements.iter_mut()
        .for_each(|statements| statements.resolve(&mut symbol_table));

    statements.iter()
        .for_each(|statement| println!("{:?}", statement));
}
