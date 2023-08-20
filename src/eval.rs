use crate::helpers::eval_test;
use crate::lexer::tokenise;
use crate::parser::{Node, Parser};
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug)]
pub enum EvalResult {
    Void,
    Integer(i64),
    StringLiteral(String),
    Bool(bool),
    List(Vec<EvalResult>),
    Function(Rc<FunctionDefinition>),
}

#[derive(Debug)]
pub struct Environment {
    functions: HashMap<String, Rc<FunctionDefinition>>,
    variables: HashMap<String, Rc<Variable>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionDefinition {
    name: String,
    parameters: Vec<String>,
    docstrings: Option<String>,
    body: Node,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    name: String,
    assignment: Node,
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
        Node::StringLiteral(s) => Ok(EvalResult::StringLiteral(s.to_string())),

        Node::Symbol(s) => {
            if let Some(var) = env.variables.get(s) {
                let assignment_clone = var.assignment.clone();
                println!("{:?}", assignment_clone);
                Ok(evaluate(&assignment_clone, env)?)
            } else {
                Err(format!("Undefined symbol: {}", s))
            }
        }
        Node::Variable(n, v) => {
            // Dereference the boxed node to get the actual node
            let name_node = &**n;
            let assignment_node = &**v;

            if let Node::Symbol(name_str) = name_node {
                let var = Rc::new(Variable {
                    name: name_str.clone(),
                    assignment: assignment_node.clone(),
                });
                env.variables.insert(name_str.clone(), var);
                Ok(EvalResult::Void)
            } else {
                Err("Expected a Symbol node for variable name".to_string())
            }
        }

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

            // variable checking
            if nodes.len() == 1 {
                let node = &nodes.clone().pop().unwrap();
                println!("{:?}", node);
                Ok(evaluate(node, env)?)
            } else if let Node::Symbol(name) = &nodes[0] {
                // function checking
                println!("nodes shown: {:?}", nodes);

                if let Some(func_def) = env.functions.get(name) {
                    // check for correct number of args provided
                    if nodes.len() - 1 != func_def.parameters.len() {
                        return Err("Incorrect number of arguements".to_string());
                    }

                    // create a local scope for the function
                    // clone the global env
                    // execute within this env
                    let mut local_env = Environment {
                        functions: env.functions.clone(),
                        variables: HashMap::new(),
                    };

                    // binding of parameter with body args
                    for (param, arg) in func_def.parameters.iter().zip(&nodes[1..]) {
                        println!("{:?}", param);
                        println!("{:?}", arg);

                        local_env.variables.insert(
                            param.clone(),
                            Rc::new(Variable {
                                name: param.clone(),
                                assignment: arg.clone(),
                            }),
                        );
                    }

                    // evaluate the function body with bounded args
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

                let mut docstrings: Option<String> = None;

                if let Node::DocString(name) = &nodes[2] {
                    docstrings = Some(name.to_string());
                }

                let body = nodes[3].clone();
                let func_def = Rc::new(FunctionDefinition {
                    name: name.to_string(),
                    parameters,
                    docstrings,
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
    fn add_function_definition() {
        let code = r#"(defn add [x y] "info" (+ x y))"#.to_string();
        let mut tokens = tokenise(code);
        let mut parser = Parser::new();
        let ast = parser.parse(&mut tokens).unwrap();
        let mut env = Environment::new();
        let eval = evaluate(&ast, &mut env);
        match eval {
            Ok(EvalResult::Integer(_))
            | Ok(EvalResult::Bool(_))
            | Ok(EvalResult::List(_))
            | Ok(EvalResult::Void) => {}
            Ok(EvalResult::Function(n)) => {
                assert_eq!(
                    n,
                    Rc::new(FunctionDefinition {
                        name: "add".to_string(),
                        parameters: vec!["x".to_string(), "y".to_string()],
                        docstrings: Some("info".to_string()),
                        body: Node::Expression(vec![
                            Node::Symbol("+".to_string()),
                            Node::Symbol("x".to_string()),
                            Node::Symbol("y".to_string()),
                        ])
                    })
                )
            }

            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }
    #[test]
    fn function_call() {
        // Define and call the add function
        let function_def = r#"
        (defn add [x y] "adding lmao" (+ x y))
    "#
        .to_string();

        let mut tokens = tokenise(function_def);
        let mut parser = Parser::new();
        let ast = parser.parse(&mut tokens).unwrap();

        let mut env = Environment::new();

        evaluate(&ast, &mut env).unwrap();

        // check if add has been defined
        assert!(env.functions.contains_key("add"));

        // call add function
        let function_call = r#"(add 2 3)"#.to_string();
        let mut tokens = tokenise(function_call);

        let func_ast = parser.parse(&mut tokens).unwrap();
        let result = evaluate(&func_ast, &mut env);

        // Check the result
        match result {
            Ok(EvalResult::Integer(n)) => assert_eq!(n, 5),
            _ => panic!("Expected integer result of 5"),
        }
    }
}
