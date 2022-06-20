use clap::Parser as ArgParser;
use include_dir::{include_dir, Dir};
use interpreter::Interpreter;
use lexer::{LexerToken, Span, TokenType};
use parser::Parser;
use peekmore::{PeekMore, PeekMoreIterator};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::stdout;
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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Tokens {
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

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Settings {
    #[serde(rename = "ignoreWhiteSpace")]
    ignore_whitespace: bool,
}

#[derive(Serialize, Deserialize, Clone)]
struct LangFile {
    tokens: Tokens,
    settings: Settings,
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

    let mut tokens: Tokens = lang_file.tokens;

    if lang_file.settings.ignore_whitespace {
        tokens.output = remove_whitespace(&tokens.output);
        tokens.input = remove_whitespace(&tokens.input);
        tokens.left = remove_whitespace(&tokens.left);
        tokens.right = remove_whitespace(&tokens.right);
        tokens.minus = remove_whitespace(&tokens.minus);
        tokens.plus = remove_whitespace(&tokens.plus);
        tokens.open_loop = remove_whitespace(&tokens.open_loop);
        tokens.close_loop = remove_whitespace(&tokens.close_loop);
    }

    let tokens = Lexer::new(tokens, &source_code, lang_file.settings.ignore_whitespace).parse();

    println!("{:?}", tokens);

    match Parser::new(tokens).parse() {
        Ok(tokens) => {
            Interpreter::new(stdout()).interpret(&tokens);
        }
        Err(err) => error_messages::print_error(&err, &source_code),
    }
}

struct Lexer<'a> {
    language: Tokens,
    code: PeekMoreIterator<CharIndices<'a>>,
    first_char: FirstChar,
    ignore_whitespace: bool,
}

impl<'a> Lexer<'a> {
    fn new(language: Tokens, code: &'a str, ignore_whitespace: bool) -> Self {
        println!("{:?}", language);
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
            ignore_whitespace,
            language,
        }
    }

    fn parse(&mut self) -> Vec<LexerToken> {
        let mut tokens = Vec::new();
        while let Some((i, char)) = self.next() {
            let (matches, amount) = self.compute_token(
                &char,
                &self.first_char.left.clone(),
                &self.language.left.clone(),
            );

            if matches {
                tokens.push(LexerToken::new(Span::from(i, i + amount), TokenType::Left));
                break;
            }

            let (matches, amount) = self.compute_token(
                &char,
                &self.first_char.right.clone(),
                &self.language.right.clone(),
            );

            if matches {
                tokens.push(LexerToken::new(Span::from(i, i + amount), TokenType::Right));
                break;
            }

            let (matches, amount) = self.compute_token(
                &char,
                &self.first_char.plus.clone(),
                &self.language.plus.clone(),
            );

            if matches {
                tokens.push(LexerToken::new(Span::from(i, i + amount), TokenType::Add));
                break;
            }

            let (matches, amount) = self.compute_token(
                &char,
                &self.first_char.minus.clone(),
                &self.language.minus.clone(),
            );

            if matches {
                tokens.push(LexerToken::new(
                    Span::from(i, i + amount),
                    TokenType::Subtract,
                ));
                break;
            }

            let (matches, amount) = self.compute_token(
                &char,
                &self.first_char.input.clone(),
                &self.language.input.clone(),
            );

            if matches {
                tokens.push(LexerToken::new(Span::from(i, i + amount), TokenType::Input));
                break;
            }

            let (matches, amount) = self.compute_token(
                &char,
                &self.first_char.output.clone(),
                &self.language.output.clone(),
            );

            if matches {
                tokens.push(LexerToken::new(
                    Span::from(i, i + amount),
                    TokenType::Output,
                ));
                break;
            }

            let (matches, amount) = self.compute_token(
                &char,
                &self.first_char.open_loop.clone(),
                &self.language.open_loop.clone(),
            );

            if matches {
                tokens.push(LexerToken::new(
                    Span::from(i, i + amount),
                    TokenType::OpenLoop,
                ));
                break;
            }

            let (matches, amount) = self.compute_token(
                &char,
                &self.first_char.close_loop.clone(),
                &self.language.close_loop.clone(),
            );

            if matches {
                tokens.push(LexerToken::new(
                    Span::from(i, i + amount),
                    TokenType::CloseLoop,
                ));
                break;
            }
        }
        tokens
    }

    fn compute_token(&mut self, char: &char, first_char: &char, to_check: &str) -> (bool, usize) {
        if char == first_char {
            let (str, amount) = self.peek_string(to_check.len() - 1);

            let matches = check_if_string_equals(&str, char, to_check);
            println!("{} == {}{}: {}", to_check, char, str, matches);
            assert_eq!(format!("{}{}", char, str).len(), to_check.len());
            if matches {
                self.consume_elements(amount);
            }
            return (matches, amount);
        }
        (false, 0)
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

    fn peek_string(&mut self, length: usize) -> (String, usize) {
        let mut string = String::new();
        let mut amount = 0;
        let ignore_whitespace = self.ignore_whitespace;
        while string.len() < length {
            if let Some((_, char)) = self.peek_nth(amount) {
                if !ignore_whitespace || !char.is_whitespace() && *char != '\n' {
                    string.push(*char)
                }
                amount += 1;
            } else {
                break;
            }
        }

        (string, amount)
    }

    fn consume_elements(&mut self, amount: usize) {
        for _ in 0..amount {
            self.next();
        }
    }

    fn advance_cursor(&mut self) -> Option<&(usize, char)> {
        self.code.advance_cursor();
        self.peek()
    }
}

fn get_first_char(s: &str) -> char {
    *s.chars().peekable().peek().unwrap()
}

fn check_if_string_equals(other_str: &str, char: &char, to_check: &str) -> bool {
    format!("{}{}", *char, other_str) == to_check
}

fn remove_whitespace(s: &str) -> String {
    s.chars()
        .filter(|x| !x.is_whitespace() && *x != '\n')
        .collect()
}
