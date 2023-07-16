use crate::lexer::{tokenise, Stack, Token};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Void,
    Integer(i64),
    Bool(bool),
    Symbol(String),
    Lambda(Vec<String>, Vec<Node>),
    List(Vec<Node>),
    Expression(Vec<Node>),
}

#[derive(Debug)]
pub struct TokenError {
    message: &'static str,
    token: Token,
}

#[derive(Debug)]
pub enum ParseError {
    InputError(String),
    TokenError(TokenError),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub body: Node,
}

impl Program {
    pub fn new() -> Self {
        Self { body: Node::Void }
    }

    pub fn parse(&mut self, tokens: &mut Stack) -> Result<Node, ParseError> {
        let nodes = self.parse_expression(tokens)?;

        if tokens.is_empty() {
            Ok(nodes)
        } else {
            Err(ParseError::InputError("Extra tokens remaining".to_string()))
        }
    }

fn parse_expression(&mut self, tokens: &mut Stack) -> Result<Node, ParseError> {
    let mut nodes = Vec::new();

    let mut lb_counter = 0;

    while let Some(token) = tokens.pop_front() {
        match token {
            Token::LeftRoundBracket => {
                // Node::Expression(nodes)
                // node::expression(+ 1 2)
                lb_counter += 1;
                if lb_counter > 1 {
                // for recursive expressions??
                // node::expression(+ 1 2 )
                    nodes.push(self.parse_expression(tokens)?)
                }
            }
            Token::Symbol(s) => {
                nodes.push(Node::Symbol(s));
            }
            Token::Integer(n) => {
                nodes.push(Node::Integer(n));
            }
            Token::RightRoundBracket => {
                lb_counter -= 1;
                if lb_counter == 0 {
                    break;
                }
            }
            _ => {
                return Err(ParseError::TokenError(TokenError {
                    message: "Unexpected token",
                    token: token,
                }));
            }
        }
    }

    Ok(Node::Expression(nodes))

}


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let code = "(+ 1 2)".to_string();
        let mut tokens = tokenise(code);
        let mut program = Program::new();
        match program.parse(&mut tokens) {
            Ok(list) => {
                assert_eq!(
                    list,
                    Node::Expression(vec![
                        Node::Symbol("+".to_string()),
                        Node::Integer(1),
                        Node::Integer(2),
                    ])
                );
            }

            Err(e) => {
                println!("{:?}", e)
            }
        }
    }

    #[test]
    fn test_parse_nested() {
        let code = "(+ 1 (+ 1 2))".to_string();
        let mut tokens = tokenise(code);
        let mut program = Program::new();
        match program.parse(&mut tokens) {
            Ok(list) => {
                assert_eq!(
                    list,
                    Node::Expression(vec![
                        Node::Symbol("+".to_string()),
                        Node::Integer(1),
                        Node::Expression(vec![
                            Node::Symbol("+".to_string()),
                            Node::Integer(1),
                            Node::Integer(2),
                        ])
                    ])
                );
            }

            Err(e) => {
                println!("{:?}", e)
            }
        }
    }
}
