use std::{fs, process};

use crate::tokens::{Location, Token, TokenKind};

const KEYWORDS: [&str; 2] = ["include", "print"];
const TYPES: [&str; 1] = ["void"];

pub struct Lexer {
    source_code_lines: Vec<String>,
    source_code_chars: Vec<char>,
    tokens: Vec<Token>,
    current_char_index: usize,

    // Location
    source_code_path: String,
    line_start_indices: Vec<usize>,
}

impl Lexer {
    pub fn new(source_code_path: String) -> Self {
        match fs::read_to_string(&source_code_path) {
            Ok(source_code) => {
                let source_code = source_code.replace("\r", "");
                Self {
                    source_code_lines: source_code.split("\n").map(String::from).collect(),
                    source_code_chars: source_code.chars().collect(),
                    tokens: Vec::new(),
                    current_char_index: 0,

                    source_code_path,
                    line_start_indices: vec![0],
                }
            }

            Err(err) => {
                println!("[Error] Could not read input file `{}`", source_code_path);
                println!("[Reason] {}", err);
                process::exit(1);
            }
        }
    }

    pub fn tokens(&self) -> Vec<Token> {
        self.tokens.to_owned()
    }

    pub fn lex(&mut self) {
        while self.is_not_eof() {
            match self.current_char() {
                // Identifiers
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut eaten_identifier = String::new();
                    let start_col = self.current_col();
                    while self.current_char().is_alphanumeric() {
                        eaten_identifier.push(self.current_char());
                        self.advance();
                    }
                    let end_col = self.current_col();

                    self.current_char_index -= 1;

                    let mut kind = TokenKind::Identifier;

                    if KEYWORDS.contains(&eaten_identifier.as_str()) {
                        kind = TokenKind::Keyword;
                    } else if TYPES.contains(&eaten_identifier.as_str()) {
                        kind = TokenKind::Type;
                    }

                    self.push_token(kind, eaten_identifier, start_col, end_col);
                }

                // Strings
                '"' => {
                    let mut eaten_string = String::new();
                    let start_col = self.current_col();

                    self.advance();
                    while self.current_char() != '"' {
                        eaten_string.push(self.current_char());

                        self.advance();
                        if self.is_eof() {
                            self.throw_err(format!(
                                    "Unended string since column {} at line {}",
                                    start_col, self.current_line_number()
                            ));
                        }
                    }

                    let end_col = self.current_col();

                    self.push_token(TokenKind::String, eaten_string, start_col, end_col);
                }

                ////////////////////////////////////////////////////
                ';' => {
                    self.push_token(
                        TokenKind::Semicolon,
                        self.current_char().to_string(),
                        self.current_col(),
                        self.current_col(),
                    );
                }

                ',' => {
                    self.push_token(
                        TokenKind::Comma,
                        self.current_char().to_string(),
                        self.current_col(),
                        self.current_col(),
                    );
                }
                '(' => {
                    self.push_token(
                        TokenKind::OParen,
                        self.current_char().to_string(),
                        self.current_col(),
                        self.current_col(),
                    );
                }

                ')' => {
                    self.push_token(
                        TokenKind::CParen,
                        self.current_char().to_string(),
                        self.current_col(),
                        self.current_col(),
                    );
                }

                '{' => {
                    self.push_token(
                        TokenKind::OCurly,
                        self.current_char().to_string(),
                        self.current_col(),
                        self.current_col(),
                    );
                }

                '}' => {
                    self.push_token(
                        TokenKind::CCurly,
                        self.current_char().to_string(),
                        self.current_col(),
                        self.current_col(),
                    );
                }

                ////////////////////////////////////////////////////
                '\n' => {
                    self.line_start_indices.push(self.current_char_index);
                }
                
                c if c.is_whitespace() => {}

                _ => self.throw_err(format!("Unknown character `{}`", self.current_char())),
            }

            self.advance();
        }

        self.push_token(
            TokenKind::Eof,
            String::new(),
            self.current_col(),
            self.current_col(),
        )
    }

    fn is_eof(&self) -> bool {
        self.current_char_index == self.source_code_chars.len()
    }

    fn is_not_eof(&self) -> bool {
        !self.is_eof()
    }

    #[inline]
    fn current_char(&self) -> char {
        self.source_code_chars[self.current_char_index].to_owned()
    }

    fn advance(&mut self) {
        self.current_char_index += 1;
    }

    fn push_token(&mut self, kind: TokenKind, value: String, start_col: usize, end_col: usize) {
        self.tokens.push(Token {
            kind,
            value,
            location: Location {
                source_code_path: self.source_code_path.clone(),
                line_number: self.current_line_number(),
                start_col,
                end_col,
            },
        })
    }

    #[inline]
    fn current_col(&self) -> usize {
        self.current_char_index - self.line_start_indices.last().unwrap() + 1
    }

    #[inline]
    fn current_line_number(&self) -> usize {
        self.line_start_indices.len()
    }

    #[inline]
    fn current_line(&self) -> String {
        self.source_code_lines[self.current_line_number() - 1].clone()
    }

    fn throw_err<T: Into<String>>(&self, msg: T) {
        let current_line_number = self.current_line_number();
        let current_line_number_spaces = " ".repeat(current_line_number.to_string().len());
        let current_col = self.current_col();
        let arrow_spaces = " ".repeat(current_col - 1);

        println!("[Error]");
        println!("{}\n", msg.into());
        println!(
            "[Location] {}:{}:{}",
            self.source_code_path, current_line_number, current_col
        );
        println!(" {} |", current_line_number_spaces);
        println!(" {} | {}", current_line_number, self.current_line(),);
        println!(" {} | {}^", current_line_number_spaces, arrow_spaces);

        std::process::exit(1);
    }
}
