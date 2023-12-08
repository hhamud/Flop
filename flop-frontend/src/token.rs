use miette::{SourceOffset, SourceSpan};
use std::path::PathBuf;

#[derive(Debug, PartialEq, Clone)]
pub struct Line {
    pub start: usize,
    pub end: usize,
}

impl Line {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token: String,
    pub token_kind: TokenKind,
    pub row: usize,
    pub column: Line,
    pub namespace: PathBuf,
}

impl From<Token> for SourceSpan {
    fn from(value: Token) -> Self {
        //TODO: check the length sourceoffset implementation
        // maybe this just means the length of the code
        let source = value.namespace.to_str().unwrap();
        let offset = SourceOffset::from_location(source, value.row, value.column.start);
        let length = SourceOffset::from_location(source, value.row, value.column.start);

        SourceSpan::new(length, offset)
    }
}

impl Token {
    pub fn new(
        token: &str,
        token_kind: TokenKind,
        row: usize,
        column: Line,
        namespace: &PathBuf,
    ) -> Self {
        Self {
            token: token.to_string(),
            token_kind,
            row,
            column,
            namespace: namespace.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenKind {
    Space,
    Comment,
    Integer,
    Symbol,
    StringLiteral,
    Bool,
    Conditional,
    LeftRoundBracket,
    RightRoundBracket,
    LeftSquareBracket,
    RightSquareBracket,
    FunctionDefinition,
    VariableDefinition,
    DocString,
    Error,
    Eof,
}
