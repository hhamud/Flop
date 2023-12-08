use miette::{SourceOffset, SourceSpan};
use std::{fmt, path::PathBuf};
use thiserror::Error;

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

#[derive(Debug, PartialEq, Clone, Error)]
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

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the token for display. This is a simple example, adjust as needed.
        write!(
            f,
            "Token: {}, Kind: {:?}, Row: {}, Column: ({}, {}), Namespace: {:?}",
            self.token,
            self.token_kind,
            self.row,
            self.column.start,
            self.column.end,
            self.namespace
        )
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
