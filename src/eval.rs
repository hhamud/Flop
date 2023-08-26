use crate::ast::{FunctionDefinition, Variable};
use crate::parser::Node;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
pub enum EvalResult {
    Void,
    Integer(i64),
    StringLiteral(String),
    Bool(bool),
    List(Vec<EvalResult>),
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

fn operation(
    ast: &[Node],
    symbol: &str,
    env: &mut Environment,
) -> Result<EvalResult, &'static str> {
    let mut oper: i64 = match evaluate(&ast[1], env)? {
        EvalResult::Integer(n) => n,
        _ => return Err("Expected integer operand"),
    };

    for operand in &ast[2..] {
        let oper_val = match evaluate(operand, env)? {
            EvalResult::Integer(n) => n,
            _ => return Err("Expected integer operand"),
        };

        match symbol {
            "+" => oper += oper_val,
            "-" => oper -= oper_val,
            "/" => oper /= oper_val,
            "*" => oper *= oper_val,
            _ => return Err("Unsupported math Operation"),
        }
    }
    Ok(EvalResult::Integer(oper))
}

fn binary_expression(
    ast: &[Node],
    symbol: &str,
    env: &mut Environment,
) -> Result<EvalResult, &'static str> {
    if ast.len() < 3 {
        return Err("Insufficient operands");
    }

    let mut oper = match evaluate(&ast[1], env)? {
        EvalResult::Integer(n) => n,
        _ => return Err("Expected integer operand"),
    };

    for operand in &ast[2..] {
        let operand_val = match evaluate(operand, env)? {
            EvalResult::Integer(n) => n,
            _ => return Err("Expected integer operand"),
        };

        match symbol {
            "=" => oper = (oper == operand_val) as i64,
            ">" => oper = (oper > operand_val) as i64,
            ">=" => oper = (oper >= operand_val) as i64,
            "<" => oper = (oper < operand_val) as i64,
            "<=" => oper = (oper <= operand_val) as i64,
            _ => return Err("Unsupported binary operation"),
        }

        // Short-circuit if the result is already false
        if symbol != "=" && oper == 0 {
            return Ok(EvalResult::Bool(false));
        }
    }

    Ok(EvalResult::Bool(oper != 0))
}

fn evaluate_variable(symbol: &str, env: &mut Environment) -> Result<EvalResult, &'static str> {
    match env.variables.get(symbol) {
        Some(variable) => Ok(evaluate(&variable.assignment.clone(), env)?),
        _ => Err("Undefined symbol: {}"),
    }
}

fn evaluate_list(list: &[Node], env: &mut Environment) -> Result<EvalResult, &'static str> {
    let mut res = Vec::new();
    for item in list {
        res.push(evaluate(item, env)?)
    }

    Ok(EvalResult::List(res))
}

fn insert_variable(
    variable: (&Box<Node>, &Box<Node>),
    env: &mut Environment,
) -> Result<EvalResult, &'static str> {
    // Dereference the boxed node to get the actual node
    let (name_node, assignment_node) = (variable.0.deref(), variable.1.deref());

    if let Node::Symbol(name_str) = name_node {
        let var = Rc::new(Variable {
            name: name_str.clone(),
            assignment: assignment_node.clone(),
        });
        env.variables.insert(name_str.clone(), var);
        Ok(EvalResult::Void)
    } else {
        Err("Expected a Symbol node for variable name")
    }
}

fn evaluate_expression(nodes: &[Node], env: &mut Environment) -> Result<EvalResult, &'static str> {
    if nodes.is_empty() {
        return Err("Empty Expression");
    }

    if nodes.len() == 1 {
        let node = nodes.last().unwrap();
        return Ok(evaluate(node, env)?);
    }

    if nodes.len() >= 2 {
        if let Node::Symbol(symbol) = &nodes[0] {
            if ["=", ">", ">=", "<", "<="].contains(&symbol.as_str()) {
                return binary_expression(&nodes, symbol, env);
            }
        }
    }

    if let Node::Symbol(name) = &nodes[0] {
        if let Some(func_def) = env.functions.get(name) {
            // does not recognise higher order functions
            println!("nodes: {:?}", nodes);
            println!("node length: {:?}", nodes.len());
            println!("params: {:?}", func_def.parameters.len());
            if nodes.len() - 1 != func_def.parameters.len() {
                return Err("Incorrect number of arguments");
            }

            let mut local_env = Environment {
                functions: env.functions.clone(),
                variables: env.variables.clone(),
            };

            for (param, arg) in func_def.parameters.iter().zip(&nodes[1..]) {
                local_env.variables.insert(
                    param.clone(),
                    Rc::new(Variable {
                        name: param.clone(),
                        assignment: arg.clone(),
                    }),
                );
            }

            return evaluate(&func_def.body, &mut local_env);
        } else {
            return operation(nodes, name.as_str(), env);
        }
    }

    Err("Expected function name, operator, or expression")
}

fn insert_function_definition(
    nodes: &[Node],
    env: &mut Environment,
) -> Result<EvalResult, &'static str> {
    if nodes.len() < 3 {
        return Err("Incomplete function definition");
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
            return Err("Expected parameter list");
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

        env.functions.insert(name.clone(), func_def);
        Ok(EvalResult::Void)
    } else {
        Err("Expected function name")
    }
}

pub fn evaluate(ast: &Node, env: &mut Environment) -> Result<EvalResult, &'static str> {
    match ast {
        Node::Integer(n) => Ok(EvalResult::Integer(*n)),
        Node::StringLiteral(s) => Ok(EvalResult::StringLiteral(s.to_string())),
        Node::Symbol(s) => Ok(evaluate_variable(s, env)?),
        Node::Variable(n, v) => Ok(insert_variable((n, v), env)?),
        Node::Bool(b) => Ok(EvalResult::Bool(*b)),
        Node::List(l) => Ok(evaluate_list(l, env)?),
        Node::Expression(nodes) => Ok(evaluate_expression(nodes, env)?),
        Node::FunctionDefinition(nodes) => Ok(insert_function_definition(nodes, env)?),
        _ => Err("Unsupported node type"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::eval_test;
    use crate::lexer::tokenise;
    use crate::parser::parse;

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
            _ => panic!("Expected integer result of 5"),
        }
    }
}
