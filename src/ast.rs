use crate::tokens::{Token, TokenKind};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    Include(String),
    Print(String),
    FuncDecl(String, String, Vec<Box<Node>>),
    ParamList(Vec<(String, String)>),
}
