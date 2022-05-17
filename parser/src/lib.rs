extern crate core;

use std::vec::IntoIter;

use peekmore::{PeekMore, PeekMoreIterator};

use lexer::{LexerToken, Span, TokenType};

use crate::ParserError::UnexpectedEOF;

pub type ParserResult<T> = Result<T, ParserError>;

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum ParserError {
    UnexpectedEOF(Span),
    UnexpectedClosing(Span),
}

#[derive(Eq, PartialEq, Debug, Clone)]
pub enum Instruction {
    Loop(Vec<Instruction>),
    Add(u8),
    Subtract(u8),
    Left(u8),
    Right(u8),
    Clear,
    Output,
    Input,
    // MC: Multiplicant (In the loop) / MP: Multiplier (Multiplier / current cell)
    Multiply { mc: u8, offset: isize },
    // DV: Dividend (In the loop)
    Divide { dv: u8, offset: isize },
}

impl Instruction {
    pub fn to_number(&self) -> isize {
        match self {
            Instruction::Add(plus) => *plus as isize,
            Instruction::Subtract(minus) => -(*minus as isize),
            Instruction::Left(left) => -(*left as isize),
            Instruction::Right(right) => *right as isize,
            _ => 0,
        }
    }
}

pub struct Parser {
    tokens: PeekMoreIterator<IntoIter<LexerToken>>,
    instructions: Vec<Instruction>,
}

impl Parser {
    pub fn new(tokens: Vec<LexerToken>) -> Self {
        Parser {
            tokens: tokens.into_iter().peekmore(),
            instructions: Vec::new(),
        }
    }

    pub fn parse(mut self) -> ParserResult<Vec<Instruction>> {
        while let Some(token) = self.next() {
            let instruction = self.parse_token(&token);
            self.instructions.push(instruction?);
        }
        Ok(self.instructions)
    }

    fn parse_token(&mut self, token: &LexerToken) -> ParserResult<Instruction> {
        match token {
            LexerToken {
                token: TokenType::OpenLoop,
                ..
            } => self.parse_loop(token),
            LexerToken {
                token: TokenType::CloseLoop,
                ..
            } => Err(ParserError::UnexpectedClosing(token.span)),
            LexerToken {
                token: TokenType::Add,
                ..
            } => Ok(Instruction::Add(1)),
            LexerToken {
                token: TokenType::Subtract,
                ..
            } => Ok(Instruction::Subtract(1)),
            LexerToken {
                token: TokenType::Left,
                ..
            } => Ok(Instruction::Left(1)),
            LexerToken {
                token: TokenType::Right,
                ..
            } => Ok(Instruction::Right(1)),
            LexerToken {
                token: TokenType::Output,
                ..
            } => Ok(Instruction::Output),
            LexerToken {
                token: TokenType::Input,
                ..
            } => Ok(Instruction::Input),
        }
    }

    fn parse_loop(&mut self, first_token: &LexerToken) -> ParserResult<Instruction> {
        let mut loop_instructions = Vec::new();
        while let Some(token) = self.next() {
            match token {
                LexerToken {
                    token: TokenType::CloseLoop,
                    ..
                } => return Ok(Instruction::Loop(loop_instructions)),

                token => {
                    let new_token = self.parse_token(&token)?;
                    loop_instructions.push(new_token);
                }
            }
        }

        Err(UnexpectedEOF(Span::from(
            first_token.span.from,
            first_token.span.to,
        )))
    }

    fn next(&mut self) -> Option<LexerToken> {
        self.tokens.next()
    }

    fn consume_elements(&mut self, amount: usize) {
        for _ in 0..amount {
            self.tokens.next();
        }
    }

    fn peek(&mut self) -> Option<LexerToken> {
        self.tokens.peek().copied()
    }

    fn peek_nth(&mut self, amount: usize) -> Option<LexerToken> {
        self.tokens.peek_nth(amount).copied()
    }
}

#[test]
fn parser_test() {
    assert_eq!(
        Parser::new(vec![
            LexerToken::new(Span::from(0, 0), TokenType::Left),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Add)
        ])
        .parse()
        .unwrap(),
        vec![
            Instruction::Left(1),
            Instruction::Right(1),
            Instruction::Add(1)
        ]
    );
    assert_eq!(
        Parser::new(vec![
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::OpenLoop),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::OpenLoop),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Left),
            LexerToken::new(Span::from(0, 0), TokenType::Left),
            LexerToken::new(Span::from(0, 0), TokenType::Left),
            LexerToken::new(Span::from(0, 0), TokenType::Left),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::CloseLoop),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::OpenLoop),
            LexerToken::new(Span::from(0, 0), TokenType::Left),
            LexerToken::new(Span::from(0, 0), TokenType::CloseLoop),
            LexerToken::new(Span::from(0, 0), TokenType::Left),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::CloseLoop),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Output),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Output),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Output),
            LexerToken::new(Span::from(0, 0), TokenType::Output),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Output),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Output),
            LexerToken::new(Span::from(0, 0), TokenType::Left),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Output),
            LexerToken::new(Span::from(0, 0), TokenType::Left),
            LexerToken::new(Span::from(0, 0), TokenType::Output),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Output),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Output),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Subtract),
            LexerToken::new(Span::from(0, 0), TokenType::Output),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Output),
            LexerToken::new(Span::from(0, 0), TokenType::Right),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Add),
            LexerToken::new(Span::from(0, 0), TokenType::Output),
        ])
        .parse()
        .unwrap(),
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
