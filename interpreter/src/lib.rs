use std::{fs, io};
use std::io::{Read, Write};
use std::num::Wrapping;

use error_messages::print_error;
use lexer::lex;
use parser::{Instruction, Parser};

const MEM_SIZE: usize = 32_000;

pub struct Interpreter<'a, W: Write> {
    cells: [Wrapping<u8>; MEM_SIZE],
    pointer: usize,
    out: &'a mut W,
}

impl<'a, W: Write> Interpreter<'a, W> {
    pub fn new(out: &'a mut W) -> Self
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
                    self.cells[self.pointer] = Wrapping(add_to_u8(self.cells[self.pointer].0, *amount));
                }

                Instruction::Subtract(amount) => {
                    self.cells[self.pointer] = Wrapping(add_to_u8(self.cells[self.pointer].0, -*amount));
                }

                Instruction::Output => {
                    let print_char: char = (self.cells[self.pointer].0.into());
                    #[cfg(debug_assertions)]
                    println!("Printing char {print_char} at position {} with value {}", self.pointer, self.cells[self.pointer]);

                    write!(self.out, "{}", print_char)
                        .expect("Couldn't write to stdout");
                }
                Instruction::Input => self.cells[self.pointer] = Wrapping(read_input()),
                Instruction::Clear => self.cells[self.pointer] = Wrapping(0),
                Instruction::Multiply { offset, multiplicand } => {
                    let old_pointer = self.pointer;
                    self.offset_to_pointer(*offset);
                    self.cells[self.pointer] += multiply_to_u8(self.cells[old_pointer].0, *multiplicand);
                    self.cells[old_pointer] = Wrapping(0);
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

fn multiply_to_u8(cell: u8, multiplier: isize) -> u8 {
    (((cell as isize) * multiplier + (u8::MAX as isize)) % (u8::MAX as isize)) as u8
}

fn add_to_u8(cell: u8, addition: isize) -> u8 {
    (((cell as isize + addition) + (u8::MAX as isize)) % (u8::MAX as isize)) as u8
}

fn read_input() -> u8 {
    let mut buf = [0; 1];
    io::stdin().read_exact(&mut buf).unwrap();
    buf[0]
}

#[cfg(test)]
const HELLO_WORLD: &str = include_str!("../../brainfuck-example/hello-world.bf");

#[test]
fn hello_world() {
    let mut out = Vec::new();

    Interpreter::new(&mut out).interpret(HELLO_WORLD, true);
    assert_eq!(&out[..], "Hello World!".as_bytes())
}
