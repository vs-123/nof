use std::{fs, os, process};

use crate::{
    ast::Node,
    tokens::{Token, TokenKind},
};

pub struct Parser {
    input_tokens: Vec<Token>,
    current_token_index: usize,

    output_nodes: Vec<Node>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            input_tokens: tokens,
            current_token_index: 0,
            output_nodes: Vec::new(),
        }
    }

    pub fn parsed(&self) -> Vec<Node> {
        self.output_nodes.to_owned()
    }

    pub fn parse(&mut self) {
        while self.is_not_eof() {
            match self.current_token().kind {
                TokenKind::Keyword => match self.current_token().value.as_str() {
                    "include" => {
                        self.expect_kind(TokenKind::Identifier);
                        self.advance();

                        let include_thing = self.current_token().value;

                        self.expect_kind(TokenKind::Semicolon);
                        self.advance();

                        self.push_node(Node::Include(include_thing));
                    }

                    "print" => {
                        self.expect_kind(TokenKind::String);
                        self.advance();

                        let print_thing = self.current_token().value;

                        self.expect_kind(TokenKind::Semicolon);
                        self.advance();

                        self.push_node(Node::Print(print_thing));
                    }

                    _ => self.throw_err(format!(
                        "Keyword `{}` not implemented",
                        self.current_token().value
                    )),
                },

                // Functions or variables
                TokenKind::Type => {
                    let thing_type = self.current_token().value;
                    let mut parameters: Vec<(String, String)> = Vec::new();

                    self.expect_kind(TokenKind::Identifier);
                    self.advance();

                    let thing_name = self.current_token().value;

                    match self.peek().kind {
                        // Function
                        TokenKind::OParen => {
                            self.advance();

                            while self.peek().kind != TokenKind::CParen {
                                self.expect_kind(TokenKind::Type);
                                self.advance();

                                let param_type = self.current_token().value;

                                self.expect_kind(TokenKind::Identifier);
                                self.advance();

                                let param_name = self.current_token().value;

                                parameters.push((param_type, param_name));

                                if self.peek().kind == TokenKind::Comma {
                                    self.advance();
                                }
                            }

                            self.expect_kind(TokenKind::CParen);
                            self.advance();

                            self.expect_kind(TokenKind::OCurly);
                            self.advance();

                            // Block

                            let mut body: Vec<Token> = Vec::new();

                            self.advance();

                            while self.current_token().kind != TokenKind::CCurly {
                                body.push(self.current_token());
                                self.advance();
                            }

                            let mut block_parser = Parser::new(body);
                            block_parser.parse();

                            let body = block_parser
                                .parsed()
                                .iter()
                                .map(|e| Box::new(e.clone()))
                                .collect();

                            self.push_node(Node::FuncDecl(thing_type, thing_name, body));
                        }

                        _ => {
                            unreachable!()
                        }
                    }
                }

                _ => self.throw_err(format!(
                    "Token `{}` (type: `{:?}`) not implemented",
                    self.current_token().value,
                    self.current_token().kind
                )),
            }
            if self.current_token_index == self.input_tokens.len() - 1 {
                break;
            }
            self.advance();
        }
    }

    fn advance(&mut self) {
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
                "Expected a/an `{:?}` after `{}`, but found `{:?}`",
                expected_kind,
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
        !self.is_eof()
    }

    #[inline]
    fn current_token(&self) -> Token {
        self.input_tokens[self.current_token_index].to_owned()
    }

    fn throw_err(&self, msg: String) {
        println!("[Error]: {}", msg);
        println!(
            "[Location]: {}:{}",
            self.current_token().location.line_number,
            self.current_token().location.start_col
        );

        let lines: Vec<String> = fs::read_to_string(self.current_token().location.source_code_path)
            .unwrap()
            .split("\n")
            .map(String::from)
            .collect();

        println!(
            " {} | {}",
            self.current_token().location.line_number,
            lines[self.current_token().location.line_number - 1].trim_start()
        );
        process::exit(1);
    }
}
