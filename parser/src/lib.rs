extern crate core;

use std::fmt::{Debug, Display, Formatter};
use std::vec::IntoIter;

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
    Add(isize),
    Subtract(isize),
    Left(isize),
    Right(isize),
    Clear,
    Output,
    Input,
    /// Multiply is based on the common brainfuck operation `>+++[<++>-]`.
    /// In this example 3 and 2 are multiplied into cell 0. Mc corresponds to the additions in the loop, the example being 2.
    /// Offset is the amount of offset from the cell where the other value is stored to the result cell
    Multiply { multiplicand: isize, offset: isize },
}

impl Display for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Instruction::Loop(instructions) => {
                f.write_str("[")?;
                for ins in instructions {
                    f.write_str(&format!("{}", ins))?;
                }
                f.write_str("]")?;
            }
            Instruction::Add(amount) => f.write_str(&"+".repeat(*amount as usize))?,
            Instruction::Subtract(amount) => f.write_str(&"-".repeat(*amount as usize))?,
            Instruction::Left(amount) => f.write_str(&"<".repeat(*amount as usize))?,
            Instruction::Right(amount) => f.write_str(&">".repeat(*amount as usize))?,
            Instruction::Clear => f.write_str("[-]")?,
            Instruction::Input => f.write_str(",")?,
            Instruction::Multiply { offset, multiplicand: mc } => {
                if *offset >= 0 {
                    f.write_str(&format!(
                        "[{}{}{}-]",
                        ">".repeat(*offset as usize),
                        "+".repeat(*mc as usize),
                        "<".repeat(*offset as usize)
                    ))?;
                } else {
                    f.write_str(&format!(
                        "[{}{}{}-]",
                        "<".repeat(*offset as usize),
                        "+".repeat(*mc as usize),
                        ">".repeat(*offset as usize)
                    ))?;
                }
            }
            Instruction::Output => f.write_str(".")?,
        }
        Ok(())
    }
}

impl Instruction {
    pub fn to_number(&self) -> isize {
        match self {
            Instruction::Add(plus) => *plus,
            Instruction::Subtract(minus) => -*minus,
            Instruction::Left(left) => -*left,
            Instruction::Right(right) => *right,
            _ => 0,
        }
    }

    pub fn is_loop(&self) -> bool {
        matches!(self, Instruction::Loop(_) | Instruction::Clear)
    }
}

pub struct Parser {
    tokens: IntoIter<LexerToken>,
    instructions: Vec<Instruction>,
}

impl Parser {
    pub fn new(tokens: Vec<LexerToken>) -> Self {
        Parser {
            tokens: tokens.into_iter(),
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
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Loop(vec![
            Instruction::Right(1),
            Instruction::Add(1),
            Instruction::Add(1),
            Instruction::Add(1),
            Instruction::Add(1),
            Instruction::Loop(vec![
                Instruction::Right(1),
                Instruction::Add(1),
                Instruction::Add(1),
                Instruction::Right(1),
                Instruction::Add(1),
                Instruction::Add(1),
                Instruction::Add(1),
                Instruction::Right(1),
                Instruction::Add(1),
                Instruction::Add(1),
                Instruction::Add(1),
                Instruction::Right(1),
                Instruction::Add(1),
                Instruction::Left(1),
                Instruction::Left(1),
                Instruction::Left(1),
                Instruction::Left(1),
                Instruction::Subtract(1),
            ]),
            Instruction::Right(1),
            Instruction::Add(1),
            Instruction::Right(1),
            Instruction::Add(1),
            Instruction::Right(1),
            Instruction::Subtract(1),
            Instruction::Right(1),
            Instruction::Right(1),
            Instruction::Add(1),
            Instruction::Loop(vec![Instruction::Left(1)]),
            Instruction::Left(1),
            Instruction::Subtract(1),
        ]),
        Instruction::Right(1),
        Instruction::Right(1),
        Instruction::Output,
        Instruction::Right(1),
        Instruction::Subtract(1),
        Instruction::Subtract(1),
        Instruction::Subtract(1),
        Instruction::Output,
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Output,
        Instruction::Output,
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Output,
        Instruction::Right(1),
        Instruction::Right(1),
        Instruction::Output,
        Instruction::Left(1),
        Instruction::Subtract(1),
        Instruction::Output,
        Instruction::Left(1),
        Instruction::Output,
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Output,
        Instruction::Subtract(1),
        Instruction::Subtract(1),
        Instruction::Subtract(1),
        Instruction::Subtract(1),
        Instruction::Subtract(1),
        Instruction::Subtract(1),
        Instruction::Output,
        Instruction::Subtract(1),
        Instruction::Subtract(1),
        Instruction::Subtract(1),
        Instruction::Subtract(1),
        Instruction::Subtract(1),
        Instruction::Subtract(1),
        Instruction::Subtract(1),
        Instruction::Subtract(1),
        Instruction::Output,
        Instruction::Right(1),
        Instruction::Right(1),
        Instruction::Add(1),
        Instruction::Output,
        Instruction::Right(1),
        Instruction::Add(1),
        Instruction::Add(1),
        Instruction::Output,
    ]
}
