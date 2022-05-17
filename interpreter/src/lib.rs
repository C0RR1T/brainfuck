use error_messages::print_error;
use owo_colors::OwoColorize;
use std::fs;
use std::io::Read;

use lexer::{lex, Span};
use parser::{Instruction, Parser, ParserError};

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

    pub fn interpret(&mut self, instructions: &[Instruction]) {
        for instruction in instructions.iter() {
            match instruction {
                Instruction::Left(amount) => self.pointer -= *amount as usize,
                Instruction::Loop(loop_instructions) => self.interpret_loop(loop_instructions),
                Instruction::Add(amount) => {
                    self.cells[self.pointer] = self.cells[self.pointer].wrapping_add(*amount)
                }

                Instruction::Subtract(amount) => {
                    self.cells[self.pointer] = self.cells[self.pointer].wrapping_sub(*amount)
                }

                Instruction::Right(amount) => self.pointer += *amount as usize,
                Instruction::Output => {
                    print!("{}", self.cells[self.pointer] as char);
                }
                Instruction::Input => self.cells[self.pointer] = read_input(),
                Instruction::Clear => self.cells[self.pointer] = 0,
            }
        }
    }

    fn interpret_loop(&mut self, instructions: &[Instruction]) {
        while self.cells[self.pointer] != 0 {
            self.interpret(instructions)
        }
    }

    pub fn interpret_file(&mut self, file: &str) {
        let file = fs::read_to_string(file);

        match file {
            Ok(file) => {
                let instructions = Parser::new(lex(&file)).parse();

                match instructions {
                    Ok(instructions) => self.interpret(&instructions),
                    Err(err) => {
                        print_error(&err, &file);
                        std::process::exit(1);
                    }
                }
            }
            Err(err) => {
                eprintln!("Error while reading file: {}", err);
                std::process::exit(1)
            }
        }
    }
}

fn read_input() -> u8 {
    let mut buf = [0; 1];
    std::io::stdin().read_exact(&mut buf).unwrap();
    buf[0]
}

#[test]
fn hello_world() {
    assert_eq!(
        Interpreter::new().interpret(&parser::hello_world()[..]),
        "Hello World!\n"
    );
}
