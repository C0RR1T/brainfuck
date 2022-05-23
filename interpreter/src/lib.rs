use std::fs;
use std::io::Read;

use error_messages::print_error;
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

    pub fn interpret_ins(&mut self, instructions: &[Instruction]) {
        for instruction in instructions.iter() {
            match instruction {
                Instruction::Left(amount) => {
                    self.pointer = self.pointer.wrapping_sub(*amount as usize)
                }
                Instruction::Right(amount) => {
                    self.pointer = self.pointer.wrapping_add(*amount as usize)
                }
                Instruction::Loop(loop_instructions) => self.interpret_loop(loop_instructions),
                Instruction::Add(amount) => {
                    self.cells[self.pointer] = self.cells[self.pointer].wrapping_add(*amount)
                }

                Instruction::Subtract(amount) => {
                    self.cells[self.pointer] = self.cells[self.pointer].wrapping_sub(*amount)
                }

                Instruction::Output => {
                    print!("{}", (self.cells[self.pointer] as char));
                }
                Instruction::Input => self.cells[self.pointer] = read_input(),
                Instruction::Clear => self.cells[self.pointer] = 0,
                Instruction::Multiply { offset, mc } => {
                    let old_pointer = self.pointer;
                    self.offset_to_pointer(*offset);
                    self.cells[self.pointer] = self.cells[self.pointer]
                        .wrapping_mul(self.cells[old_pointer].wrapping_mul(*mc));
                    self.cells[old_pointer] = 0;
                }
                Instruction::Divide { offset, dv } => {
                    let old_pointer = self.pointer;
                    self.offset_to_pointer(*offset);
                    self.cells[self.pointer] =
                        self.cells[self.pointer].wrapping_div(self.cells[old_pointer] * dv);
                    self.cells[old_pointer] = 0;
                }
            }
        }
    }

    pub fn interpret(&mut self, instructions: Vec<Instruction>, opt: bool) {
        if opt {
            let instructions = optimizer::Optimizer::new(instructions).optimize();
            self.interpret_ins(&instructions)
        } else {
            self.interpret_ins(&instructions)
        }
    }

    fn offset_to_pointer(&mut self, offset: isize) {
        self.pointer = {
            if offset <= 0 {
                self.pointer.wrapping_sub(offset.abs() as usize)
            } else {
                self.pointer.wrapping_add(offset as usize)
            }
        };
    }

    fn interpret_loop(&mut self, instructions: &[Instruction]) {
        while self.cells[self.pointer] != 0 {
            self.interpret_ins(instructions)
        }
    }

    pub fn interpret_file(&mut self, file: &str, opt: bool) {
        let file = fs::read_to_string(file);

        match file {
            Ok(file) => {
                let instructions = Parser::new(lex(&file)).parse();

                match instructions {
                    Ok(instructions) => self.interpret(instructions, opt),
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

// #[test]
// fn hello_world() {
//     assert_eq!(
//         Interpreter::new().interpret_ins(&parser::hello_world()[..]),
//         "Hello World!\n"
//     );
// }
