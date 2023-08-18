use crate::helpers::eval_test;
use crate::parser::Node;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub enum EvalResult {
    Integer(i64),
    Bool(bool),
    List(Vec<EvalResult>),
    Function(Rc<FunctionDefinition>),
}

#[derive(Debug)]
pub struct Environment {
    functions: HashMap<String, Rc<FunctionDefinition>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionDefinition {
    name: String,
    parameters: Vec<String>,
    docstrings: Option<String>,
    body: Node,
}

pub fn operation(
    ast: &Vec<Node>,
    symbol: &str,
    env: &mut Environment,
) -> Result<EvalResult, String> {
    let mut oper: i64 = match evaluate(&ast[1], env)? {
        EvalResult::Integer(n) => n,
        _ => return Err("Expected integer operand".to_string()),
    };

    for operand in &ast[2..] {
        let oper_val = match evaluate(operand, env)? {
            EvalResult::Integer(n) => n,
            _ => return Err("Expected integer operand".to_string()),
        };
        match symbol {
            "+" => oper += oper_val,
            "-" => oper -= oper_val,
            "/" => oper /= oper_val,
            "*" => oper *= oper_val,
            _ => return Err(format!("Unsupported operation: {}", symbol)),
        }
    }
    Ok(EvalResult::Integer(oper))
}

pub fn evaluate(ast: &Node, env: &mut Environment) -> Result<EvalResult, String> {
    match ast {
        Node::Integer(n) => Ok(EvalResult::Integer(*n)),
        Node::Symbol(s) => Err("Cannot evaluate a standalone symbol".to_string()),
        Node::Bool(b) => Ok(EvalResult::Bool(*b)),
        Node::List(l) => {
            let mut res = Vec::new();
            for list in l {
                res.push(evaluate(&list, env)?)
            }

            Ok(EvalResult::List(res))
        }
        Node::Expression(nodes) => {
            if nodes.is_empty() {
                return Err("Empty Expression".to_string());
            }

            if let Node::Symbol(name) = &nodes[0] {
                if let Some(func_def) = env.functions.get(name) {
                    if nodes.len() - 1 != func_def.parameters.len() {
                        return Err("Incorrect number of arguements".to_string());
                    }

                    let mut local_env = Environment {
                        functions: env.functions.clone(),
                    };

                    for (param, arg) in func_def.parameters.iter().zip(&nodes[1..]) {
                        local_env.functions.insert(
                            param.clone(),
                            Rc::new(FunctionDefinition {
                                name: String::new(),
                                parameters: Vec::new(),
                                docstrings: None,
                                body: arg.clone(),
                            }),
                        );
                    }

                    evaluate(&func_def.body, &mut local_env)
                } else {
                    operation(nodes, name.as_str(), env)
                }
            } else {
                Err("Expected function name or operator".to_string())
            }
        }
        Node::FunctionDefinition(nodes) => {
            if nodes.len() < 3 {
                return Err("Incomplete function definition".to_string());
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
                    return Err("Expected parameter list".to_string());
                };

                let body = nodes[3].clone();
                let func_def = Rc::new(FunctionDefinition {
                    name: name.to_string(),
                    parameters,
                    docstrings: None,
                    body,
                });

                env.functions.insert(name.clone(), func_def.clone());
                Ok(EvalResult::Function(func_def))
            } else {
                Err("Expected function name".to_string())
            }
        }

        _ => Err("Unsupported node type".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn functions() {
        let code = eval_test("(- 1 (+ 1 2))").unwrap();
        assert_eq!(code, -2)
    }
}
