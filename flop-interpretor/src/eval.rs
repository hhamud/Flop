use crate::ast::{FunctionDefinition, Variable};
use crate::env::Environment;
use crate::error::EvalError;
use flop_frontend::parser::Node;
use std::ops::Deref;

#[derive(Debug)]
pub enum EvalResult {
    Void,
    Integer(i64),
    StringLiteral(String),
    Bool(bool),
    List(Vec<EvalResult>),
}

fn operation(
    ast: &Vec<Node>,
    symbol: &str,
    env: &mut Environment,
) -> Result<EvalResult, EvalError> {
    let mut oper: i64 = match evaluate(&ast[1], env)? {
        EvalResult::Integer(n) => n,
        _ => {
            return Err(EvalError::Integer(
                ast.clone(),
                ast[1].clone(),
                ast[2..].to_vec(),
                symbol.to_string(),
            ))
        }
    };

    for operand in &ast[2..] {
        let oper_val = match evaluate(operand, env)? {
            EvalResult::Integer(n) => n,
            _ => {
                return Err(EvalError::Integer(
                    ast.clone(),
                    ast[1].clone(),
                    ast[2..].to_vec(),
                    symbol.to_string(),
                ))
            }
        };

        match symbol {
            "+" => oper += oper_val,
            "-" => oper -= oper_val,
            "/" => oper /= oper_val,
            "*" => oper *= oper_val,
            _ => return Err(EvalError::Math(symbol.to_string())),
        }
    }
    Ok(EvalResult::Integer(oper))
}

fn binary_expression(
    ast: &Vec<Node>,
    symbol: &str,
    env: &mut Environment,
) -> Result<EvalResult, EvalError> {
    if ast.len() < 3 {
        return Err(EvalError::Operands);
    }

    let mut oper = match evaluate(&ast[1], env)? {
        EvalResult::Integer(n) => n,
        _ => {
            return Err(EvalError::Integer(
                ast.clone(),
                ast[1].clone(),
                ast[2..].to_vec(),
                symbol.to_string(),
            ))
        }
    };

    for operand in &ast[2..] {
        let operand_val = match evaluate(operand, env)? {
            EvalResult::Integer(n) => n,

            _ => {
                return Err(EvalError::Integer(
                    ast.clone(),
                    ast[1].clone(),
                    ast[2..].to_vec(),
                    symbol.to_string(),
                ))
            }
        };

        match symbol {
            "=" => oper = (oper == operand_val) as i64,
            ">" => oper = (oper > operand_val) as i64,
            ">=" => oper = (oper >= operand_val) as i64,
            "<" => oper = (oper < operand_val) as i64,
            "<=" => oper = (oper <= operand_val) as i64,
            _ => return Err(EvalError::Binary(symbol.to_string())),
        }

        // Short-circuit if the result is already false
        if symbol != "=" && oper == 0 {
            return Ok(EvalResult::Bool(false));
        }
    }

    Ok(EvalResult::Bool(oper != 0))
}

fn evaluate_variable(symbol: &str, env: &mut Environment) -> Result<EvalResult, EvalError> {
    let mut e = env.clone();
    match env.variables.get(symbol) {
        Some(variable) => evaluate(&variable.assignment, &mut e),
        None => Err(EvalError::Variable(symbol.to_string())),
    }
}

fn evaluate_list(list: &Vec<Node>, env: &mut Environment) -> Result<EvalResult, EvalError> {
    let mut res = Vec::new();
    for item in list {
        res.push(evaluate(item, env)?)
    }

    Ok(EvalResult::List(res))
}

fn insert_variable(
    variable: (&Box<Node>, &Box<Node>),
    env: &mut Environment,
) -> Result<EvalResult, EvalError> {
    // Dereference the boxed node to get the actual node
    let (name_node, assignment_node) = (variable.0.deref(), variable.1.deref());

    if let Node::Symbol(name_str) = name_node {
        let var = Variable {
            name: name_str.clone(),
            assignment: assignment_node.clone(),
        };
        env.variables.insert(name_str.clone(), var);
        Ok(EvalResult::Void)
    } else {
        Err(EvalError::Symbol(name_node.clone()))
    }
}

fn evaluate_expression(nodes: &Vec<Node>, env: &mut Environment) -> Result<EvalResult, EvalError> {
    if nodes.is_empty() {
        return Err(EvalError::EmptyExpression(nodes.clone()));
    }

    if nodes.len() == 1 {
        let node = nodes.last().unwrap();
        // could either be a bool, int, string or var & func call
        return evaluate(node, env);
    }

    if nodes.len() >= 2 {
        if let Node::Symbol(symbol) = &nodes[0] {
            if ["=", ">", ">=", "<", "<="].contains(&symbol.as_str()) {
                return binary_expression(&nodes, symbol, env);
            }
        }
    }

    // already checks if it is a function call
    if let Node::Symbol(name) = &nodes[0] {
        if let Some(func_def) = env.functions.get(name) {
            // does not recognise higher order functions

            if nodes.len() - 1 != func_def.parameters.len() {
                return Err(EvalError::Parameter(nodes.clone()));
            }

            let mut local_env = Environment {
                functions: env.functions.clone(),
                variables: env.variables.clone(),
            };

            for (param, arg) in func_def.parameters.iter().zip(&nodes[1..]) {
                local_env.variables.insert(
                    param.clone(),
                    Variable {
                        name: param.clone(),
                        assignment: arg.clone(),
                    },
                );
            }

            // evaluate function bidy
            return evaluate(&func_def.body, &mut local_env);
        } else {
            // if it is not a function call then it is a math operation
            return operation(nodes, name.as_str(), env);
        }
    }

    Err(EvalError::UnexpectedExpression(nodes.clone()))
}

fn insert_function_definition(
    nodes: &Vec<Node>,
    env: &mut Environment,
) -> Result<EvalResult, EvalError> {
    if nodes.len() < 3 {
        return Err(EvalError::FunctionDefinition(nodes.clone()));
    }

    if let Node::Symbol(name) = &nodes[0] {
        let parameters = if let Node::Parameter(params) = &nodes[1] {
            params
                .iter()
                .filter_map(|param| {
                    if let Node::Symbol(s) = param {
                        Some(s.clone())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        } else {
            return Err(EvalError::Parameter(nodes.clone()));
        };

        let mut docstrings: Option<String> = None;

        if let Node::DocString(name) = &nodes[2] {
            docstrings = Some(name.to_string());
        }

        let body = nodes[3].clone();
        let func_def = FunctionDefinition {
            name: name.to_string(),
            parameters,
            docstrings,
            body,
        };

        env.functions.insert(name.clone(), func_def);
        Ok(EvalResult::Void)
    } else {
        Err(EvalError::FunctionName(nodes[0].clone()))
    }
}

// refactor this and split up the functions
pub fn evaluate(ast: &Node, env: &mut Environment) -> Result<EvalResult, EvalError> {
    match ast {
        Node::Integer(n) => Ok(EvalResult::Integer(*n)),
        Node::StringLiteral(s) => Ok(EvalResult::StringLiteral(s.to_string())),
        Node::Bool(b) => Ok(EvalResult::Bool(*b)),
        Node::List(l) => evaluate_list(l, env),
        Node::Symbol(s) => evaluate_variable(s, env),
        Node::Variable(n, v) => insert_variable((n, v), env),
        Node::Expression(nodes) => evaluate_expression(nodes, env),
        Node::FunctionDefinition(nodes) => insert_function_definition(nodes, env),
        _ => Err(EvalError::Node(ast.clone())),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::eval_test;
    use flop_frontend::{lexer::tokenise, parser::parse};

    #[test]
    fn add() {
        let code = eval_test("(+ 1 2)").unwrap();
        assert_eq!(code, 3)
    }

    #[test]
    fn nested_add() {
        let code = eval_test("(+ 1 (+ 1 2))").unwrap();
        assert_eq!(code, 4)
    }

    #[test]
    fn minus() {
        let code = eval_test("(- 1 2)").unwrap();
        assert_eq!(code, -1)
    }
    #[test]
    fn nested_minus() {
        let code = eval_test("(- 1 (- 1 2))").unwrap();
        assert_eq!(code, 2)
    }

    #[test]
    fn nested_mixed() {
        let code = eval_test("(+ 1 (- 1 2))").unwrap();
        assert_eq!(code, 0)
    }

    #[test]
    fn nested_mixed2() {
        let code = eval_test("(- 1 (+ 1 2))").unwrap();
        assert_eq!(code, -2)
    }

    #[test]
    fn function_call() {
        let function_def = r#"
        (defn add [x y] "adding lmao" (+ x y))
    "#
        .to_string();

        let mut tokens = tokenise(function_def);
        let ast = parse(&mut tokens).unwrap();

        let mut env = Environment::new();

        evaluate(&ast, &mut env).unwrap();

        // check if add has been defined
        assert!(env.functions.contains_key("add"));

        // call add function
        let function_call = r#"(add 2 3)"#.to_string();
        let mut tokens = tokenise(function_call);

        let func_ast = parse(&mut tokens).unwrap();
        let result = evaluate(&func_ast, &mut env);

        // Check the result
        match result {
            Ok(EvalResult::Integer(n)) => assert_eq!(n, 5),
            Err(e) => panic!("Expected integer result of 5: got {e}"),
            _ => todo!(),
        }
    }
}
