use crate::helpers::eval_test;
use crate::lexer::tokenise;
use crate::parser::{Node, Program};

pub fn add(a: i64, b: i64) -> Result<i64, String> {
    Ok(a + b)
}

pub fn evaluate(ast: &Node) -> Result<i64, String> {
    match ast {
        Node::Integer(n) => Ok(*n),
        Node::Symbol(s) => Err("Cannot evaluate a standalone symbol".to_string()),
        Node::Bool(b) => Err("Cannot evaluate a standalone bool".to_string()),
        Node::Expression(v) => {
            let operation = &v[0];
            match operation {
                Node::Symbol(s) => match s.as_str() {
                    "+" => {
                        let mut sum = 0;
                        for operand in &v[1..] {
                            sum += evaluate(operand)?
                        }
                        Ok(sum)
                    }
                    "-" => {
                        // start value from actual number and not from 0
                        let mut minus = 0;
                        for operand in &v[1..] {
                            minus -= evaluate(operand)?
                        }
                        Ok(minus)
                    }
                    _ => Err("Unsupported operation".to_string()),
                },
                _ => Err("First element of an expression should be a symbol".to_string()),
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
}
