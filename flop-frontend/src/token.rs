use ariadne::Span;
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

impl Span for Token {
    type SourceId = PathBuf;

    fn source(&self) -> &PathBuf {
        &self.namespace
    }

    fn start(&self) -> usize {
        self.column.start
    }

    fn end(&self) -> usize {
        self.column.end
    }

    fn len(&self) -> usize {
        self.column.start - self.column.end
    }

    fn contains(&self, offset: usize) -> bool {
        todo!()
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

#[derive(Debug, PartialEq)]
pub struct TokenError {
    pub expected: &'static str,
    pub found: &'static str,
    pub token: Token,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token: String,
    pub token_kind: TokenKind,
    pub row: usize,
    pub column: Line,
    pub namespace: &'static PathBuf,
}

impl Token {
    pub fn new(
        token: &str,
        token_kind: TokenKind,
        row: usize,
        column: Line,
        namespace: PathBuf,
    ) -> Self {
        Self {
            token: token.to_string(),
            token_kind,
            row,
            column,
            namespace: &namespace,
        }
    }
}
