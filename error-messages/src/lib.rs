use lexer::Span;
use owo_colors::OwoColorize;
use parser::ParserError;

pub fn print_error(err: &ParserError, input: &str) {
    match err {
        ParserError::UnexpectedEOF(span) => {
            print_error_line("End of file before loop close");
            print_location(span, input, "close the loop before the file ends")
        }
        ParserError::UnexpectedClosing(span) => {
            print_error_line("Unexpected closing of loop");
            print_location(span, input, "Remove the unnecessary \"]\"");
        }
    }
}

fn print_error_line(message: &str) {
    eprintln!("{}: {}", "Error".bold().bright_red(), message);
}

fn print_location(span: &Span, input: &str, note: &str) {
    let line = get_line_of_error(span, input);
    let info_string = format!("{} | ", line + 1);
    let code_line = get_line(line, input);
    eprintln!("{} {}", " ".repeat(info_string.len() - 3), "|".blue());
    eprintln!("{}{}", info_string.blue(), code_line);
    eprintln!(
        "{} {}{}{}",
        " ".repeat(info_string.len() - 3),
        "|".blue(),
        " ".repeat(code_line.split_at(span.from).0.len() + 1),
        "^".repeat(span.to - span.from).bright_red(),
    );
    eprintln!(
        "{}{} {}: {}",
        " ".repeat(info_string.len() - 2),
        "=".blue(),
        "note".bold(),
        note
    )
}

fn get_line(line: usize, input: &str) -> &str {
    input.lines().nth(line).unwrap()
}

fn get_line_of_error(span: &Span, input: &str) -> usize {
    input
        .char_indices()
        .filter(|(i, char)| *i < span.from && *char == '\n')
        .count()
}
