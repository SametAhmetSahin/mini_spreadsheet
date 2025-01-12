use std::iter::Peekable;

use crate::common_types::{Token, Value, AST};

pub struct ASTCreator<I>
where
    I: Iterator<Item = Token>,
{
    tokens: Peekable<I>,
}
#[derive(Debug)]
pub enum ASTCreateError {
    UnexpectedToken,
    MismatchedParentheses,
    InvalidRange,
}

impl<I> ASTCreator<I>
where
    I: Iterator<Item = Token>,
{
    pub fn new(tokens: I) -> Self {
        Self {
            tokens: tokens.peekable(),
        }
    }

    pub fn parse(&mut self) -> Result<crate::common_types::AST, ASTCreateError> {
        let result = self.parse_expression(0);
        if let Some(_) = self.tokens.next() {
            // We have not parsed all tokens
            Err(ASTCreateError::UnexpectedToken)
        } else {
            result
        }
    }

    fn parse_expression(&mut self, min_precedence: usize) -> Result<AST, ASTCreateError> {
        // Min presedence arguement is important for recursive calls where it may be higher than 1

        let mut left = self.parse_primary()?;

        while let Some(op) = self.peek_operator() {
            let precedence = op.get_precedence();
            if precedence < min_precedence {
                break;
            }
            self.tokens.next(); // Consume the operator

            let right = self.parse_expression(precedence + 1)?;

            left = AST::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<AST, ASTCreateError> {
        match self.tokens.next() {
            Some(Token::FunctionName(name)) => {
                self.expect_token(Token::LParen)?;
                let arguments = self.parse_function_arguements()?;
                Ok(AST::FunctionCall {
                    name,
                    arguments: arguments,
                })
            }
            Some(Token::CellName(name)) => {
                // Check if this might be the start of a range
                if let Some(Token::Colon) = self.tokens.peek() {
                    self.tokens.next(); // consume colon
                    match self.tokens.next() {
                        Some(Token::CellName(to_name)) => Ok(AST::Range {
                            from: name,
                            to: to_name,
                        }),
                        _ => Err(ASTCreateError::InvalidRange),
                    }
                } else {
                    Ok(AST::CellName(name))
                }
            }
            Some(Token::Number(n)) => Ok(AST::Value(Value::Number(n))),
            Some(Token::LParen) => {
                let expr = self.parse_expression(0)?;
                match self.tokens.next() {
                    Some(Token::RParen) => Ok(expr),
                    _ => Err(ASTCreateError::MismatchedParentheses),
                }
            }
            _ => Err(ASTCreateError::UnexpectedToken),
        }
    }

    fn peek_operator(&mut self) -> Option<Token> {
        match self.tokens.peek() {
            Some(Token::Plus | Token::Minus | Token::Multiply | Token::Division) => {
                self.tokens.peek().cloned()
            }
            _ => None,
        }
    }

    // Helper function to expect a specific token
    fn expect_token(&mut self, expected: Token) -> Result<(), ASTCreateError> {
        match self.tokens.next() {
            Some(token) if token == expected => Ok(()),
            _ => Err(ASTCreateError::UnexpectedToken),
        }
    }

    fn parse_function_arguements(&mut self) -> Result<Vec<AST>, ASTCreateError> {
        let mut arguements = Vec::new();

        let mut expecting_comma = false;

        loop {
            if !expecting_comma {
                expecting_comma = true;
                let arg = self.parse_expression(0)?;
                arguements.push(arg);
            } else {
                match self.tokens.next() {
                    Some(Token::Comma) => expecting_comma = false,
                    Some(Token::RParen) => break,
                    Some(_unexpected) => return Err(ASTCreateError::UnexpectedToken),
                    None => return Err(ASTCreateError::MismatchedParentheses),
                }
            }
        }

        Ok(arguements)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_cell_name() {
        let tokens = vec![Token::CellName("A1".to_string())];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let ast = parser.parse().unwrap();
        assert_eq!(ast, AST::CellName("A1".to_string()));
    }

    #[test]
    fn test_simple_addition() {
        let tokens = vec![
            Token::CellName("A1".to_string()),
            Token::Plus,
            Token::CellName("B2".to_string()),
        ];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            AST::BinaryOp {
                op: Token::Plus,
                left: Box::new(AST::CellName("A1".to_string())),
                right: Box::new(AST::CellName("B2".to_string())),
            }
        );
    }

    #[test]
    fn test_operator_precedence() {
        let tokens = vec![
            Token::CellName("A1".to_string()),
            Token::Plus,
            Token::CellName("B2".to_string()),
            Token::Multiply,
            Token::CellName("C3".to_string()),
        ];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            AST::BinaryOp {
                op: Token::Plus,
                left: Box::new(AST::CellName("A1".to_string())),
                right: Box::new(AST::BinaryOp {
                    op: Token::Multiply,
                    left: Box::new(AST::CellName("B2".to_string())),
                    right: Box::new(AST::CellName("C3".to_string())),
                }),
            }
        );
    }

    #[test]
    fn test_parentheses_override_precedence() {
        let tokens = vec![
            Token::LParen,
            Token::CellName("A1".to_string()),
            Token::Plus,
            Token::CellName("B2".to_string()),
            Token::RParen,
            Token::Multiply,
            Token::CellName("C3".to_string()),
        ];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            AST::BinaryOp {
                op: Token::Multiply,
                left: Box::new(AST::BinaryOp {
                    op: Token::Plus,
                    left: Box::new(AST::CellName("A1".to_string())),
                    right: Box::new(AST::CellName("B2".to_string())),
                }),
                right: Box::new(AST::CellName("C3".to_string())),
            }
        );
    }

    #[test]
    fn test_mismatched_parentheses() {
        let tokens = vec![
            Token::LParen,
            Token::CellName("A1".to_string()),
            Token::Plus,
            Token::CellName("B2".to_string()),
        ];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let result = parser.parse();
        assert!(matches!(result, Err(ASTCreateError::MismatchedParentheses)));
    }

    #[test]
    fn test_unexpected_token() {
        let tokens = vec![Token::Plus, Token::CellName("A1".to_string())];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let result = parser.parse();
        assert!(matches!(result, Err(ASTCreateError::UnexpectedToken)));
    }

    #[test]
    fn test_nested_parentheses() {
        let tokens = vec![
            Token::LParen,
            Token::LParen,
            Token::CellName("A1".to_string()),
            Token::Plus,
            Token::CellName("B2".to_string()),
            Token::RParen,
            Token::Multiply,
            Token::CellName("C3".to_string()),
            Token::RParen,
        ];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            AST::BinaryOp {
                op: Token::Multiply,
                left: Box::new(AST::BinaryOp {
                    op: Token::Plus,
                    left: Box::new(AST::CellName("A1".to_string())),
                    right: Box::new(AST::CellName("B2".to_string())),
                }),
                right: Box::new(AST::CellName("C3".to_string())),
            }
        );
    }

    #[test]
    fn test_simple_function_call() {
        let tokens = vec![
            Token::FunctionName("sum".to_string()),
            Token::LParen,
            Token::CellName("A1".to_string()),
            Token::RParen,
        ];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            AST::FunctionCall {
                name: "sum".to_string(),
                arguments: vec![AST::CellName("A1".to_string())],
            }
        );
    }

    #[test]
    fn test_function_call_multiple_arguments() {
        let tokens = vec![
            Token::FunctionName("average".to_string()),
            Token::LParen,
            Token::CellName("A1".to_string()),
            Token::Comma,
            Token::CellName("B2".to_string()),
            Token::Comma,
            Token::Number(42.0),
            Token::RParen,
        ];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            AST::FunctionCall {
                name: "average".to_string(),
                arguments: vec![
                    AST::CellName("A1".to_string()),
                    AST::CellName("B2".to_string()),
                    AST::Value(Value::Number(42.0)),
                ],
            }
        );
    }

    #[test]
    fn test_nested_function_calls() {
        let tokens = vec![
            Token::FunctionName("sum".to_string()),
            Token::LParen,
            Token::FunctionName("average".to_string()),
            Token::LParen,
            Token::CellName("A1".to_string()),
            Token::Comma,
            Token::CellName("B2".to_string()),
            Token::RParen,
            Token::RParen,
        ];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            AST::FunctionCall {
                name: "sum".to_string(),
                arguments: vec![AST::FunctionCall {
                    name: "average".to_string(),
                    arguments: vec![
                        AST::CellName("A1".to_string()),
                        AST::CellName("B2".to_string()),
                    ],
                }],
            }
        );
    }

    #[test]
    fn test_function_call_with_expression() {
        let tokens = vec![
            Token::FunctionName("max".to_string()),
            Token::LParen,
            Token::CellName("A1".to_string()),
            Token::Plus,
            Token::Number(10.0),
            Token::RParen,
        ];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            AST::FunctionCall {
                name: "max".to_string(),
                arguments: vec![AST::BinaryOp {
                    op: Token::Plus,
                    left: Box::new(AST::CellName("A1".to_string())),
                    right: Box::new(AST::Value(Value::Number(10.0))),
                }],
            }
        );
    }

    #[test]
    fn test_function_call_missing_parentheses() {
        let tokens = vec![
            Token::FunctionName("sum".to_string()),
            Token::CellName("A1".to_string()),
        ];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let result = parser.parse();
        assert!(matches!(result, Err(ASTCreateError::UnexpectedToken)));
    }

    #[test]
    fn test_function_call_missing_closing_parenthesis() {
        let tokens = vec![
            Token::FunctionName("sum".to_string()),
            Token::LParen,
            Token::CellName("A1".to_string()),
        ];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let result = parser.parse();
        assert!(matches!(result, Err(ASTCreateError::MismatchedParentheses)));
    }

    #[test]
    fn test_simple_range() {
        let tokens = vec![
            Token::CellName("A1".to_string()),
            Token::Colon,
            Token::CellName("B5".to_string()),
        ];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            AST::Range {
                from: "A1".to_string(),
                to: "B5".to_string(),
            }
        );
    }

    #[test]
    fn test_range_in_function() {
        let tokens = vec![
            Token::FunctionName("sum".to_string()),
            Token::LParen,
            Token::CellName("A1".to_string()),
            Token::Colon,
            Token::CellName("A10".to_string()),
            Token::RParen,
        ];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            AST::FunctionCall {
                name: "sum".to_string(),
                arguments: vec![AST::Range {
                    from: "A1".to_string(),
                    to: "A10".to_string(),
                }],
            }
        );
    }

    #[test]
    fn test_invalid_range_missing_second_cell() {
        let tokens = vec![
            Token::CellName("A1".to_string()),
            Token::Colon,
            Token::Number(42.0), // Should be a cell name
        ];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let result = parser.parse();
        assert!(matches!(result, Err(ASTCreateError::InvalidRange)));
    }

    #[test]
    fn test_invalid_range_missing_colon() {
        let tokens = vec![
            Token::CellName("A1".to_string()),
            Token::CellName("A10".to_string()),
        ];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let result = parser.parse();
        assert!(matches!(result, Err(ASTCreateError::UnexpectedToken)));
    }

    #[test]
    fn test_range_with_operation() {
        let tokens = vec![
            Token::CellName("A1".to_string()),
            Token::Colon,
            Token::CellName("A10".to_string()),
            Token::Plus,
            Token::Number(5.0),
        ];
        let mut parser = ASTCreator::new(tokens.into_iter());
        let ast = parser.parse().unwrap();
        assert_eq!(
            ast,
            AST::BinaryOp {
                op: Token::Plus,
                left: Box::new(AST::Range {
                    from: "A1".to_string(),
                    to: "A10".to_string(),
                }),
                right: Box::new(AST::Value(Value::Number(5.0))),
            }
        );
    }
}
