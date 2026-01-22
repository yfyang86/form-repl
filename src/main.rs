mod ast;
mod evaluator;
mod lexer;
mod parser;
mod repl;

use repl::Repl;

fn main() {
    let mut repl = match Repl::new() {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to initialize REPL: {}", e);
            std::process::exit(1);
        }
    };

    if let Err(e) = repl.run() {
        eprintln!("REPL error: {}", e);
        std::process::exit(1);
    }
}
