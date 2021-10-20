use std::env::args;

mod cmd;
mod token;
mod parser;
mod compiler;
mod vm;
mod object;

fn main() {
    if args().len() <= 1 {
        cmd::help();
    }

    let file = match args().nth(1) {
        Some(f) => f,
        None => {
            eprintln!("Please provide a file path.");
            std::process::exit(1);
        }
    };

    let contents = match std::fs::read_to_string(file) {
        Ok(s) => s,
        Err(_) => {
            eprintln!("Unable to open file.");
            std::process::exit(1);
        }
    };

    let tokens = token::generate(&contents);
    let ast = parser::parse(tokens);
    let (constants, code) = compiler::compile(ast);

    #[cfg(debug_assertions)]
    dbg!(&constants, &code);
    
    vm::run(constants, code);
}