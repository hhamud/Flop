use miette::{MietteSpanContents, SourceCode, SourceOffset, SourceSpan};
use std::{fmt, path::PathBuf};
use thiserror::Error;

#[derive(Debug, Clone, Error)]
pub struct Token {
    pub token: String,
    pub token_kind: TokenKind,
    pub row: usize,
    pub offset: usize,
    pub length: usize,
    pub namespace: PathBuf,
}

impl From<Token> for SourceSpan {
    fn from(value: Token) -> Self {
        let source = value.namespace.to_str().unwrap();
        let offset = SourceOffset::from_location(source, value.row, value.offset);
        let length = SourceOffset::from_location(source, value.row, value.length);

        SourceSpan::new(offset, length)
    }
}

impl SourceCode for Token {
    fn read_span<'a>(
        &'a self,
        span: &SourceSpan,
        _context_lines_before: usize,
        _context_lines_after: usize,
    ) -> Result<Box<dyn miette::SpanContents<'a> + 'a>, miette::MietteError> {
        let name: String = self
            .namespace
            .file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("No valid file name")
            .to_string();

        Ok(Box::new(MietteSpanContents::new_named(
            name,
            self.token.as_bytes(),
            *span,
            self.row,
            self.offset,
            0,
        )))
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Token: {}, Kind: {:?}, Row: {},  span: ({}, {}), Namespace: {:?}",
            self.token, self.token_kind, self.row, self.offset, self.length, self.namespace
        )
    }
}

impl Token {
    pub fn new(
        token: &str,
        token_kind: TokenKind,
        row: usize,
        offset: usize,
        length: usize,
        namespace: &PathBuf,
    ) -> Self {
        Self {
            token: token.to_string(),
            token_kind,
            row,
            offset,
            length,
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
