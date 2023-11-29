use crate::{
    error::ParseError,
    stack::Stack,
    token::{Token, TokenKind},
};

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    pub name: Token,
    pub parameters: Stack<Token>,
    pub docstrings: Token,
    pub body: Stack<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Conditional {
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
    pub name: Token,
    pub arguments: Stack<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct VariableCall {
    pub name: Token,
}

#[derive(Debug, Clone, PartialEq)]
pub struct List {
    pub data: Stack<Node>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Documentation {
    pub commentary: Token,
    pub code: Option<Stack<Node>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    FunctionDefinition(FunctionDefinition),
    Conditional(Conditional),
    VariableDefinition(VariableDefinition),
    FunctionCall(FunctionCall),
    Literal(Token),
    VariableCall(VariableCall),
    List(List),
    Documentation(Documentation),
}

fn parse_variable_definition(tokens: &mut Stack<Token>) -> Result<Node, ParseError> {
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

    Ok(Node::VariableDefinition(var))
}

fn parse_list(tokens: &mut Stack<Token>) -> Result<Node, ParseError> {
    let mut list_args: Stack<Node> = Stack::new();

    let mut nested_level = 0;

    while let Some(list_arg) = tokens.pop_front() {
        match list_arg.token_kind {
            TokenKind::Bool | TokenKind::Integer | TokenKind::StringLiteral => {
                list_args.push(Node::Literal(list_arg))
            }
            TokenKind::LeftSquareBracket => {
                nested_level += 1;
                let res = parse_list(tokens)?;
                list_args.push(res)
            }
            TokenKind::RightSquareBracket => {
                nested_level -= 1;
                if nested_level == 0 {
                    break;
                }
            }
            _ => {
                return Err(ParseError::TokenError {
                    message: "LIST can only contain NUMBER, STRING, BOOLEAN OR another LIST",
                    token: list_arg,
                })
            }
        }
    }

    let node = List { data: list_args };

    Ok(Node::List(node))
}

fn parse_function_definition(tokens: &mut Stack<Token>) -> Result<Node, ParseError> {
    let name = tokens
        .pop_front()
        .ok_or(ParseError::StackError("No function name Token in Stack"))
        .and_then(|name| match name.token_kind {
            TokenKind::Symbol => Ok(name),
            _ => {
                return Err(ParseError::TokenError {
                    message: "Function name must be a symbol",
                    token: name,
                })
            }
        })?;

    let mut parameters: Stack<Token> = Stack::new();

    while let Some(function_arg) = tokens.pop_front() {
        match function_arg.token_kind {
            TokenKind::Symbol => parameters.push(function_arg),
            _ => {
                return Err(ParseError::TokenError {
                    message: "Function Parameter must be a SYMBOL",
                    token: function_arg,
                })
            }
        }
    }

    let docstrings = tokens
        .pop_front()
        .ok_or(ParseError::StackError("No DocString Token in Stack"))
        .and_then(|doc_string| match name.token_kind {
            TokenKind::StringLiteral => Ok(doc_string),
            _ => {
                return Err(ParseError::TokenError {
                    message: "Function name must be a symbol",
                    token: doc_string,
                })
            }
        })?;

    // function body
    // Check if the first token is a LeftRoundBracket
    if let Some(first_token) = tokens.pop_front() {
        if first_token.token_kind != TokenKind::LeftRoundBracket {
            return Err(ParseError::TokenError {
                message: "Function body must start with a LeftRoundBracket",
                token: first_token,
            });
        }

        // Process the first LeftRoundBracket
        let mut body: Stack<Node> = Stack::new();

        let res = parse_expression(tokens)?;

        body.push(res);

        let fd = FunctionDefinition {
            name,
            parameters,
            docstrings,
            body,
        };
        return Ok(Node::FunctionDefinition(fd));
    } else {
        // Handle the case when tokens.pop_front() returns None
        return Err(ParseError::StackError("Function body stack is empty"));
    }
}

fn parse_conditional(tokens: &mut Stack<Token>) -> Result<Node, ParseError> {
    todo!()
}

fn parse_var_call(tokens: &mut Stack<Token>) -> Result<Node, ParseError> {
    let assignment = tokens
        .pop_front()
        .ok_or(ParseError::StackError("No variable symbol in Stack"))
        .and_then(|name| match name.token_kind {
            TokenKind::Symbol => Ok(name),
            _ => {
                return Err(ParseError::TokenError {
                    message: "Variable call must be a SYMBOL",
                    token: name,
                })
            }
        })?;

    let vc = VariableCall { name: assignment };

    Ok(Node::VariableCall(vc))
}

fn parse_expression(tokens: &mut Stack<Token>) -> Result<Node, ParseError> {
    let mut nested_level = 0;

    let name = tokens
        .pop_front()
        .ok_or(ParseError::StackError("No expression symbol in Stack"))
        .and_then(|name| match name.token_kind {
            TokenKind::Symbol => Ok(name),
            _ => {
                return Err(ParseError::TokenError {
                    message: "Expression name must be a symbol",
                    token: name,
                })
            }
        })?;

    let mut arg_vec: Stack<Node> = Stack::new();

    while let Some(token_arg) = tokens.pop_front() {
        match token_arg.token_kind {
            TokenKind::Bool | TokenKind::Integer | TokenKind::StringLiteral => {
                arg_vec.push(Node::Literal(token_arg))
            }
            TokenKind::LeftRoundBracket => {
                nested_level += 1;
                // recursive function call
                // added to stack linearly
                let res = parse_expression(tokens)?;
                arg_vec.push(res)
            }
            TokenKind::RightRoundBracket => {
                nested_level -= 1;

                if nested_level == 0 {
                    break;
                }
            }
            _ => {
                return Err(ParseError::TokenError {
                    message: "Function argument is not of the correct type: NUMBER, STRING, BOOL",
                    token: token_arg,
                })
            }
        }
    }

    let fc = FunctionCall {
        name,
        arguments: arg_vec,
    };

    Ok(Node::FunctionCall(fc))
}

pub fn parse(tokens: &mut Stack<Token>) -> Result<Stack<Node>, ParseError> {
    let mut nodes: Stack<Node> = Stack::new();

    while let Some(token) = tokens.pop_front() {
        match token.token_kind {
            TokenKind::VariableDefinition => nodes.push(parse_variable_definition(tokens)?),
            TokenKind::LeftSquareBracket => nodes.push(parse_list(tokens)?),
            TokenKind::FunctionDefinition => nodes.push(parse_function_definition(tokens)?),
            TokenKind::Conditional => nodes.push(parse_conditional(tokens)?),
            TokenKind::LeftRoundBracket => nodes.push(parse_expression(tokens)?),
            TokenKind::Symbol => nodes.push(parse_var_call(tokens)?),

            _ => {
                return Err(ParseError::TokenError {
                    message: "Wrong token",
                    token,
                });
            }
        };
    }

    Ok(nodes)
}
