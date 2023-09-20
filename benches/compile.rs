use std::fs;
use std::str::Chars;
use criterion::{
    criterion_group,
    criterion_main,
    Criterion
};
use compiler_in_rust::parser::Parser;
use compiler_in_rust::tokenizer::Tokenizer;

fn compile_small_program(b: &mut Criterion) {
    let input_string: String = fs::read_to_string("res/script.he").expect("Unknown file!");
    let input_chars: Chars = input_string.chars();

    b.bench_function(
        "compile small program",
        |b| b.iter(|| {
            let tokens = Tokenizer::new(input_chars.clone().peekable()).tokenize()
                .into_iter()
                .peekable();
            let _statements = Parser::new(tokens).parse_statements();
        })
    );
}

criterion_group!(benches, compile_small_program);
criterion_main!(benches);