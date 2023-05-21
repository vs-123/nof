#![allow(warnings)]
use std::{env, fs, process};

mod ast;
pub mod lexer;

mod parser;
pub mod tokens;

use lexer::Lexer;
use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        let program = &args[0];
        println!("[Usage]\n{} <input_file>\n", program);
        println!("[Example]\n{} main.nof\n", program);
        println!("[Provided]\n{} argument(s)", args.len());
        process::exit(1);
    }

    let input_file_path = &args[1];

    let source = {
        if let Ok(source) = fs::read_to_string(input_file_path) {
            source
        } else {
            println!("[Error]\nCould not open file");
            return;
        }
    };

    let mut lexer = Lexer::new(source.to_owned(), input_file_path.clone());

    lexer.lex();

    // dbg!(lexer.tokens());

    let mut parser = Parser::new(lexer.output_tokens);

    parser.parse();

    dbg!(parser.output_nodes);
}
