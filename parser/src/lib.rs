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

#[test]
fn parser_test() {
    assert_eq!(
        Parser::new(vec![Token::Left, Token::Right, Token::Add]).parse(),
        vec![Instruction::Left, Instruction::Right, Instruction::Add]
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
        Instruction::Add,
        Instruction::Add,
        Instruction::Add,
        Instruction::Add,
        Instruction::Add,
        Instruction::Add,
        Instruction::Add,
        Instruction::Add,
        Instruction::Loop(vec![
            Instruction::Right,
            Instruction::Add,
            Instruction::Add,
            Instruction::Add,
            Instruction::Add,
            Instruction::Loop(vec![
                Instruction::Right,
                Instruction::Add,
                Instruction::Add,
                Instruction::Right,
                Instruction::Add,
                Instruction::Add,
                Instruction::Add,
                Instruction::Right,
                Instruction::Add,
                Instruction::Add,
                Instruction::Add,
                Instruction::Right,
                Instruction::Add,
                Instruction::Left,
                Instruction::Left,
                Instruction::Left,
                Instruction::Left,
                Instruction::Subtract,
            ]),
            Instruction::Right,
            Instruction::Add,
            Instruction::Right,
            Instruction::Add,
            Instruction::Right,
            Instruction::Subtract,
            Instruction::Right,
            Instruction::Right,
            Instruction::Add,
            Instruction::Loop(vec![Instruction::Left]),
            Instruction::Left,
            Instruction::Subtract,
        ]),
        Instruction::Right,
        Instruction::Right,
        Instruction::Output,
        Instruction::Right,
        Instruction::Subtract,
        Instruction::Subtract,
        Instruction::Subtract,
        Instruction::Output,
        Instruction::Add,
        Instruction::Add,
        Instruction::Add,
        Instruction::Add,
        Instruction::Add,
        Instruction::Add,
        Instruction::Add,
        Instruction::Output,
        Instruction::Output,
        Instruction::Add,
        Instruction::Add,
        Instruction::Add,
        Instruction::Output,
        Instruction::Right,
        Instruction::Right,
        Instruction::Output,
        Instruction::Left,
        Instruction::Subtract,
        Instruction::Output,
        Instruction::Left,
        Instruction::Output,
        Instruction::Add,
        Instruction::Add,
        Instruction::Add,
        Instruction::Output,
        Instruction::Subtract,
        Instruction::Subtract,
        Instruction::Subtract,
        Instruction::Subtract,
        Instruction::Subtract,
        Instruction::Subtract,
        Instruction::Output,
        Instruction::Subtract,
        Instruction::Subtract,
        Instruction::Subtract,
        Instruction::Subtract,
        Instruction::Subtract,
        Instruction::Subtract,
        Instruction::Subtract,
        Instruction::Subtract,
        Instruction::Output,
        Instruction::Right,
        Instruction::Right,
        Instruction::Add,
        Instruction::Output,
        Instruction::Right,
        Instruction::Add,
        Instruction::Add,
        Instruction::Output,
    ]
}
