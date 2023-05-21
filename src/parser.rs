use std::{fs, os, process};

use crate::{
    ast::{Node, NodeKind},
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

                        self.push_node(Node {
                            kind: NodeKind::Include(include_thing),
                            location: self.current_token().location,
                        });
                    }

                    _ => self.throw_err(format!(
                        "Keyword `{}` not implemented",
                        self.current_token().value
                    )),
                },

                // Function or variable **declaration**
                // E.g. void main() {
                //      ^^^^
                // Or
                // int x = 5;
                // ^^^
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

                            self.push_node(Node {
                                kind: NodeKind::FuncDecl(thing_type, thing_name, body),
                                location: self.current_token().location,
                            });
                        }

                        _ => self.throw_err(format!(
                            "Eat my dust lol",
                        )),
                    }
                }

                // Function calls and Identifiers
                // E.g. print();
                //      ^^^^^
                TokenKind::Identifier => {
                    let function_name = self.current_token().value;

                    self.expect_kind(TokenKind::OParen);
                    self.advance();

                    let mut parameters = Vec::new();

                    while self.peek().kind != TokenKind::CParen {
                        self.expect_either(&[TokenKind::Identifier, TokenKind::String]);
                        self.advance();

                        let parameter = Box::new(self.match_token_kind(self.current_token()));

                        parameters.push(parameter);

                        if self.peek().kind == TokenKind::Comma {
                            self.advance();
                        }
                    }

                    self.advance();
                    self.expect_kind(TokenKind::Semicolon);
                    self.advance();

                    self.output_nodes.push(Node {
                        kind: NodeKind::FuncCall(function_name, parameters),
                        location: self.current_token().location,
                    });
                }

                _ => self.throw_err(format!(
                    "Token `{}` (kind: `{:?}`) not implemented",
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

    fn match_token_kind(&self, token: Token) -> NodeKind {
        match token.kind {
            TokenKind::Identifier => NodeKind::Identifier(token.value),

            other => {
                self.throw_err(format!(
                    "Expected either of kinds Identifier, String, etc., found {:?}",
                    other
                ));

                unreachable!();
            }
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
        !self.is_eof()
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
