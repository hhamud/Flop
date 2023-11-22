use crate::{
    error::ParseError,
    stack::Stack,
    token::{Token, TokenKind},
};

#[derive(Debug, Clone, PartialEq)]
pub enum Node {
    FunctionDefinition {
        name: Token,
        parameters: Stack<Token>,
        docstrings: Option<Token>,
        body: Stack<Node>,
    },

    Conditional {
        condition: Stack<Node>,
        true_expression: Stack<Node>,
        false_expression: Stack<Node>,
    },

    VariableDefinition {
        name: Token,
        assignment: Token,
    },

    FunctionCall {
        name: Token,
        arguments: Stack<Token>,
    },

    VariableCall {
        name: Token,
    },

    List {
        data: Stack<Token>,
    },

    Documentation {
        commentary: Token,
        code: Option<Stack<Node>>,
    },
}

fn parse_variable_definition(
    tokens: &mut Stack<Token>,
    nodes: &mut Stack<Node>,
) -> Result<&'static mut Stack<Node>, ParseError> {
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

    let var = Node::VariableDefinition {
        name: var_name,
        assignment: value,
    };

    nodes.push(var);

    Ok(nodes)
}

fn parse_list(
    tokens: &mut Stack<Token>,
    nodes: &mut Stack<Node>,
) -> Result<&'static mut Stack<Node>, ParseError> {
    let mut list_args: Stack<Token> = Stack::new();

    let mut nested_level = 0;

    while let Some(list_arg) = tokens.pop_front() {
        match list_arg.token_kind {
            TokenKind::Bool => list_args.push(list_arg),
            TokenKind::Integer => list_args.push(list_arg),
            TokenKind::StringLiteral => list_args.push(list_arg),
            TokenKind::LeftSquareBracket => {
                nested_level += 1;
                let res = parse_list(tokens, nodes)?;
                nodes.push(res.pop_front().unwrap());
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

    let node = Node::List { data: list_args };

    nodes.push(node);

    Ok(nodes)
}

fn parse_function_definition(
    tokens: &mut Stack<Token>,
    nodes: &mut Stack<Node>,
) -> Result<Stack<Node>, ParseError> {
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

    let mut function_args: Stack<Token> = Stack::new();

    while let Some(function_arg) = tokens.pop_front() {
        match function_arg.token_kind {
            TokenKind::Symbol => function_args.push(function_arg),
            _ => {
                return Err(ParseError::TokenError {
                    message: "Function Parameter must be a SYMBOL",
                    token: function_arg,
                })
            }
        }
    }

    let doc_string = tokens
        .pop_front()
        .ok_or(ParseError::StackError("No DocString Token in Stack"))
        .and_then(|doc_string| match name.token_kind {
            TokenKind::StringLiteral => Ok(doc_string),
            _ => {
                return Err(ParseError::TokenError {
                    message: "Function name must be a symbol",
                    token: name,
                })
            }
        })?;

    // function body
    let mut count_brackets = 0;
    // Check if the first token is a LeftRoundBracket
    if let Some(first_token) = tokens.pop_front() {
        if first_token.token_kind != TokenKind::LeftRoundBracket {
            return Err(ParseError::TokenError {
                message: "Function body must start with a LeftRoundBracket",
                token: first_token,
            });
        }
        // Process the first LeftRoundBracket
        let mut function_body: Stack<Token> = Stack::new();
        function_body.push(first_token);

        // Initialize with 1 since the first token is a LeftRoundBracket
        let mut count_brackets = 1;

        while let Some(fb) = tokens.pop_front() {
            match fb.token_kind {
                TokenKind::LeftRoundBracket => {
                    count_brackets += 1;
                    function_body.push(fb);
                }
                TokenKind::RightRoundBracket => {
                    count_brackets -= 1;
                    if count_brackets == 0 {
                        break;
                    }
                    // Continue processing if there are more brackets
                }
                _ => {
                    return Err(ParseError::TokenError {
                        message: "Function Parameter must be a SYMBOL",
                        token: fb,
                    });
                }
            }
        }

        // Check if all brackets are closed
        if count_brackets != 0 {
            return Err(ParseError::TokenError {
                message: "Unclosed brackets in function body",
                token: function_body.top().unwrap_or(first_token),
            });
        }
    } else {
        // Handle the case when tokens.pop_front() returns None
        return Err(ParseError::StackError("Function body stack is empty"));
    }
}

fn parse_conditional(
    tokens: &mut Stack<Token>,
    nodes: &mut Stack<Node>,
) -> Result<Stack<Node>, ParseError> {
    todo!()
}

fn parse_expression(
    tokens: &mut Stack<Token>,
    nodes: &mut Stack<Node>,
) -> Result<&'static mut Stack<Node>, ParseError> {
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

    let mut arg_vec: Stack<Token> = Stack::new();

    while let Some(token_arg) = tokens.pop_front() {
        match token_arg.token_kind {
            TokenKind::Bool | TokenKind::Integer | TokenKind::StringLiteral => {
                arg_vec.push(token_arg)
            }
            TokenKind::LeftRoundBracket => {
                nested_level += 1;
                let res = parse(tokens)?;
                // returns Stack<Node>
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

    let fc = Node::FunctionCall {
        name,
        arguments: arg_vec,
    };

    nodes.push(fc);

    Ok(nodes)
}

pub fn parse(tokens: &mut Stack<Token>) -> Result<Stack<Node>, ParseError> {
    let mut nodes: Stack<Node> = Stack::new();

    while let Some(token) = tokens.pop_front() {
        match token.token_kind {
            TokenKind::VariableDefinition => parse_variable_definition(tokens, &mut nodes)?,
            TokenKind::LeftSquareBracket => parse_list(tokens, &mut nodes)?,
            TokenKind::FunctionDefinition => parse_function_definition(tokens, &mut nodes)?,
            TokenKind::Conditional => parse_conditional(tokens, &mut nodes)?,
            TokenKind::LeftRoundBracket => parse_expression(tokens, &mut nodes)?,

            _ => {
                return Err(ParseError::TokenError {
                    message: "Wrong token",
                    token,
                });
            }
        }
    }

    Ok(nodes)
}
