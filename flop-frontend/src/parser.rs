use crate::error::ParseError;
use crate::lexer::{Token, TokenKind};
use crate::stack::Stack;

#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Void,
    StringLiteral(Token),
    Integer(Token),
    Bool(Token),
    Symbol(Token),
    List(Vec<Node>),
    Expression(Vec<Node>),
    FunctionDefinition(Vec<Node>),
    Parameter(Vec<Node>),
    Conditional(Vec<Node>),
    DocString(Token),
    Variable(Box<Node>, Box<Node>),
}

pub fn parse(tokens: &mut Stack<Token>) -> Result<Node, ParseError> {
    // rewrite this to avoid parsing issues
    let nodes = match tokens.data[0].token_kind {
        TokenKind::FunctionDefinition => parse_function_definition(tokens)?,
        TokenKind::VariableDefinition => parse_variables(tokens)?,
        TokenKind::Conditional => parse_conditional(tokens)?,
        _ => parse_expression(tokens)?,
    };

    if tokens.is_empty() {
        Ok(nodes)
    } else {
        Err(ParseError::StackError("Extra tokens remaining"))
    }
}

fn parse_parameter(tokens: &mut Stack<Token>) -> Result<Node, ParseError> {
    let nodes = parse_square_brakets(tokens)?;
    Ok(Node::Parameter(nodes))
}

fn parse_list(tokens: &mut Stack<Token>) -> Result<Node, ParseError> {
    let nodes = parse_square_brakets(tokens)?;
    Ok(Node::List(nodes))
}

fn parse_expression(tokens: &mut Stack<Token>) -> Result<Node, ParseError> {
    let token = parse_token(tokens)?;
    Ok(Node::Expression(token))
}

fn parse_variables(tokens: &mut Stack<Token>) -> Result<Node, ParseError> {
    // Expecting a variable name after `setq`

    // pop Node::Expression
    let _exp = tokens.pop_front();

    let var_token = tokens.pop_front().ok_or(ParseError::StackError(
        "Failed to pop stack for variable name",
    ))?;

    let var_name = match var_token.token_kind {
        TokenKind::Symbol => Node::Symbol(var_token),
        _ => {
            return Err(ParseError::TokenError {
                message: "Variable Definition: Expected a variable name",
                token: var_token,
            })
        }
    };

    let value_token = tokens.pop_front().ok_or(ParseError::StackError(
        "Failed to pop stack for variable reference",
    ))?;

    let value = match value_token.token_kind {
        TokenKind::Integer => Node::Integer(value_token),
        TokenKind::Bool => Node::Bool(value_token),
        TokenKind::StringLiteral => Node::StringLiteral(value_token),
        TokenKind::LeftRoundBracket => parse_expression(tokens)?,
        _ => {
            return Err(ParseError::TokenError {
                message: "Variable assignment: Expected a variable value",
                token: value_token,
            })
        }
    };

    Ok(Node::Variable(Box::new(var_name), Box::new(value)))
}

// (+ 1 2)
// (/ (- 3 4) 5)
// (defn add [x y] "lma" (+ 1 2) )

fn parse_token(tokens: &mut Stack<Token>) -> Result<Vec<Node>, ParseError> {
    let mut nodes = Vec::new();

    let mut counter = 0;

    while let Some(token) = tokens.pop_front() {
        match token.token_kind {
            TokenKind::VariableDefinition => {
                nodes.push(parse_variables(tokens)?);
            }
            TokenKind::LeftRoundBracket => {
                // counter exists to count the expression brackets, both left and right
                counter += 1;
                if counter > 1 {
                    // if it is a nested expression, it will add to the node
                    // otherwise it will skip this and carry on
                    nodes.push(parse_expression(tokens)?)
                }
            }
            TokenKind::Symbol => {
                nodes.push(Node::Symbol(token));
            }

            TokenKind::StringLiteral => {
                nodes.push(Node::StringLiteral(token));
            }
            TokenKind::Integer => {
                nodes.push(Node::Integer(token));
            }

            TokenKind::Bool => {
                nodes.push(Node::Bool(token));
            }

            TokenKind::RightRoundBracket => {
                // counter exists to count the expression brackets, both left and right
                counter -= 1;
                if counter <= 1 {
                    // if it is a nested expression, it will carry on reducing the counter
                    // otherwise it will break to signify end of the expression
                    break;
                }
            }

            TokenKind::LeftSquareBracket => nodes.push(parse_list(tokens)?),

            TokenKind::RightSquareBracket => {
                continue;
            }

            TokenKind::FunctionDefinition => continue,

            _ => {
                return Err(ParseError::TokenError {
                    message: "Expression: Unexpected token",
                    token,
                });
            }
        }
    }

    if counter != 0 {
        return Err("failed to parse");
    }

    Ok(nodes)
}

// a special case expression
// (if
// (exp)
// (exp))

fn parse_conditional(tokens: &mut Stack<Token>) -> Result<Node, ParseError> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut pcounter = 1;
    let mut counter = 0;

    while let Some(token) = tokens.pop_front() {
        match token.token_kind {
            TokenKind::Conditional => continue,

            TokenKind::VariableDefinition => {
                nodes.push(parse_variables(tokens)?);
            }

            TokenKind::Symbol => {
                nodes.push(Node::Symbol(token));
            }

            TokenKind::Integer => {
                nodes.push(Node::Integer(token));
            }

            TokenKind::Bool => {
                nodes.push(Node::Bool(token));
            }

            TokenKind::LeftRoundBracket => {
                // counter exists to count the expression brackets, both left and right
                counter += 1;
                if counter > 0 {
                    // if it is a nested expression, it will add to the node
                    // otherwise it will skip this and carry on
                    nodes.push(parse_expression(tokens)?)
                }
            }

            TokenKind::RightRoundBracket => {
                // counter exists to count the expression brackets, both left and right
                counter -= 1;
                if counter == 0 {
                    // if it is a nested expression, it will carry on reducing the counter
                    // otherwise it will break to signify end of the expression
                    break;
                }
            }

            TokenKind::LeftSquareBracket => {
                pcounter += 1;
                if pcounter > 1 {
                    nodes.push(parse_parameter(tokens)?)
                }
            }

            TokenKind::RightSquareBracket => {
                pcounter -= 1;
                if pcounter == 0 {
                    break;
                }
            }

            _ => {
                return Err(ParseError::TokenError {
                    message: "Conditional statement: Unexpected token",
                    token,
                });
            }
        }
    }
    Ok(Node::Conditional(nodes))
}

fn parse_square_brakets(tokens: &mut Stack<Token>) -> Result<Vec<Node>, ParseError> {
    let mut nodes = Vec::new();
    let mut counter = 1;

    while let Some(token) = tokens.pop_front() {
        match token.token_kind {
            TokenKind::Symbol => {
                nodes.push(Node::Symbol(token));
            }
            TokenKind::Integer => {
                nodes.push(Node::Integer(token));
            }

            TokenKind::Bool => {
                nodes.push(Node::Bool(token));
            }

            // only for nested lists
            TokenKind::LeftSquareBracket => {
                counter += 1;
                if counter > 1 {
                    nodes.push(parse_list(tokens)?)
                }
            }
            // only for nested lists
            TokenKind::RightSquareBracket => {
                counter -= 1;
                if counter == 0 {
                    break;
                }
            }

            _ => {
                return Err(ParseError::TokenError {
                    message: "Parameter: Unexpected token",
                    token,
                });
            }
        }
    }
    Ok(nodes)
}

fn parse_function_definition(tokens: &mut Stack<Token>) -> Result<Node, ParseError> {
    let mut nodes: Vec<Node> = Vec::new();
    let mut pcounter = 1;
    let mut counter = 0;

    while let Some(token) = tokens.pop_front() {
        match token.token_kind {
            TokenKind::VariableDefinition => {
                nodes.push(parse_variables(tokens)?);
            }

            TokenKind::Symbol => {
                nodes.push(Node::Symbol(token));
            }

            TokenKind::Integer => {
                nodes.push(Node::Integer(token));
            }

            TokenKind::Bool => {
                nodes.push(Node::Bool(token));
            }

            TokenKind::LeftRoundBracket => {
                // counter exists to count the expression brackets, both left and right
                counter += 1;
                if counter > 0 {
                    // if it is a nested expression, it will add to the node
                    // otherwise it will skip this and carry on
                    nodes.push(parse_expression(tokens)?)
                }
            }

            TokenKind::RightRoundBracket => {
                // counter exists to count the expression brackets, both left and right
                counter -= 1;
                if counter == 0 {
                    // if it is a nested expression, it will carry on reducing the counter
                    // otherwise it will break to signify end of the expression
                    break;
                }
            }

            TokenKind::LeftSquareBracket => {
                pcounter += 1;
                if pcounter > 1 {
                    nodes.push(parse_parameter(tokens)?)
                }
            }

            TokenKind::RightSquareBracket => {
                pcounter -= 1;
                if pcounter == 0 {
                    break;
                }
            }

            TokenKind::DocString => nodes.push(Node::DocString(token)),

            TokenKind::FunctionDefinition => continue,

            _ => {
                return Err(ParseError::TokenError {
                    message: "Function Definition: Unexpected token",
                    token,
                });
            }
        }
    }

    Ok(Node::FunctionDefinition(nodes))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenise;

    #[test]
    fn test_parse() {
        let code = "(+ 1 2)".to_string();
        let mut tokens = tokenise(code);

        match parse(&mut tokens) {
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

        match parse(&mut tokens) {
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

        match parse(&mut tokens) {
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

        match parse(&mut tokens) {
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

        match parse(&mut tokens) {
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

        match parse(&mut tokens) {
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

        assert_eq!(tokens.data, vec![TokenKind::Symbol("v".to_string())]);
        match parse(&mut tokens) {
            Ok(list) => {
                assert_eq!(list, Node::Expression(vec![Node::Symbol("v".to_string())]));
            }

            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }

    #[test]
    fn test_conditional() {
        let code = r#"(if (> 1 2) (print "1") (print 2))"#.to_string();
        let mut tokens = tokenise(code);

        assert_eq!(
            tokens.data,
            vec![
                TokenKind::Conditional,
                TokenKind::LeftRoundBracket,
                TokenKind::Symbol(">".to_string()),
                TokenKind::Integer(1),
                TokenKind::Integer(2),
                TokenKind::RightRoundBracket,
                TokenKind::LeftRoundBracket,
                TokenKind::Symbol("print".to_string()),
                TokenKind::StringLiteral("1".to_string()),
                TokenKind::RightRoundBracket,
                TokenKind::LeftRoundBracket,
                TokenKind::Symbol("print".to_string()),
                TokenKind::Integer(2),
                TokenKind::RightRoundBracket,
            ]
        );

        match parse(&mut tokens) {
            Ok(list) => {
                assert_eq!(
                    list,
                    Node::Conditional(vec![
                        Node::Expression(vec![
                            Node::Symbol(">".to_string()),
                            Node::Integer(1),
                            Node::Integer(2)
                        ]),
                        Node::Expression(vec![
                            Node::Symbol("print".to_string()),
                            Node::StringLiteral("1".to_string()),
                        ]),
                        Node::Expression(
                            vec![Node::Symbol("print".to_string()), Node::Integer(2),]
                        )
                    ])
                );
            }

            Err(e) => {
                panic!("{:?}", e)
            }
        }
    }
}
