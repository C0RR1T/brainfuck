use std::env;
use std::io::stdout;

use interpreter::Interpreter;

fn main() {
    let file = env::args().nth(1).expect("Please provide an input file.");
    Interpreter::new(stdout()).interpret_file(&file);
}
