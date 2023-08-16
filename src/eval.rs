use crate::helpers::eval_test;
use crate::parser::Node;

#[derive(Debug)]
pub enum EvalResult {
    Integer(i64),
    Bool(bool),
    List(Vec<EvalResult>),
}

pub fn operation(ast: &Vec<Node>, symbol: &str) -> Result<EvalResult, String> {
    let mut oper: i64 = match evaluate(&ast[1])? {
        EvalResult::Integer(n) => n,
        _ => return Err("Expected integer operand".to_string()),
    };

    for operand in &ast[2..] {
        let oper_val = match evaluate(operand)? {
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

pub fn evaluate(ast: &Node) -> Result<EvalResult, String> {
    match ast {
        Node::Integer(n) => Ok(EvalResult::Integer(*n)),
        Node::Symbol(s) => Err("Cannot evaluate a standalone symbol".to_string()),
        Node::Bool(b) => Ok(EvalResult::Bool(*b)),
        Node::List(l) => {
            let mut res = Vec::new();
            for list in l {
                res.push(evaluate(&list)?)
            }

            Ok(EvalResult::List(res))
        }
        Node::Expression(v) => match &v[0] {
            Node::Symbol(s) => match s.as_str() {
                s => operation(v, s),
            },
            _ => Err("First element of an expression should be a symbol".to_string()),
        },
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
}
