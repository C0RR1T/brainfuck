use clap::Parser as ArgParser;
use include_dir::{include_dir, Dir};
use interpreter::Interpreter;
use lexer::{LexerToken, Span, TokenType};
use parser::Parser;
use peekmore::{PeekMore, PeekMoreIterator};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::str::CharIndices;
use std::string::String;

static LANG_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/langs");

#[derive(Debug, ArgParser)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short, long)]
    lang: String,
    file: PathBuf,
}

#[derive(Serialize, Deserialize, Debug)]
struct LangFile {
    left: String,
    right: String,

    #[serde(rename = "openLoop")]
    open_loop: String,

    #[serde(rename = "closeLoop")]
    close_loop: String,
    output: String,
    input: String,
    plus: String,
    minus: String,
}

struct FirstChar {
    left: char,
    right: char,
    plus: char,
    minus: char,
    open_loop: char,
    close_loop: char,
    input: char,
    output: char,
}

fn main() {
    let args: Args = Args::parse();

    let lang_file = LANG_DIR
        .get_file(format!("{}.json", args.lang))
        .expect("Language does not exist.");

    let source_code = fs::read_to_string(args.file).expect("File not found");

    let lang_file = serde_json::from_str::<LangFile>(lang_file.contents_utf8().unwrap()).unwrap();

    let tokens = Lexer::new(lang_file, &source_code).parse();

    match Parser::new(tokens).parse() {
        Ok(tokens) => {
            Interpreter::new().interpret(&tokens);
        }
        Err(err) => error_messages::print_error(&err, &source_code),
    }
}

struct Lexer<'a> {
    language: LangFile,
    code: PeekMoreIterator<CharIndices<'a>>,
    first_char: FirstChar,
}

impl<'a> Lexer<'a> {
    fn new(language: LangFile, code: &'a str) -> Self {
        Lexer {
            code: code.char_indices().peekmore(),
            first_char: FirstChar {
                left: get_first_char(&language.left),
                right: get_first_char(&language.right),
                plus: get_first_char(&language.plus),
                minus: get_first_char(&language.minus),
                open_loop: get_first_char(&language.open_loop),
                close_loop: get_first_char(&language.close_loop),
                input: get_first_char(&language.input),
                output: get_first_char(&language.output),
            },
            language,
        }
    }

    fn parse(&mut self) -> Vec<LexerToken> {
        let mut tokens = Vec::new();
        while let Some((i, char)) = self.next() {
            if char == self.first_char.left
                && check_if_string_equals(
                    &self.peek_string(&self.language.left.len() - 1),
                    &char,
                    &self.language.left,
                )
            {
                self.consume_elements(self.language.left.len() - 1);
                tokens.push(LexerToken::new(
                    Span::from(i, self.language.left.len() - 1),
                    TokenType::Left,
                ))
            } else if char == self.first_char.right
                && check_if_string_equals(
                    &self.peek_string(&self.language.right.len() - 1),
                    &char,
                    &self.language.left,
                )
            {
                self.consume_elements(&self.language.right.len() - 1);
                tokens.push(LexerToken::new(
                    Span::from(i, self.language.right.len() - 1),
                    TokenType::Right,
                ))
            } else if char == self.first_char.plus
                && check_if_string_equals(
                    &self.peek_string(&self.language.plus.len() - 1),
                    &char,
                    &self.language.plus,
                )
            {
                self.consume_elements(&self.language.plus.len() - 1);
                tokens.push(LexerToken::new(
                    Span::from(i, self.language.plus.len() - 1),
                    TokenType::Add,
                ));
            } else if char == self.first_char.minus
                && check_if_string_equals(
                    &self.peek_string(&self.language.minus.len() - 1),
                    &char,
                    &self.language.minus,
                )
            {
                self.consume_elements(&self.language.minus.len() - 1);
                tokens.push(LexerToken::new(
                    Span::from(i, self.language.minus.len() - 1),
                    TokenType::Subtract,
                ));
            } else if char == self.first_char.open_loop
                && check_if_string_equals(
                    &self.peek_string(&self.language.open_loop.len() - 1),
                    &char,
                    &self.language.open_loop,
                )
            {
                self.consume_elements(&self.language.open_loop.len() - 1);
                tokens.push(LexerToken::new(
                    Span::from(i, self.language.open_loop.len() - 1),
                    TokenType::OpenLoop,
                ));
            } else if char == self.first_char.close_loop
                && check_if_string_equals(
                    &self.peek_string(&self.language.close_loop.len() - 1),
                    &char,
                    &self.language.close_loop,
                )
            {
                self.consume_elements(&self.language.close_loop.len() - 1);
                tokens.push(LexerToken::new(
                    Span::from(i, self.language.close_loop.len() - 1),
                    TokenType::CloseLoop,
                ));
            } else if char == self.first_char.input
                && check_if_string_equals(
                    &self.peek_string(&self.language.input.len() - 1),
                    &char,
                    &self.language.input,
                )
            {
                self.consume_elements(&self.language.input.len() - 1);
                tokens.push(LexerToken::new(
                    Span::from(i, self.language.input.len() - 1),
                    TokenType::Input,
                ));
            } else if char == self.first_char.output
                && check_if_string_equals(
                    &self.peek_string(&self.language.output.len() - 1),
                    &char,
                    &self.language.output,
                )
            {
                self.consume_elements(&self.language.output.len() - 1);
                tokens.push(LexerToken::new(
                    Span::from(i, self.language.output.len() - 1),
                    TokenType::Input,
                ));
            }
        }
        tokens
    }

    fn next(&mut self) -> Option<(usize, char)> {
        self.code.next()
    }
    fn peek(&mut self) -> Option<&(usize, char)> {
        self.code.peek()
    }

    fn peek_nth(&mut self, amount: usize) -> Option<&(usize, char)> {
        self.code.peek_nth(amount)
    }

    fn peek_string(&mut self, length: usize) -> String {
        let mut string = String::new();
        for x in 0..length {
            if let Some((_, char)) = self.peek_nth(x) {
                string.push(*char)
            } else {
                break;
            }
        }
        string
    }

    fn consume_elements(&mut self, amount: usize) {
        for _ in 0..amount {
            self.next();
        }
    }
}

fn get_first_char(s: &str) -> char {
    *s.chars().peekable().peek().unwrap()
}

fn check_if_string_equals(other_str: &str, char: &char, to_check: &str) -> bool {
    format!("{}{}", *char, other_str) == to_check
}
