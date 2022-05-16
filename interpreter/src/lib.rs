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

fn print_error(err: &ParserError, input: &str) {
    match err {
        ParserError::UnexpectedEOF(span) => {
            print_error_line("End of file before loop close");
            print_location(span, input, "close the loop before the file ends")
        }

        ParserError::UnexpectedClosing(span) => {
            print_error_line("Unexpected closing of loop");
            print_location(span, input, "Remove the unnecessary \"]\"");
        }
    }
}

fn print_error_line(message: &str) {
    eprintln!("{}: {}", "Error".bold().bright_red(), message);
}

fn print_location(span: &Span, input: &str, note: &str) {
    let line = get_line_of_error(span, &input);
    let info_string = format!("{} | ", line + 1);
    let code_line = get_line(line, &input);
    eprintln!("{} {}", " ".repeat(info_string.len() - 3), "|".blue());
    eprintln!("{}{}", info_string.blue(), code_line);
    eprintln!(
        "{} {}{}{}",
        " ".repeat(info_string.len() - 3),
        "|".blue(),
        " ".repeat(code_line.split_at(span.from).0.len() + 1),
        "^".repeat(span.to - span.from).bright_red(),
    );
    " ".repeat(info_string.len() - 3);
    eprintln!(
        "{}{} {}: {}",
        " ".repeat(info_string.len() - 2),
        "=".blue(),
        "note".bold(),
        note
    )
}

fn get_line(line: usize, input: &str) -> &str {
    input.lines().nth(line).unwrap()
}

fn get_line_of_error(span: &Span, input: &str) -> usize {
    input
        .char_indices()
        .filter(|(i, char)| *i < span.from && *char == '\n')
        .map(|t| t.0)
        .count()
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
