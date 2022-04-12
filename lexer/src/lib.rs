use std::fmt;
use std::fmt::Formatter;

pub fn lex(input: &str) -> Vec<Token> {
    input
        .split("")
        .filter_map(|t| match t {
            "<" => Some(Token::Left),
            ">" => Some(Token::Right),
            "." => Some(Token::Output),
            "," => Some(Token::Input),
            "[" => Some(Token::OpenLoop),
            "]" => Some(Token::CloseLoop),
            "+" => Some(Token::Add),
            "-" => Some(Token::Subtract),
            _ => None,
        })
        .collect()
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum Token {
    Left,
    Right,
    Add,
    Subtract,
    OpenLoop,
    CloseLoop,
    Output,
    Input,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Token::Left => write!(f, "<"),
            Token::Right => write!(f, ">"),
            Token::Add => write!(f, "+"),
            Token::Subtract => write!(f, "-"),
            Token::OpenLoop => write!(f, "["),
            Token::CloseLoop => write!(f, "]"),
            Token::Output => write!(f, "."),
            Token::Input => write!(f, ","),
        }
    }
}
