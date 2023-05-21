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

    let mut lexer = Lexer::new(input_file_path.to_owned());

    lexer.lex();

    let mut parser = Parser::new(lexer.tokens());

    parser.parse();

    dbg!(parser.parsed());
}
