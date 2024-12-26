use std::iter::Peekable;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    CellName(String),
    Plus,
    Minus,
    Division,
    Multiply,
    LParen,
    RParen,
}

impl Token {
    fn get_precedence(&self) -> usize {
        match &self {
            Token::Plus | Token::Minus => 1,
            Token::Division | Token::Multiply => 2,
            _ => 0,
        }
    }
}

#[derive(Debug)]
pub enum AST {
    CellName(String),
    BinaryOp {
        op: Token,
        left: Box<AST>,
        right: Box<AST>,
    },
}

pub struct ASTCreator<I>
where
    I: Iterator<Item = Token>,
{
    tokens: Peekable<I>,
}
#[derive(Debug)]
pub enum ParseError {
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

    pub fn parse(&mut self) -> Result<AST, ParseError> {
        self.parse_expression(0)
    }

    fn parse_expression(&mut self, min_precedence: usize) -> Result<AST, ParseError> {
        // Min presedence arguement is important for recursive calls where it may be higher than 1

        let mut left = self.parse_primary()?;

        while let Some(op) = self.peek_operator() {
            let precedence = op.get_precedence();
            if precedence < min_precedence {
                break;
            }
            self.tokens.next(); // Consume the operator

            let mut right = self.parse_expression(precedence + 1)?;

            left = AST::BinaryOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }

        Ok(left)
    }

    fn parse_primary(&mut self) -> Result<AST, ParseError> {
        match self.tokens.next() {
            Some(Token::CellName(n)) => Ok(AST::CellName(n)),
            Some(Token::LParen) => {
                let expr = self.parse_expression(0)?;
                match self.tokens.next() {
                    Some(Token::RParen) => Ok(expr),
                    _ => Err(ParseError::MismatchedParentheses),
                }
            }
            _ => Err(ParseError::UnexpectedToken),
        }
    }

    fn peek_operator(&mut self) -> Option<Token> {
        match self.tokens.peek() {
            Some(Token::Plus)
            | Some(Token::Minus)
            | Some(Token::Multiply)
            | Some(Token::Division) => self.tokens.peek().cloned(),
            _ => None,
        }
    }
}
