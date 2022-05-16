use std::env;

use interpreter::Interpreter;

fn main() {
    let file = env::args().nth(1).expect("Please provide an input file.");
    Interpreter::new().interpret_file(&file);
}
