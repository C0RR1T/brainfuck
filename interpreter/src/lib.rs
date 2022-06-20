use error_messages::print_error;
use owo_colors::OwoColorize;
use std::fs;
use std::io::{stdout, Read, Write};

use lexer::{lex, Span};
use parser::{Instruction, Parser, ParserError};

pub struct Interpreter<W> {
    cells: [u8; 32_000],
    pointer: usize,
    output: W,
}

impl<W: Write> Interpreter<W> {
    pub fn new(output: W) -> Self
    where
        W: Write,
    {
        Interpreter {
            cells: [0; 32_000],
            pointer: 0,
            output,
        }
    }

    pub fn interpret(&mut self, instructions: &[Instruction]) {
        for instruction in instructions.iter() {
            match instruction {
                Instruction::Left => self.pointer -= 1,
                Instruction::Loop(loop_instructions) => self.interpret_loop(loop_instructions),
                Instruction::Add => {
                    self.cells[self.pointer] = self.cells[self.pointer].wrapping_add(1)
                }
                Instruction::Subtract => {
                    self.cells[self.pointer] = self.cells[self.pointer].wrapping_sub(1)
                }
                Instruction::Right => self.pointer += 1 as usize,
                Instruction::Output => {
                    let char = self.cells[self.pointer] as char;
                    write!(self.output, "{char}").unwrap();
                }
                Instruction::Input => self.cells[self.pointer] = read_input(),
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
    let mut out = Vec::new();
    Interpreter::new(out).interpret(&parser::hello_world()[..]);
    assert_eq!(
        out.iter().map(|x| *x as char).collect::<String>(),
        "Hello World!\n"
    );
}
