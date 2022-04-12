extern crate core;

use std::vec::IntoIter;

use peekmore::{PeekMore, PeekMoreIterator};

use lexer::Token;

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Instruction {
    Loop(Vec<Instruction>),
    Add(u8),
    Subtract(u8),
    Left(u8),
    Right(u8),
    Output,
    Input,
}

pub struct Parser {
    tokens: PeekMoreIterator<IntoIter<Token>>,
    instructions: Vec<Instruction>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekmore(),
            instructions: Vec::new(),
        }
    }

    pub fn parse(mut self) -> Vec<Instruction> {
        while let Some(token) = self.next() {
            let instruction = self.parse_token(&token);
            self.instructions.push(instruction);
        }
        self.instructions
    }

    fn parse_token(&mut self, token: &Token) -> Instruction {
        match token {
            Token::OpenLoop => self.parse_loop(),
            Token::CloseLoop => panic!("Unexpected Closing Bracket."),
            Token::Add => Instruction::Add(self.combine_token(Token::Add)),
            Token::Subtract => Instruction::Subtract(self.combine_token(Token::Subtract)),
            Token::Left => Instruction::Left(self.combine_token(Token::Left)),
            Token::Right => Instruction::Right(self.combine_token(Token::Right)),
            Token::Output => Instruction::Output,
            Token::Input => Instruction::Input,
        }
    }

    fn combine_token(&mut self, token_to_match: Token) -> u8 {
        let mut amount = 1;
        while let Some(token) = self.peek_nth((amount - 1) as usize) {
            if token == token_to_match && amount < u8::MAX {
                amount += 1;
            } else {
                break;
            }
        }
        if amount > 1 {
            self.consume_elements((amount - 1) as usize);
        }
        amount
    }

    fn parse_loop(&mut self) -> Instruction {
        let mut loop_instructions = Vec::new();
        while let Some(token) = self.next() {
            if token == Token::CloseLoop {
                break;
            } else {
                loop_instructions.push(self.parse_token(&token));
            }
        }
        Instruction::Loop(loop_instructions)
    }

    fn next(&mut self) -> Option<Token> {
        self.tokens.next()
    }

    fn consume_elements(&mut self, amount: usize) {
        for _ in 0..amount {
            self.tokens.next();
        }
    }

    fn peek(&mut self) -> Option<Token> {
        self.tokens.peek().copied()
    }

    fn peek_nth(&mut self, amount: usize) -> Option<Token> {
        self.tokens.peek_nth(amount).copied()
    }
}

#[test]
fn parser_test() {
    assert_eq!(
        Parser::new(vec![Token::Left, Token::Right, Token::Add]).parse(),
        vec![
            Instruction::Left(1),
            Instruction::Right(1),
            Instruction::Add(1)
        ]
    );
    assert_eq!(
        Parser::new(vec![
            Token::Add,
            Token::Add,
            Token::Add,
            Token::Add,
            Token::Add,
            Token::Add,
            Token::Add,
            Token::Add,
            Token::OpenLoop,
            Token::Right,
            Token::Add,
            Token::Add,
            Token::Add,
            Token::Add,
            Token::OpenLoop,
            Token::Right,
            Token::Add,
            Token::Add,
            Token::Right,
            Token::Add,
            Token::Add,
            Token::Add,
            Token::Right,
            Token::Add,
            Token::Add,
            Token::Add,
            Token::Right,
            Token::Add,
            Token::Left,
            Token::Left,
            Token::Left,
            Token::Left,
            Token::Subtract,
            Token::CloseLoop,
            Token::Right,
            Token::Add,
            Token::Right,
            Token::Add,
            Token::Right,
            Token::Subtract,
            Token::Right,
            Token::Right,
            Token::Add,
            Token::OpenLoop,
            Token::Left,
            Token::CloseLoop,
            Token::Left,
            Token::Subtract,
            Token::CloseLoop,
            Token::Right,
            Token::Right,
            Token::Output,
            Token::Right,
            Token::Subtract,
            Token::Subtract,
            Token::Subtract,
            Token::Output,
            Token::Add,
            Token::Add,
            Token::Add,
            Token::Add,
            Token::Add,
            Token::Add,
            Token::Add,
            Token::Output,
            Token::Output,
            Token::Add,
            Token::Add,
            Token::Add,
            Token::Output,
            Token::Right,
            Token::Right,
            Token::Output,
            Token::Left,
            Token::Subtract,
            Token::Output,
            Token::Left,
            Token::Output,
            Token::Add,
            Token::Add,
            Token::Add,
            Token::Output,
            Token::Subtract,
            Token::Subtract,
            Token::Subtract,
            Token::Subtract,
            Token::Subtract,
            Token::Subtract,
            Token::Output,
            Token::Subtract,
            Token::Subtract,
            Token::Subtract,
            Token::Subtract,
            Token::Subtract,
            Token::Subtract,
            Token::Subtract,
            Token::Subtract,
            Token::Output,
            Token::Right,
            Token::Right,
            Token::Add,
            Token::Output,
            Token::Right,
            Token::Add,
            Token::Add,
            Token::Output,
        ])
        .parse(),
        hello_world()
    )
}

pub fn hello_world() -> Vec<Instruction> {
    vec![
        Instruction::Add(8),
        Instruction::Loop(vec![
            Instruction::Right(1),
            Instruction::Add(4),
            Instruction::Loop(vec![
                Instruction::Right(1),
                Instruction::Add(2),
                Instruction::Right(1),
                Instruction::Add(3),
                Instruction::Right(1),
                Instruction::Add(3),
                Instruction::Right(1),
                Instruction::Add(1),
                Instruction::Left(4),
                Instruction::Subtract(1),
            ]),
            Instruction::Right(1),
            Instruction::Add(1),
            Instruction::Right(1),
            Instruction::Add(1),
            Instruction::Right(1),
            Instruction::Subtract(1),
            Instruction::Right(2),
            Instruction::Add(1),
            Instruction::Loop(vec![Instruction::Left(1)]),
            Instruction::Left(1),
            Instruction::Subtract(1),
        ]),
        Instruction::Right(2),
        Instruction::Output,
        Instruction::Right(1),
        Instruction::Subtract(3),
        Instruction::Output,
        Instruction::Add(7),
        Instruction::Output,
        Instruction::Output,
        Instruction::Add(3),
        Instruction::Output,
        Instruction::Right(2),
        Instruction::Output,
        Instruction::Left(1),
        Instruction::Subtract(1),
        Instruction::Output,
        Instruction::Left(1),
        Instruction::Output,
        Instruction::Add(3),
        Instruction::Output,
        Instruction::Subtract(6),
        Instruction::Output,
        Instruction::Subtract(8),
        Instruction::Output,
        Instruction::Right(2),
        Instruction::Add(1),
        Instruction::Output,
        Instruction::Right(1),
        Instruction::Add(2),
        Instruction::Output,
    ]
}
