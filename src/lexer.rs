use std;

use crate::tokens::{Token, TokenKind, Location};

pub struct Lexer {
    pub output_tokens: Vec<Token>,

    source_code_length: usize,
    source_code_chars: Vec<char>,
    source_code_lines: Vec<String>,
    
    current_char_index: usize,
    line_start_indices: Vec<usize>,
    source_code_path: String,
}

impl Lexer {
    pub fn new(source_code: String, file_path: String) -> Self {     
        let source_code_chars: Vec<char> = source_code.chars().collect();
        Self {
            output_tokens: Vec::new(),

            source_code_length: source_code_chars.len(),
            source_code_chars: source_code_chars,
            source_code_lines: source_code.split("\n").map(String::from).collect(),

            current_char_index: 0,
            line_start_indices: vec![0],
            source_code_path: file_path,
        }
    }

    pub fn lex(&mut self) {
        while self.is_not_eof() {
            if self.current_line().starts_with("//") { self.next(); continue; }

            match self.current_char() {
                c if c.is_whitespace() => {}

                c if c.is_alphabetic() => {
                    self.eat_identifier();
                }

                '"' => {
                    self.eat_string();
                }

                ';' => {
                    self.output_tokens.push(Token {
                        kind: TokenKind::Semicolon,
                        value: ";".to_string(),
                        location: Location {
                            source_code_path: self.source_code_path.clone(),
                            line_number: self.current_line_number(),
                            start_col: self.current_col(),
                            end_col: self.current_col(),
                        }
                    });
                }

                '(' => {
                    self.output_tokens.push(Token {
                        kind: TokenKind::OParen,
                        value: "(".to_string(),
                        location: Location {
                            source_code_path: self.source_code_path.clone(),
                            line_number: self.current_line_number(),
                            start_col: self.current_col(),
                            end_col: self.current_col(),
                        }
                    });
                }

                ')' => {
                    self.output_tokens.push(Token {
                        kind: TokenKind::CParen,
                        value: ")".to_string(),
                        location: Location {
                            source_code_path: self.source_code_path.clone(),
                            line_number: self.current_line_number(),
                            start_col: self.current_col(),
                            end_col: self.current_col(),
                        }
                    });
                }

                '{' => {
                    self.output_tokens.push(Token {
                        kind: TokenKind::OCurly,
                        value: "{".to_string(),
                        location: Location {
                            source_code_path: self.source_code_path.clone(),
                            line_number: self.current_line_number(),
                            start_col: self.current_col(),
                            end_col: self.current_col(),
                        }
                    });
                }

                '}' => {
                    self.output_tokens.push(Token {
                        kind: TokenKind::CCurly,
                        value: "}".to_string(),
                        location: Location {
                            source_code_path: self.source_code_path.clone(),
                            line_number: self.current_line_number(),
                            start_col: self.current_col(),
                            end_col: self.current_col(),
                        }
                    });
                }

                other => {
                    self.throw_err(format!(
                        "Unexpected character '{}'",
                        other
                    ))
                }
            }

            self.next();
        }
    }

    fn next(&mut self) {
        self.current_char_index += 1;

        if self.is_not_eof() && self.current_char() == '\n' {
            self.line_start_indices.push(self.current_char_index + 1);
            self.next();
        }
    }

    fn eat_identifier(&mut self) {
        let start_col = self.current_col();
        let start_line = self.current_line_number();

        let mut eaten_identifier = String::new();

        while self.current_char().is_alphanumeric() {
            eaten_identifier.push(self.current_char());
            if self.is_eof() {
                break;
            }
            self.current_char_index += 1;
        }

        self.current_char_index -= 1;

        self.output_tokens.push(Token {
            kind: TokenKind::Identifier,
            value: eaten_identifier,
            location: Location {
                start_col,
                end_col: self.current_col(),
                line_number: self.current_line_number(),

                source_code_path: self.source_code_path.clone(),
            },
        })
    }

    fn eat_string(&mut self) {
        let start_col = self.current_col();
        let start_line = self.current_line_number();

        let mut eaten_string = String::new();

        self.current_char_index += 1;
        while self.current_char() != '"' {
            eaten_string.push(self.current_char());
            if self.is_eof() {
                self.throw_err_custom_pointer(format!(
                    "Unended string since line {}, column {}\n\n[Help]\n{}",
                    start_line, start_col,
                    "Consider adding a '\"' when the string ends."
                ), start_col);
            }
            self.current_char_index += 1;
        }

        self.output_tokens.push(Token {
            kind: TokenKind::Identifier,
            value: eaten_string,
            location: Location {
                start_col,
                end_col: self.current_col(),
                line_number: self.current_line_number(),

                source_code_path: self.source_code_path.clone(),
            },
        })
    }

    #[inline]
    fn is_not_eof(&self) -> bool {
        self.current_char_index < self.source_code_length
    }

    #[inline]
    fn is_eof(&self) -> bool {
        self.current_char_index + 1 >= self.source_code_length
    }

    #[inline]
    fn current_char(&mut self) -> char {
        self.source_code_chars[self.current_char_index].clone()
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
        let mut arrow_spaces = " ".repeat(current_col);

        println!("[Error]");
        println!("{}\n", msg.into());
        println!(
            "[Location] {}:{}:{}",
            self.source_code_path, current_line_number, current_col
        );
        println!(" {} |", current_line_number_spaces);
        println!(" {} | {}", current_line_number, self.current_line());
        println!(" {} |{}^", current_line_number_spaces, arrow_spaces);

        std::process::exit(1);
    }

    fn throw_err_custom_pointer<T: Into<String>>(&self, msg: T, pointer_position: usize) {
        let current_line_number = self.current_line_number();
        let current_line_number_spaces = " ".repeat(current_line_number.to_string().len());
        let current_col = self.current_col();
        let mut arrow_spaces = " ".repeat(pointer_position);

        println!("[Error]");
        println!("{}\n", msg.into());
        println!(
            "[Location] {}:{}:{}",
            self.source_code_path, current_line_number, current_col
        );
        println!(" {} |", current_line_number_spaces);
        println!(" {} | {}", current_line_number, self.current_line());
        println!(" {} |{}^", current_line_number_spaces, arrow_spaces);

        std::process::exit(1);
    }
}