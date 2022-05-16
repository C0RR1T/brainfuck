use std::fs;
use std::io::Read;

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
                Instruction::Left(amount) => self.pointer -= *amount as usize,
                Instruction::Loop(loop_instructions) => {
                    output.push_str(&self.interpret_loop(loop_instructions))
                }
                Instruction::Add(amount) => {
                    self.cells[self.pointer] = self.cells[self.pointer].wrapping_add(*amount)
                }
                Instruction::Subtract(amount) => {
                    self.cells[self.pointer] = self.cells[self.pointer].wrapping_sub(*amount)
                }
                Instruction::Right(amount) => self.pointer += *amount as usize,
                Instruction::Output => {
                    output.push(self.cells[self.pointer] as char);
                }
                Instruction::Input => self.cells[self.pointer] = read_input(),
                Instruction::Clear => self.cells[self.pointer] = 0,
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
        println!("{}", self.interpret_file_quiet(file))
    }

    pub fn interpret_file_quiet(&mut self, file: String) -> String {
        let file = fs::read_to_string(file);

        match file {
            Ok(file) => {
                let instructions = Parser::new(lex(&file)).parse();

                match instructions {
                    Ok(instructions) => self.interpret(&instructions),
                    Err(err) => {}
                }
            }
            Err(err) => {
                eprintln!("Error while reading file: {}", err);
                std::process::exit(1)
            }
        }
    }
}

fn print_error()

fn read_input() -> u8 {
    let mut buf = [0;1];
    std::io::stdin()
        .read_exact(&mut buf)
        .unwrap();
    buf[0]
}

#[test]
fn hello_world() {
    assert_eq!(
        Interpreter::new().interpret(&parser::hello_world()[..]),
        "Hello World!\n"
    );
}
