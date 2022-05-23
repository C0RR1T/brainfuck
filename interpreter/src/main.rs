use std::path::PathBuf;

use clap::Parser;
use interpreter::Interpreter;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Arguments {
    #[clap(short, long = "no-optimization")]
    pub no_opt: bool,

    pub file: PathBuf,
}

fn main() {
    let args: Arguments = Arguments::parse();

    Interpreter::new().interpret_file(
        args.file.to_str().expect("Expected valid path"),
        !args.no_opt,
    );
}
