use std::{env, fs};

use lexer::lex;
use parser::{Instruction, Parser};

pub struct Interpreter {
    cells: [u8; 32_000],
    pointer: usize,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            cells: [0; 32_000],
            pointer: 0,
        }
    }

    pub fn interpret(&mut self, instructions: &[Instruction]) -> String {
        let mut output = String::new();
        for instruction in instructions.iter() {
            match instruction {
                Instruction::Left => self.pointer -= 1,
                Instruction::Loop(loop_instructions) => {
                    output.push_str(&self.interpret_loop(loop_instructions))
                }
                Instruction::Add => {
                    self.cells[self.pointer] = self.cells[self.pointer].wrapping_add(1)
                }
                Instruction::Subtract => {
                    self.cells[self.pointer] = self.cells[self.pointer].wrapping_sub(1)
                }
                Instruction::Right => self.pointer += 1,
                Instruction::Output => {
                    output.push(self.cells[self.pointer] as char);
                }
                Instruction::Input => self.cells[self.pointer] = read_input(),
            }
        }
        output
    }

    fn interpret_loop(&mut self, instructions: &[Instruction]) -> String {
        let mut output = String::new();
        while self.cells[self.pointer] != 0 {
            output.push_str(&self.interpret(instructions))
        }
        output
    }

    pub fn interpret_file(&mut self, file: String) {
        let file = fs::read_to_string(file);

        match file {
            Ok(file) => println!("{}", self.interpret(&Parser::new(lex(&file)).parse())),
            Err(err) => eprintln!("Error while reading file: {}", err),
        }
    }
}

fn read_input() -> u8 {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    *input.as_bytes().get(0).unwrap_or(&0)
}

#[test]
fn hello_world() {
    assert_eq!(
        Interpreter::new().interpret(&parser::hello_world()[..]),
        "Hello World!\n"
    );
}