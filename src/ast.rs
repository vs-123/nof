use crate::tokens::{Token, TokenKind, Location};

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
    String(String),
    Identifier(String),

    Include(String),
    Print(String),
    FuncDecl(String, String, Vec<Box<Node>>),
    FuncCall(String, Vec<Box<NodeKind>>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    pub location: Location,
}