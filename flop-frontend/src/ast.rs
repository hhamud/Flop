use crate::{
    error::ParseError,
    stack::Stack,
    token::{Token, TokenKind},
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    pub name: Token,
    pub parameters: Stack<Node>,
    pub docstrings: Option<Token>,
    pub body: Stack<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Conditional {
    // some sort of function call ends in a bool
    pub condition: Stack<Node>,
    pub true_expression: Stack<Node>,
    pub false_expression: Stack<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDefinition {
    pub name: Token,
    pub assignment: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionCall {
    name: Token,
    arguments: Stack<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableCall {
    name: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Documentation {
    commentary: Token,
    code: Option<Token>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    String(Token),
    Integer(Token),
    Bool(Token),
    FunctionCall(FunctionCall),
    FunctionDefinition(FunctionDefinition),
    VariableDefinition(VariableDefinition),
    VariableCall(VariableCall),
}

fn parse(tokens: &mut Stack<Token>) -> Result<Stack<Node>, ParseError> {
    let mut stack: Stack<Node> = Stack::new();

    while let Some(token) = tokens.pop_front() {
        match token.token_kind {
            TokenKind::VariableDefinition => {
                // pop Node::Expression
                let _exp = tokens.pop_front();

                let var_token = tokens.pop_front().ok_or(ParseError::StackError(
                    "Failed to pop stack for variable name",
                ))?;

                let var_name = match var_token.token_kind {
                    TokenKind::Symbol => var_token,
                    _ => {
                        return Err(ParseError::TokenError {
                            message: "Variable Definition: Expected a variable name",
                            token: var_token,
                        })
                    }
                };

                let value_token = tokens.pop_front().ok_or(ParseError::StackError(
                    "Failed to pop stack for variable reference",
                ))?;

                let value = match value_token.token_kind {
                    TokenKind::Integer => value_token,
                    TokenKind::Bool => value_token,
                    TokenKind::StringLiteral => value_token,
                    //TODO: Add ability to assign expressions to variables
                    _ => {
                        return Err(ParseError::TokenError {
                            message: "Variable assignment: Expected a variable value",
                            token: value_token,
                        })
                    }
                };

                let var = VariableDefinition {
                    name: var_name,
                    assignment: value,
                };

                stack.push(Node::VariableDefinition(var))
            }

            TokenKind::LeftRoundBracket => {
                let mut arg_vec: Stack<Node> = Stack::new();
                while let Some(token_arg) = tokens.pop_front() {
                    match token_arg.token_kind {
                        TokenKind::Bool => arg_vec.push(Node::Bool(token_arg)),
                        TokenKind::Integer => arg_vec.push(Node::Integer(token_arg)),
                        TokenKind::StringLiteral => arg_vec.push(Node::String(token_arg)),
                        TokenKind::LeftRoundBracket => {
                            todo!()
                        }
                        TokenKind::RightRoundBracket => break,
                        //TODO: add function calls within function calls
                        _ => return Err(ParseError::TokenError {
                            message:
                                "Function argument is not of the correct type: NUMBER, STRING, BOOL",
                            token: token_arg,
                        }),
                    }
                }

                let fc = FunctionCall {
                    name: token,
                    arguments: arg_vec,
                };

                stack.push(Node::FunctionCall(fc))
            }

            TokenKind::LeftSquareBracket => {
                let mut list_args: Stack<Node> = Stack::new();
                while let Some(list_arg) = tokens.pop_front() {
                    match list_arg.token_kind {
                        TokenKind::Bool => list_args.push(Node::Bool(list_arg)),
                        TokenKind::Integer => list_args.push(Node::Integer(list_arg)),
                        TokenKind::StringLiteral => list_args.push(Node::String(list_arg)),
                        _ => {
                            return Err(ParseError::TokenError {
                                message: "List can only contain NUMBER, STRING OR BOOLEAN",
                                token: list_arg,
                            })
                        }
                    }
                }

                stack.push(list_args);
            }

            TokenKind::FunctionDefinition => {
                let mut function_args = Stack::new();
                while let Some(fa) = tokens.pop_front() {}
            }

            TokenKind::Conditional => continue,

            _ => {
                return Err(ParseError::TokenError {
                    message: "Wrong token",
                    token,
                });
            }
        }
    }

    Ok(stack)
}
