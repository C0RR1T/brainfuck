use lexer::Token;

pub fn parse(tokens: Vec<Token>) -> Vec<Instruction> {
    let mut iter = tokens.iter();
    let mut instructions = Vec::new();

    while let Some(token) = iter.next() {
        match token {
            Token::OpenLoop => {
                let mut loop_content = Vec::new();
                for token in &mut iter {
                    if *token == Token::CloseLoop {
                        break;
                    } else {
                        loop_content.push(parse_token(token))
                    }
                }
                instructions.push(Instruction::Loop(loop_content))
            }
            other => instructions.push(parse_token(other)),
        }
    }

    instructions
}

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
