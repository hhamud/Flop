use crate::lexer::{tokenise, Stack, Token};

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Void,
    Integer(i64),
    Bool(bool),
    Symbol(String),
    List(Vec<Node>),
    Expression(Vec<Node>),
    FunctionDefinition(Vec<Node>),
    Parameter(Vec<Node>),
    DocString(String),
    Variable(Box<Node>, Box<Node>),
    StringLiteral(String),
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

#[derive(Debug, PartialEq)]
pub struct Parser {
    pub body: Node,
}

impl Parser {
    pub fn new() -> Self {
        Self { body: Node::Void }
    }

    pub fn parse(&mut self, tokens: &mut Stack) -> Result<Node, ParseError> {
        let nodes = match tokens.data[0] {
            Token::FunctionDefinition => self.parse_function_definition(tokens)?,
            Token::VariableDefinition => self.parse_variables(tokens)?,
            _ => self.parse_expression(tokens)?,
        };

        if tokens.is_empty() {
            Ok(nodes)
        } else {
            Err(ParseError::InputError("Extra tokens remaining".to_string()))
        }
    }

    pub fn parse_variables(&mut self, tokens: &mut Stack) -> Result<Node, ParseError> {
        // Expecting a variable name after `setq`

        // pop Node::Expression
        let _exp = tokens.pop_front();

        let var_token = tokens.pop_front();

        let var_name = match var_token {
            Some(Token::Symbol(s)) => Node::Symbol(s),
            _ => {
                return Err(ParseError::TokenError(TokenError {
                    message: "Variable Definition: Expected a variable name",
                    token: var_token.unwrap_or(Token::EOF),
                }))
            }
        };

        let value_token = tokens.pop_front();

        let value = match value_token {
            Some(Token::Integer(v)) => Node::Integer(v),
            Some(Token::Bool(b)) => Node::Bool(b),
            Some(Token::StringLiteral(s)) => Node::StringLiteral(s),
            Some(Token::LeftRoundBracket) => self.parse_expression(tokens)?,
            _ => {
                return Err(ParseError::TokenError(TokenError {
                    message: "Variable assignment: Expected a variable value",
                    token: value_token.unwrap_or(Token::EOF),
                }))
            }
        };

        Ok(Node::Variable(Box::new(var_name), Box::new(value)))
    }

    pub fn parse_function_definition(&mut self, tokens: &mut Stack) -> Result<Node, ParseError> {
        let mut nodes: Vec<Node> = Vec::new();
        let mut pcounter = 1;
        let mut counter = 0;

        while let Some(token) = tokens.pop_front() {
            match token {
                Token::VariableDefinition => {
                    nodes.push(self.parse_variables(tokens)?);
                }

                Token::Symbol(s) => {
                    nodes.push(Node::Symbol(s));
                }

                Token::Integer(n) => {
                    nodes.push(Node::Integer(n));
                }

                Token::Bool(b) => {
                    nodes.push(Node::Bool(b));
                }

                Token::LeftRoundBracket => {
                    // counter exists to count the expression brackets, both left and right
                    counter += 1;
                    if counter > 0 {
                        // if it is a nested expression, it will add to the node
                        // otherwise it will skip this and carry on
                        nodes.push(self.parse_expression(tokens)?)
                    }
                }

                Token::RightRoundBracket => {
                    // counter exists to count the expression brackets, both left and right
                    counter -= 1;
                    if counter == 0 {
                        // if it is a nested expression, it will carry on reducing the counter
                        // otherwise it will break to signify end of the expression
                        break;
                    }
                }

                Token::LeftSquareBracket => {
                    pcounter += 1;
                    if pcounter > 1 {
                        nodes.push(self.parse_parameter(tokens)?)
                    }
                }

                Token::RightSquareBracket => {
                    pcounter -= 1;
                    if pcounter == 0 {
                        break;
                    }
                }

                Token::DocString(s) => nodes.push(Node::DocString(s)),

                Token::FunctionDefinition => continue,

                _ => {
                    return Err(ParseError::TokenError(TokenError {
                        message: "Function Definition: Unexpected token",
                        token: token,
                    }));
                }
            }
        }

        Ok(Node::FunctionDefinition(nodes))
    }

    fn parse_parameter(&mut self, tokens: &mut Stack) -> Result<Node, ParseError> {
        let mut nodes = Vec::new();
        let mut counter = 1;

        while let Some(token) = tokens.pop_front() {
            match token {
                Token::Symbol(s) => {
                    nodes.push(Node::Symbol(s));
                }
                Token::Integer(n) => {
                    nodes.push(Node::Integer(n));
                }

                Token::Bool(b) => {
                    nodes.push(Node::Bool(b));
                }

                // only for nested lists
                Token::LeftSquareBracket => {
                    counter += 1;
                    if counter > 1 {
                        nodes.push(self.parse_list(tokens)?)
                    }
                }

                // only for nested lists
                Token::RightSquareBracket => {
                    counter -= 1;
                    if counter == 0 {
                        break;
                    }
                }

                _ => {
                    return Err(ParseError::TokenError(TokenError {
                        message: "Parameter: Unexpected token",
                        token: token,
                    }));
                }
            }
        }

        Ok(Node::Parameter(nodes))
    }

    fn parse_list(&mut self, tokens: &mut Stack) -> Result<Node, ParseError> {
        let mut nodes = Vec::new();
        let mut counter = 1;

        while let Some(token) = tokens.pop_front() {
            match token {
                Token::Integer(n) => {
                    nodes.push(Node::Integer(n));
                }

                Token::Bool(b) => {
                    nodes.push(Node::Bool(b));
                }

                // only for nested lists
                Token::LeftSquareBracket => {
                    counter += 1;
                    if counter > 1 {
                        nodes.push(self.parse_list(tokens)?)
                    }
                }

                // only for nested lists
                Token::RightSquareBracket => {
                    counter -= 1;
                    if counter == 0 {
                        break;
                    }
                }

                _ => {
                    return Err(ParseError::TokenError(TokenError {
                        message: "List: Unexpected token",
                        token: token,
                    }));
                }
            }
        }

        Ok(Node::List(nodes))
    }

    fn parse_expression(&mut self, tokens: &mut Stack) -> Result<Node, ParseError> {
        let mut nodes = Vec::new();

        let mut counter = 0;

        while let Some(token) = tokens.pop_front() {
            match token {
                Token::VariableDefinition => {
                    nodes.push(self.parse_variables(tokens)?);
                }
                Token::LeftRoundBracket => {
                    // counter exists to count the expression brackets, both left and right
                    counter += 1;
                    if counter > 1 {
                        // if it is a nested expression, it will add to the node
                        // otherwise it will skip this and carry on
                        nodes.push(self.parse_expression(tokens)?)
                    }
                }
                Token::Symbol(s) => {
                    nodes.push(Node::Symbol(s));
                }

                Token::StringLiteral(s) => {
                    nodes.push(Node::StringLiteral(s));
                }
                Token::Integer(n) => {
                    nodes.push(Node::Integer(n));
                }

                Token::Bool(b) => {
                    nodes.push(Node::Bool(b));
                }

                Token::RightRoundBracket => {
                    // counter exists to count the expression brackets, both left and right
                    counter -= 1;
                    if counter == 0 {
                        // if it is a nested expression, it will carry on reducing the counter
                        // otherwise it will break to signify end of the expression
                        break;
                    }
                }

                Token::LeftSquareBracket => nodes.push(self.parse_list(tokens)?),

                Token::RightSquareBracket => {
                    continue;
                }

                Token::FunctionDefinition => continue,

                _ => {
                    return Err(ParseError::TokenError(TokenError {
                        message: "Expression: Unexpected token",
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
        let mut program = Parser::new();
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
                panic!("{:?}", e)
            }
        }
    }

    #[test]
    fn test_parse_nested() {
        let code = "(+ 1 (+ 1 2))".to_string();
        let mut tokens = tokenise(code);
        let mut program = Parser::new();
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
                panic!("{:?}", e)
            }
        }
    }

    #[test]
    fn test_list() {
        let code = "(+ [1 2 3])".to_string();
        let mut tokens = tokenise(code);
        let mut program = Parser::new();
        match program.parse(&mut tokens) {
            Ok(list) => {
                assert_eq!(
                    list,
                    Node::Expression(vec![
                        Node::Symbol("+".to_string()),
                        Node::List(vec![Node::Integer(1), Node::Integer(2), Node::Integer(3)])
                    ])
                );
            }

            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }

    #[test]
    fn test_function_definition() {
        let code = r#"(defn hi [name] "lmao" (+ 1 1))"#.to_string();
        let mut tokens = tokenise(code);
        let mut program = Parser::new();
        match program.parse(&mut tokens) {
            Ok(list) => {
                assert_eq!(
                    list,
                    Node::FunctionDefinition(vec![
                        Node::Symbol("hi".to_string()),
                        Node::Parameter(vec![Node::Symbol("name".to_string()),]),
                        Node::DocString("lmao".to_string()),
                        Node::Expression(vec![
                            Node::Symbol("+".to_string()),
                            Node::Integer(1),
                            Node::Integer(1)
                        ])
                    ])
                );
            }

            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }

    #[test]
    fn test_function_add() {
        let code = "(defn add [x y] (+ x y))".to_string();
        let mut tokens = tokenise(code);
        let mut program = Parser::new();
        match program.parse(&mut tokens) {
            Ok(list) => {
                assert_eq!(
                    list,
                    Node::FunctionDefinition(vec![
                        Node::Symbol("add".to_string()),
                        Node::Parameter(vec![
                            Node::Symbol("x".to_string()),
                            Node::Symbol("y".to_string()),
                        ]),
                        Node::Expression(vec![
                            Node::Symbol("+".to_string()),
                            Node::Symbol("x".to_string()),
                            Node::Symbol("y".to_string()),
                        ])
                    ])
                );
            }

            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }

    #[test]
    fn test_variable() {
        let code = r#"(setq v "hi")"#.to_string();
        let mut tokens = tokenise(code);
        let mut program = Parser::new();
        match program.parse(&mut tokens) {
            Ok(list) => {
                assert_eq!(
                    list,
                    Node::Variable(
                        Box::new(Node::Symbol("v".to_string())),
                        Box::new(Node::StringLiteral("hi".to_string()))
                    )
                );
            }

            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }

    #[test]
    fn test_variable_call() {
        let code = r#"v"#.to_string();
        let mut tokens = tokenise(code);
        let mut program = Parser::new();
        assert_eq!(tokens.data, vec![Token::Symbol("v".to_string())]);
        match program.parse(&mut tokens) {
            Ok(list) => {
                assert_eq!(list, Node::Expression(vec![Node::Symbol("v".to_string())]));
            }

            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }
}
