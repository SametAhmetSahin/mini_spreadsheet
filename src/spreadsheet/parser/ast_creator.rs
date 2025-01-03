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
        self.parse_expression(0)
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
            Some(Token::CellName(n)) => Ok(AST::CellName(n)),
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
            Some(Token::Plus | Token::Minus | Token::Multiply | Token::Division) => self.tokens.peek().cloned(),
            _ => None,
        }
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
}
