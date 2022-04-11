extern crate core;

use std::vec::IntoIter;

use lexer::Token;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Instruction {
    Loop(Vec<Instruction>),
    Add,
    Subtract,
    Left,
    Right,
    Output,
    Input,
}

pub struct Parser {
    tokens: IntoIter<Token>,
    instructions: Vec<Instruction>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter(),
            instructions: Vec::new(),
        }
    }

    pub fn parse(&mut self) -> Vec<Instruction> {
        while let Some(token) = self.tokens.next() {
            let instruction = self.parse_token(&token);
            self.instructions.push(instruction);
        }
        self.instructions.to_vec()
    }

    fn parse_token(&mut self, token: &Token) -> Instruction {
        match token {
            Token::OpenLoop => self.parse_loop(),
            Token::CloseLoop => panic!("Unexpected Closing Bracket."),
            Token::Add => Instruction::Add,
            Token::Subtract => Instruction::Subtract,
            Token::Left => Instruction::Left,
            Token::Right => Instruction::Right,
            Token::Output => Instruction::Output,
            Token::Input => Instruction::Input,
        }
    }

    fn parse_loop(&mut self) -> Instruction {
        let mut loop_instructions = Vec::new();
        while let Some(token) = self.tokens.next() {
            if token == Token::CloseLoop {
                break;
            } else {
                loop_instructions.push(self.parse_token(&token));
            }
        }
        Instruction::Loop(loop_instructions)
    }
}
