use std::collections::HashMap;

use crate::{env::Environment, error::EvalError, operation::Operation};

use flop_frontend::{
    ast::{FunctionCall, Node, VariableDefinition},
    stack::Stack,
    token::{Token, TokenKind},
};

pub enum EvalResult {
    /// handles base literals like integers, bools and strings
    Literal(Token),
    Void,
    List(Stack<Node>),
}

fn parse_literal(node: Node, env: &mut Environment) -> Result<i64, EvalError> {
    match evaluate_node(node, env) {
        Ok(EvalResult::Literal(token)) => Ok(token.token.parse::<i64>()?),
        _ => unreachable!(),
    }
}

fn evaluate_math(fc: &mut FunctionCall, env: &mut Environment) -> Result<EvalResult, EvalError> {
    let node: Node = fc.arguments.pop_front().unwrap();

    let mut oper = parse_literal(node, env)?;

    let operation = Operation::try_from(&fc.name)?;

    while let Some(operand) = fc.arguments.pop_front() {
        let oper_val = parse_literal(operand, env)?;

        oper = operation.apply(oper, oper_val);
    }

    let final_token = Token::new(
        &oper.to_string(),
        TokenKind::Integer,
        1,
        1,
        oper.to_string().len(),
        &fc.name.namespace,
    );

    Ok(EvalResult::Literal(final_token))
}

pub fn evaluate_node(mut node: Node, env: &mut Environment) -> Result<EvalResult, EvalError> {
    match node {
        Node::FunctionCall(ref mut fc) => match env.functions.get(&fc.name.token) {
            Some(function) => {
                let mut local_env = Environment {
                    functions: env.functions.clone(),
                    variables: HashMap::new(),
                };

                for (param, arg) in function
                    .parameters
                    .data
                    .iter()
                    .zip(fc.arguments.data.clone())
                {
                    let assignment = match arg {
                        Node::Literal(token) => token,
                        _ => unreachable!(),
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

                while let Some(b) = body.pop_front() {
                    return evaluate_node(b, &mut local_env);
                }

                Ok(EvalResult::Void)
            }
            None => return evaluate_math(fc, env),
        },
        Node::FunctionDefinition(fd) => {
            env.functions.insert(fd.name.token.clone(), fd.clone());
            Ok(EvalResult::Void)
        }
        Node::VariableDefinition(vd) => {
            env.variables.insert(vd.name.token.clone(), vd.clone());
            Ok(EvalResult::Void)
        }
        Node::Literal(token) => Ok(EvalResult::Literal(token)),

        Node::VariableCall(vc) => match env.variables.get(&vc.name.token) {
            Some(variable) => Ok(EvalResult::Literal(variable.assignment.clone())),
            None => Err(EvalError::FunctionCallMissing(vc.name)),
        },

        Node::List(ls) => Ok(EvalResult::List(ls.data)),

        _ => unreachable!(),
    }
}
