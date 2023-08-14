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
    fn test_eval() {
        let code = "(+ 1 2)".to_string();
        let mut tokens = tokenise(code);
        let mut program = Program::new();
        let ast = program.parse(&mut tokens).unwrap();
        let eval = evaluate(&ast).unwrap();
        assert_eq!(eval, 3)
    }

    #[test]
    fn test_nested_eval() {
        let code = "(+ 1 (+ 1 2))".to_string();
        let mut tokens = tokenise(code);
        let mut program = Program::new();
        let ast = program.parse(&mut tokens).unwrap();
        let eval = evaluate(&ast).unwrap();
        assert_eq!(eval, 4)
    }
}
