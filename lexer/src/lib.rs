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

#[derive(Eq, PartialEq, Debug, Clone)]
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
