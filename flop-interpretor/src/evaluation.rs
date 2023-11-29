use std::{
    collections::{HashMap, VecDeque},
    error::Error,
};

use crate::env::Environment;
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

fn operation(fc: FunctionCall) -> Result<EvalResult, Box<dyn Error>> {
    let mut oper: i64 = match &fc.arguments.data[0] {
        Node::Literal(token) => token.token.parse::<i64>()?,
        _ => todo!(),
    };

    for operand in fc.arguments.data.range(1..) {
        let oper_val: i64 = match operand {
            Node::Literal(token) => token.token.parse::<i64>()?,
            _ => todo!(),
        };

        match fc.name.token.as_str() {
            "+" => oper += oper_val,
            "-" => oper -= oper_val,
            "/" => oper /= oper_val,
            "*" => oper *= oper_val,
            _ => return Err(format!("lmao how did you break this, nice find more").into()),
        }
    }

    let new_token: Token = match &fc.arguments.data[0] {
        Node::Literal(token) => Token::new(
            &oper.to_string(),
            token.token_kind.clone(),
            token.row,
            token.column.clone(),
            &token.namespace,
        ),
        _ => todo!(),
    };

    Ok(EvalResult::Literal(new_token))
}

pub fn evaluate(
    nodes: &mut Stack<Node>,
    env: &mut Environment,
) -> Result<EvalResult, Box<dyn Error>> {
    while let Some(node) = nodes.pop_front() {
        match node {
            Node::FunctionCall(fc) => {
                if let Some(function) = env.functions.get(&fc.name.token) {
                    // new local scope
                    // TODO only clone this specific function and not all
                    let mut local_env = Environment {
                        functions: env.functions.clone(),
                        variables: HashMap::new(),
                    };

                    //(Add 1 2)

                    //(defn Add [x y]
                    // "adds two numbers together"
                    // (+ x y))

                    // grouping parmeter with function arg
                    // (Add [x y]) -> (Add 1 2) -> (x, 1) (y, 2)
                    // (+ x y)

                    for (param, arg) in function.parameters.data.iter().zip(fc.arguments.data) {
                        //TODO do something better than this
                        // unwrap assignment of token back into node
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
                    // could be a math operation
                    return operation(fc);
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

    return Err(format!("Failed to pop off").into());
}
