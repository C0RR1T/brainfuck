use std::fmt;
use std::fmt::Formatter;

pub fn lex(input: &str) -> Vec<LexerToken> {
    input
        .char_indices()
        .filter_map(|(i, t)| match t {
            '<' => Some((i, TokenType::Left)),
            '>' => Some((i, TokenType::Right)),
            '.' => Some((i, TokenType::Output)),
            ',' => Some((i, TokenType::Input)),
            '[' => Some((i, TokenType::OpenLoop)),
            ']' => Some((i, TokenType::CloseLoop)),
            '+' => Some((i, TokenType::Add)),
            '-' => Some((i, TokenType::Subtract)),
            _ => None,
        })
        .map(|(i, t)| LexerToken::new(Span::from(i, i + 1), t))
        .collect()
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub enum TokenType {
    Left,
    Right,
    Add,
    Subtract,
    OpenLoop,
    CloseLoop,
    Output,
    Input,
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct LexerToken {
    pub span: Span,
    pub token: TokenType,
}

impl LexerToken {
    pub fn new(span: Span, token: TokenType) -> Self {
        LexerToken { span, token }
    }
}

#[derive(Eq, PartialEq, Debug, Clone, Copy)]
pub struct Span {
    pub from: usize,
    pub to: usize,
}

impl Span {
    pub fn from(from: usize, to: usize) -> Self {
        Span { from, to }
    }
}

impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Left => write!(f, "<"),
            Self::Right => write!(f, ">"),
            Self::Add => write!(f, "+"),
            Self::Subtract => write!(f, "-"),
            Self::OpenLoop => write!(f, "["),
            Self::CloseLoop => write!(f, "]"),
            Self::Output => write!(f, "."),
            Self::Input => write!(f, ","),
        }
    }
}
