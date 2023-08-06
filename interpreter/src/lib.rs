use std::io::{stdout, Read, Write};
use std::num::Wrapping;
use std::{fs, io};

use error_messages::print_error;
use lexer::lex;
use parser::{Instruction, Parser};

const MEM_SIZE: usize = 32_000;

pub struct Interpreter<W: Write> {
    cells: [Wrapping<u8>; MEM_SIZE],
    pointer: usize,
    out: W,
}

impl<W: Write> Interpreter<W> {
    pub fn new(out: W) -> Self
    where
        W: Write,
    {
        Interpreter {
            cells: [Wrapping(0); MEM_SIZE],
            pointer: 0,
            out,
        }
    }

    pub fn interpret_ins(&mut self, instructions: &[Instruction]) {
        for instruction in instructions.iter() {
            match instruction {
                Instruction::Left(amount) => self.offset_to_pointer(*amount),
                Instruction::Right(amount) => self.offset_to_pointer(*amount),
                Instruction::Loop(loop_instructions) => self.interpret_loop(loop_instructions),
                Instruction::Add(amount) => {
                    self.cells[self.pointer] += *amount;
                }

                Instruction::Subtract(amount) => {
                    self.cells -= *amount;
                    self.cells[self.pointer] = self.cells[self.pointer].wrapping_sub(*amount)
                }

                Instruction::Output => {
                    write!(&mut self.out, "{}", (self.cells[self.pointer].0 as char))
                        .expect("Couldn't write to stdout");
                }
                Instruction::Input => self.cells[self.pointer] = Wrapping(read_input()),
                Instruction::Clear => self.cells[self.pointer] = Wrapping(0),
                Instruction::Multiply { offset, multiplicand } => {
                    // TODO: Fix this mess
                    let old_pointer = self.pointer;
                    self.offset_to_pointer(*offset);
                    self.cells[self.pointer] = self.cells[self.pointer]
                        .wrapping_mul(self.cells[old_pointer].wrapping_mul(*mc));
                    self.pointer = old_pointer;
                    self.cells[self.pointer] = Wrapping(0);
                }
            }
        }
    }

    pub fn interpret(&mut self, src: &str, opt: bool) {
        let instructions = Parser::new(lex(src)).parse();

        match instructions {
            Ok(instructions) => {
                if opt {
                    let instructions = optimizer::Optimizer::new(&instructions).optimize();
                    self.interpret_ins(&instructions)
                } else {
                    self.interpret_ins(&instructions)
                }
            }
            Err(err) => {
                print_error(&err, src);
                std::process::exit(1);
            }
        }
    }

    fn offset_to_pointer(&mut self, offset: isize) {
        self.pointer = (self.pointer + MEM_SIZE + (offset as usize)) % MEM_SIZE;
    }

    fn interpret_loop(&mut self, instructions: &[Instruction]) {
        while self.cells[self.pointer] != Wrapping(0) {
            self.interpret_ins(instructions)
        }
    }

    pub fn interpret_file(&mut self, file: &str, opt: bool) {
        let file = fs::read_to_string(file);

        match file {
            Ok(file) => self.interpret(&file, opt),
            Err(err) => {
                eprintln!("Error while reading file: {}", err);
                std::process::exit(1)
            }
        }
    }
}
fn read_input() -> u8 {
    let mut buf = [0; 1];
    io::stdin().read_exact(&mut buf).unwrap();
    buf[0]
}

#[test]
fn hello_world() {
    Interpreter::new(stdout()).interpret("Hello World!\n", true);
    assert_eq!(
        Interpreter::new(stdout()).interpret_ins(&parser::hello_world()[..]),
        "Hello World!\n"
    );
}
