use crate::{
    ast::*,
    error::ParseError,
    stack::Stack,
    token::{Token, TokenKind},
};

fn parse_variable_definition(tokens: &mut Stack<Token>) -> Result<Node, ParseError<Token>> {
    let var_token = tokens.pop_front().ok_or(ParseError::StackError {
        name: "variable name",
        stack: tokens.clone(),
    })?;

    let var_name = match var_token.token_kind {
        TokenKind::Symbol => var_token,
        _ => return Err(ParseError::VariableDefinition(var_token)),
    };

    let value_token = tokens.pop_front().ok_or(ParseError::StackError {
        name: "Failed to pop stack for variable reference",
        stack: tokens.clone(),
    })?;

    let value = match value_token.token_kind {
        TokenKind::Integer => value_token,
        TokenKind::Bool => value_token,
        TokenKind::StringLiteral => value_token,
        _ => return Err(ParseError::VariableAssignment(value_token)),
    };

    let var = VariableDefinition {
        name: var_name,
        assignment: value,
    };

    Ok(Node::VariableDefinition(var))
}

fn parse_list(tokens: &mut Stack<Token>) -> Result<Node, ParseError<Token>> {
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
            _ => return Err(ParseError::ListDefinition(list_arg)),
        }
    }

    let node = List { data: list_args };

    Ok(Node::List(node))
}

fn parse_function_definition(tokens: &mut Stack<Token>) -> Result<Node, ParseError<Token>> {
    let name = tokens
        .pop_front()
        .ok_or(ParseError::StackError {
            name: "No function name Token in Stack",
            stack: tokens.clone(),
        })
        .and_then(|name| match name.token_kind {
            TokenKind::Symbol => Ok(name),
            _ => return Err(ParseError::FunctionName(name)),
        })?;

    let _left_bracket = tokens
        .pop_front()
        .ok_or(ParseError::StackError {
            name: "No left braket in stack",
            stack: tokens.clone(),
        })
        .and_then(|lft| match lft.token_kind {
            TokenKind::LeftSquareBracket => Ok(lft),
            _ => return Err(ParseError::NoLeftBracket(lft)),
        })?;

    let mut parameters: Stack<Token> = Stack::new();

    while let Some(function_arg) = tokens.pop_front() {
        match function_arg.token_kind {
            TokenKind::Symbol => parameters.push(function_arg),
            TokenKind::RightSquareBracket => break,
            _ => return Err(ParseError::FunctionParameter(function_arg)),
        }
    }

    let docstrings = tokens
        .pop_front()
        .ok_or(ParseError::StackError {
            name: "No DocString Token in Stack",
            stack: tokens.clone(),
        })
        .and_then(|doc_string| match doc_string.token_kind {
            TokenKind::DocString => Ok(doc_string),
            _ => Err(ParseError::FunctionDocstring(doc_string)),
        })?;

    if let Some(first_token) = tokens.pop_front() {
        if first_token.token_kind != TokenKind::LeftRoundBracket {
            return Err(ParseError::FunctionBody(first_token));
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
        return Err(ParseError::StackError {
            name: "Function body stack is empty",
            stack: tokens.clone(),
        });
    }
}

fn parse_var_call(token: Token) -> Result<Node, ParseError<Token>> {
    let vc = VariableCall { name: token };

    Ok(Node::VariableCall(vc))
}

fn parse_expression(tokens: &mut Stack<Token>) -> Result<Node, ParseError<Token>> {
    let mut nested_level = 0;

    let name = tokens
        .pop_front()
        .ok_or(ParseError::StackError {
            name: "No expression symbol in Stack",
            stack: tokens.clone(),
        })
        .and_then(|name| match name.token_kind {
            TokenKind::Symbol => Ok(name),
            _ => return Err(ParseError::FunctionCallName(name)),
        })?;

    let mut arg_vec: Stack<Node> = Stack::new();

    while let Some(token_arg) = tokens.pop_front() {
        match token_arg.token_kind {
            TokenKind::Bool | TokenKind::Integer | TokenKind::StringLiteral | TokenKind::Symbol => {
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
            _ => return Err(ParseError::FunctionCallArg(token_arg)),
        }
    }

    let fc = FunctionCall {
        name,
        arguments: arg_vec,
    };

    Ok(Node::FunctionCall(fc))
}

pub fn parse(tokens: &mut Stack<Token>) -> Result<Stack<Node>, ParseError<Token>> {
    let mut nodes: Stack<Node> = Stack::new();

    while let Some(token) = tokens.pop_front() {
        match token.token_kind {
            TokenKind::VariableDefinition => nodes.push(parse_variable_definition(tokens)?),
            TokenKind::LeftSquareBracket => nodes.push(parse_list(tokens)?),
            TokenKind::FunctionDefinition => nodes.push(parse_function_definition(tokens)?),
            TokenKind::LeftRoundBracket => nodes.push(parse_expression(tokens)?),
            TokenKind::Symbol => nodes.push(parse_var_call(token)?),
            _ => return Err(ParseError::ParseError(token)),
        };
    }

    Ok(nodes)
}
