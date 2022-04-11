use std::{env, fs};

use lexer::lex;
use parser::{parse, Instruction};

fn main() {
    let file = env::args().nth(1).expect("Please provide an input file.");

    println!("{}", file);

    let file = fs::read_to_string(file);

    match file {
        Ok(file) => Interpreter::new().interpret(&parse(lex(&file))),
        Err(err) => eprintln!("Error while reading file: {}", err),
    }
}

struct Interpreter {
    cells: [u8; 32_000],
    pointer: usize,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            cells: [0; 32_000],
            pointer: 0,
        }
    }

    fn interpret(&mut self, instructions: &Vec<Instruction>) {
        for instruction in instructions.iter() {
            match instruction {
                Instruction::Left => self.pointer += 1,
                Instruction::Loop(loop_instructions) => self.interpret(loop_instructions),
                Instruction::Add => self.cells[self.pointer] += 1,
                Instruction::Subtract => self.cells[self.pointer] -= 1,
                Instruction::Right => self.pointer += 1,
                Instruction::Output => print!("{}", self.cells[self.pointer] as char),
                Instruction::Input => self.cells[self.pointer] = read_input(),
            }
        }
    }
}

fn read_input() -> u8 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.bytes().nth(0).unwrap_or(0)
}
