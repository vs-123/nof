pub const KEYWORDS: [&str; 1] = ["include"];
pub const TYPES: [&str; 1] = ["void"];

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Identifier,
    String,
    Number,

    Keyword,

    Type,

    OParen,
    CParen,
    OCurly,
    CCurly,

    Semicolon,
    Comma,

    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub kind: TokenKind,
    pub value: String,
    pub location: Location,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Location {
    pub source_code_path: String,
    pub line_number: usize,
    pub start_col: usize,
    pub end_col: usize,
}

impl Location {
    pub fn new(source_code_path: String) -> Self {
        Self {
            source_code_path,
            line_number: 1,
            start_col: 0,
            end_col: 0,
        }
    }
}
