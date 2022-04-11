use std::{env, fs};

use interpreter::Interpreter;
use lexer::lex;
use parser::Parser;

fn main() {
    let file = env::args().nth(1).expect("Please provide an input file.");
    Interpreter::new().interpret_file(file);
}
