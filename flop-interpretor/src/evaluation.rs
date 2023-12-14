use std::collections::HashMap;

use crate::{env::Environment, error::EvalError, operation::Operation};

use flop_frontend::{
    ast::{FunctionCall, Node, VariableDefinition},
    stack::Stack,
    token::Token,
};

pub enum EvalResult {
    Literal(Token),
    Void,
    List(Stack<Node>),
}

fn parse_literal(node: &Node) -> Result<i64, EvalError> {
    if let Node::Literal(token) = node {
        Ok(token.token.parse::<i64>()?)
    } else {
        Err(EvalError::LiteralError(node.clone()))
    }
}

fn evaluate_math(fc: FunctionCall) -> Result<EvalResult, EvalError> {
    let mut oper = parse_literal(&fc.arguments.data[0])?;

    let operation = Operation::try_from(fc.name.token.as_str())?;

    for operand in fc.arguments.data.iter().skip(1) {
        let oper_val = parse_literal(operand)?;

        oper = operation.apply(oper, oper_val);
    }

    let new_token: Token = match &fc.arguments.data[0] {
        Node::Literal(token) => Token::new(
            &oper.to_string(),
            token.token_kind.clone(),
            token.row,
            token.column.clone(),
            &token.namespace,
        ),
        _ => return Err(EvalError::OperationError(fc.arguments.data[0])),
    };

    Ok(EvalResult::Literal(new_token))
}

pub fn evaluate(nodes: &mut Stack<Node>, env: &mut Environment) -> Result<EvalResult, EvalError> {
    while let Some(node) = nodes.pop_front() {
        match node {
            Node::FunctionCall(fc) => {
                if let Some(function) = env.functions.get(&fc.name.token) {
                    let mut local_env = Environment {
                        functions: env.functions.clone(),
                        variables: HashMap::new(),
                    };

                    for (param, arg) in function.parameters.data.iter().zip(fc.arguments.data) {
                        let assignment = match arg {
                            Node::Literal(token) => token,
                            _ => todo!(),
                        };

                        local_env.variables.insert(
                            param.token.clone(),
                            VariableDefinition {
                                name: param.clone(),
                                assignment,
                            },
                        );
                    }

                    let mut body = function.body.clone();

                    return evaluate(&mut body, &mut local_env);
                } else {
                    return evaluate_math(fc);
                }
            }
            Node::FunctionDefinition(fd) => {
                env.functions.insert(fd.name.token.clone(), fd);
                return Ok(EvalResult::Void);
            }
            Node::VariableDefinition(vd) => {
                env.variables.insert(vd.name.token.clone(), vd);
                return Ok(EvalResult::Void);
            }
            Node::Conditional(_) => {
                todo!()
            }
            Node::Literal(token) => {
                // handles integers, bools and strings
                return Ok(EvalResult::Literal(token));
            }
            Node::VariableCall(vc) => {
                if let Some(variable) = env.variables.get(&vc.name.token) {
                    return Ok(EvalResult::Literal(variable.assignment.clone()));
                }
            }
            Node::List(ls) => {
                return Ok(EvalResult::List(ls.data));
            }
            Node::Documentation(_) => {
                todo!();
            }
        }
    }

    return Err(EvalError::StackError(nodes.clone()));
}
