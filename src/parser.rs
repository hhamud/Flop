use crate::lexer::{tokenise, Token};
use std::string::ParseError;

#[derive(Debug, PartialEq)]
pub enum Object {
    Void,
    Integer(i64),
    Bool(bool),
    Symbol(String),
    Lambda(Vec<String>, Vec<Object>),
    List(Vec<Object>),
}

fn parse(code: String) -> Vec<Object> {
    let tokens = tokenise(code);
    let token = tokens.pop();

    if token != Some(Token::LeftRoundBracket) {
        return Err(ParseError {
            err: format!("Expected (, found {:?}", token),
        });
    }

    let mut list: Vec<Object> = Vec::new();
    while !tokens.is_empty() {
        let token = tokens.pop();
        if token == None {
            return Err(ParseError, { err: format!("Insufficient tokens") });
        }

        let t = token.unwrap();
        match t {
            Token::Integer(n) => list.push(Object::Integer(n)),
            Token::Symbol(s) => list.push(Object::Symbol(s)),
            Token::LeftRoundBracket => {
                tokens.push(Token::LeftRoundBraket);
                let sub_list = parse_list(tokens)?;
                list.push(sub_list);
            }
            Token::RightRoundBracket => return Ok(Object::List(list)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let code = "(+ 1 2)".to_string();
        let list = parse(code);
        assert_eq!(
            list,
            Object::List(vec![
                Object::Symbol("+".to_string()),
                Object::Integer(1),
                Object::Integer(2),
            ])
        )
    }
}
