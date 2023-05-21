use std::{fs, os, process};

use crate::{
    ast::{Node, NodeKind},
    tokens::{Token, TokenKind},
};

pub struct Parser {
    pub output_nodes: Vec<Node>,

    input_tokens: Vec<Token>,
    current_token_index: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            input_tokens: tokens,
            current_token_index: 0,
            output_nodes: Vec::new(),
        }
    }

    pub fn parse(&mut self) {
        while self.is_not_eof() {
            match self.current_token().kind {
                TokenKind::Type => {
                    let type_name = self.current_token().value;
                    self.expect_kind(TokenKind::Identifier);
                    self.next();
                    let name = self.current_token().value;

                    self.expect_either(&[TokenKind::OParen, TokenKind::Identifier]);
                    match self.peek().kind {
                        // Function
                        TokenKind::OParen => {
                            self.parse_function();
                        }

                        // Variable
                        TokenKind::Identifier => {
                            todo!()
                        }

                        _ => unreachable!(),
                    }
                }

                other => self.throw_err(format!("Use of unimplemented token kind {:?}", other)),
            }

            self.next();
        }
    }

    fn parse_function(&mut self) {
        self.next();

        let mut parameters: Vec<(String, String)> = Vec::<(String, String)>::new();

        self.expect_either(&[TokenKind::Type, TokenKind::CParen]);
        loop {
            self.next();

            match self.current_token().kind {
                TokenKind::Type => {
                    let param_type = self.current_token().value;
                    self.expect_kind(TokenKind::Identifier);
                    self.next();

                    let param_name = self.current_token().value;
                }

                TokenKind::CParen => {
                    break;
                }

                TokenKind::Comma => {}

                _ => unreachable!(),
            }

            self.expect_either(&[TokenKind::Type, TokenKind::CParen, TokenKind::Comma]);
        }
    }

    fn next(&mut self) {
        self.current_token_index += 1;
    }

    #[inline]
    fn peek(&self) -> Token {
        self.input_tokens[self.current_token_index + 1].to_owned()
    }

    fn expect_kind(&self, expected_kind: TokenKind) {
        let next_token = self.peek();

        if next_token.kind != expected_kind {
            self.throw_err(format!(
                "Expected token kind `{:?}` after `{}`, but found `{:?}`",
                expected_kind,
                self.current_token().value,
                next_token.kind
            ))
        }
    }

    fn expect_either(&self, expected_kinds: &[TokenKind]) {
        let next_token = self.peek();

        if !expected_kinds.contains(&next_token.kind) {
            self.throw_err(format!(
                "Expected either of token kinds `{:?}` after `{}`, but found `{:?}`",
                expected_kinds,
                self.current_token().value,
                next_token.kind
            ))
        }
    }

    fn push_node(&mut self, node: Node) {
        self.output_nodes.push(node);
    }

    #[inline]
    fn is_eof(&self) -> bool {
        self.current_token().kind == TokenKind::Eof
    }

    #[inline]
    fn is_not_eof(&self) -> bool {
        self.current_token().kind != TokenKind::Eof
    }

    #[inline]
    fn current_token(&self) -> Token {
        self.input_tokens[self.current_token_index].to_owned()
    }

    fn throw_err(&self, msg: String) {
        let current_token_location = self.current_token().location;
        let line_number_spaces = " ".repeat(current_token_location.line_number.to_string().len());

        println!("[Error]\n{}\n", msg);
        println!(
            "[Location]: {}:{}:{}",
            current_token_location.source_code_path,
            current_token_location.line_number,
            current_token_location.start_col
        );

        let lines: Vec<String> = fs::read_to_string(current_token_location.source_code_path)
            .unwrap()
            .split("\n")
            .map(String::from)
            .collect();

        println!(" {} |", line_number_spaces);
        println!(
            " {} | {}",
            current_token_location.line_number,
            lines[current_token_location.line_number - 1]
        );
        println!(
            " {} |{}{}",
            line_number_spaces,
            " ".repeat(current_token_location.start_col),
            "^".repeat(current_token_location.end_col - current_token_location.start_col + 1)
        );
        process::exit(1);
    }
}
