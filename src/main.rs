mod ast;
mod lexer;
mod parser;
mod codegen;

use std::fs;

fn main() {
    let src = fs::read_to_string("example.zr").expect("example.zr not found");

    let tokens = lexer::tokenize(&src);
    let mut parser = parser::Parser::new(tokens);
    let ast = parser.parse();

    codegen::run(ast);
}
