use std::fs;
use criterion::{
    black_box,
    criterion_group,
    criterion_main,
    Criterion
};
use compiler_in_rust::parser::Parser;
use compiler_in_rust::tokenizer::Tokenizer;

fn compile_small_program(b: &mut Criterion) {
    let input: Vec<_> = black_box(fs::read("res/script.he").expect("Unknown file!")
        .iter()
        .map(|char| *char as char)
        .collect());

    b.bench_function(
        "compile small program",
        |b| b.iter(|| {
            let tokens = Tokenizer::new(input.clone()).tokenize();
            let _statements = Parser::new(tokens).parse_statements();
        })
    );
}

criterion_group!(benches, compile_small_program);
criterion_main!(benches);