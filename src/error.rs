use crate::lexer::Token;

#[derive(Debug)]
pub struct TokenError {
    pub message: &'static str,
    pub token: Token,
}

#[derive(Debug)]
pub enum ParseError {
    InputError(&'static str),
    TokenError(TokenError),
}

#[derive(Debug)]
pub enum ErrorType {
    FunctionDefinition,
    VariableName,
    VariableValue,
    Parameter,
    List,
    Expression,
}

fn generate_error_message(error_type: ErrorType) -> &'static str {
    match error_type {
        ErrorType::VariableName => "Variable Definition: Expected a variable name",
        ErrorType::VariableValue => "Variable assignment: Expected a variable value",
        ErrorType::Parameter => "Parameter: Unexpected token",
        ErrorType::List => "List: Unexpected token",
        ErrorType::Expression => "Expression: Unexpected token",
        ErrorType::FunctionDefinition => "Function Definition: Unexpected token",
    }
}
