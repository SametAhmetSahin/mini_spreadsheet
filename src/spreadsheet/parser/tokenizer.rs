use crate::common_types::Token;

pub struct ExpressionTokenizer {
    index: usize,
    chars: Vec<char>,
}

#[derive(Debug)]
pub enum TokenizeError {
    UnexpectedCharacter(char),
    InvalidCellName(String),
    InvalidNumber(String),
}

impl ExpressionTokenizer {
    pub fn new(chars: Vec<char>) -> Self {
        Self { index: 0, chars }
    }

    pub fn tokenize_expression(&mut self) -> Result<Vec<Token>, TokenizeError> {
        self.skip_whitespace();
        let mut expr_tokens = Vec::new();
        while !self.is_done() {
            let token = match self.peek().expect("Should never fail") {
                '+' | '-' | '/' | '*' | '(' | ')' | ':' | ',' => self.parse_operator(),
                '=' | '!' | '>' | '<' | '&' | '|' => self.parse_logical_operator()?,
                letter if letter.is_uppercase() => self.parse_cell_name_or_bool()?,
                letter if letter.is_lowercase() => self.parse_function_name()?,
                digit if digit.is_ascii_digit() => self.parse_number()?,
                unknown => return Err(TokenizeError::UnexpectedCharacter(*unknown)),
            };

            expr_tokens.push(token);

            self.skip_whitespace();
        }

        Ok(expr_tokens)
    }

    fn parse_cell_name_or_bool(&mut self) -> Result<Token, TokenizeError> {
        // [A-Z]+\d+

        let mut is_valid = false;
        let mut letters = String::new();

        // Collect the uppercase letters
        while let Some(&ch) = self.peek() {
            if ch.is_ascii_uppercase() {
                letters.push(ch);
                self.pop();
            } else {
                break;
            }
        }

        if letters == "TRUE" {
            return Ok(Token::Bool(true));
        }

        if letters == "FALSE" {
            return Ok(Token::Bool(false));
        }

        // At this point we know that we are parsing a Cell Name

        // Ensure there are letters
        if letters.is_empty() {
            return Err(TokenizeError::InvalidCellName(String::new()));
        }

        // Collect the digits
        while let Some(&ch) = self.peek() {
            if ch.is_ascii_digit() {
                letters.push(ch);
                self.pop();
                is_valid = true;
            } else {
                break;
            }
        }

        // Ensure the format was valid ``
        if !is_valid {
            return Err(TokenizeError::InvalidCellName(letters));
        }

        Ok(Token::CellName(letters))
    }

    fn parse_operator(&mut self) -> Token {
        match self.pop().expect("Shoud never fail") {
            '+' => Token::Plus,
            '-' => Token::Minus,
            '/' => Token::Division,
            '*' => Token::Multiply,
            '(' => Token::LParen,
            ')' => Token::RParen,
            ':' => Token::Colon,
            ',' => Token::Comma,
            _ => unreachable!(),
        }
    }

    fn peek(&self) -> Option<&char> {
        self.chars.get(self.index)
    }

    fn is_done(&self) -> bool {
        self.index >= self.chars.len()
    }

    fn pop(&mut self) -> Option<&char> {
        let val = self.chars.get(self.index);
        self.index += 1;
        val
    }

    fn skip_whitespace(&mut self) -> bool {
        while !self.is_done() && self.peek().expect("Should not fail").is_ascii_whitespace() {
            self.pop();
        }
        // Some error occured
        if self.is_done() {
            return false;
        }
        true
    }

    fn parse_number(&mut self) -> Result<Token, TokenizeError> {
        let mut number = String::new();
        while let Some(&ch) = self.peek() {
            if ch.is_ascii_digit() || ch == '.' {
                number.push(ch);
                self.pop();
            } else {
                break;
            }
        }

        match number.parse() {
            Ok(inner) => Ok(Token::Number(inner)),
            Err(_) => Err(TokenizeError::InvalidNumber(number)),
        }
    }

    fn parse_function_name(&mut self) -> Result<Token, TokenizeError> {
        let mut name = String::new();
        while let Some(&ch) = self.peek() {
            if ch.is_ascii_alphabetic() || ch == '_' {
                name.push(ch);
                self.pop();
            } else {
                break;
            }
        }

        Ok(Token::FunctionName(name))
    }

    fn parse_logical_operator(&mut self) -> Result<Token, TokenizeError> {
        let first = self.pop().expect("Should never fail");
        let token = match first {
            '=' => {
                if let Some('=') = self.peek() {
                    self.pop();
                    Token::Equals
                } else {
                    return Err(TokenizeError::UnexpectedCharacter('='));
                }
            }
            '!' => {
                if let Some('=') = self.peek() {
                    self.pop();
                    Token::NotEquals
                } else {
                    Token::Not
                }
            }
            '>' => {
                if let Some('=') = self.peek() {
                    self.pop();
                    Token::GreaterEquals
                } else {
                    Token::GreaterThan
                }
            }
            '<' => {
                if let Some('=') = self.peek() {
                    self.pop();
                    Token::LessEquals
                } else {
                    Token::LessThan
                }
            }
            '&' => {
                if let Some('&') = self.peek() {
                    self.pop();
                    Token::And
                } else {
                    return Err(TokenizeError::UnexpectedCharacter('&'));
                }
            }
            '|' => {
                if let Some('|') = self.peek() {
                    self.pop();
                    Token::Or
                } else {
                    return Err(TokenizeError::UnexpectedCharacter('|'));
                }
            }
            _ => unreachable!(),
        };
        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_expression() {
        let s = "A1 + A2";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::CellName("A1".to_string()),
                Token::Plus,
                Token::CellName("A2".to_string())
            ]
        );
    }

    #[test]
    fn test_expression_with_parentheses() {
        let s = "(A1 + B2) * C3";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::CellName("A1".to_string()),
                Token::Plus,
                Token::CellName("B2".to_string()),
                Token::RParen,
                Token::Multiply,
                Token::CellName("C3".to_string())
            ]
        );
    }

    #[test]
    fn test_expression_with_division_and_whitespace() {
        let s = "  A1   /   B2 ";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::CellName("A1".to_string()),
                Token::Division,
                Token::CellName("B2".to_string())
            ]
        );
    }

    #[test]
    fn test_complex_expression() {
        let s = "((A1 + B2) - C3) * D4 / E5";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::LParen,
                Token::CellName("A1".to_string()),
                Token::Plus,
                Token::CellName("B2".to_string()),
                Token::RParen,
                Token::Minus,
                Token::CellName("C3".to_string()),
                Token::RParen,
                Token::Multiply,
                Token::CellName("D4".to_string()),
                Token::Division,
                Token::CellName("E5".to_string())
            ]
        );
    }

    #[test]
    fn test_empty_expression() {
        let s = "";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert!(
            tokens.is_empty(),
            "Expected empty token list for empty expression"
        );
    }

    #[test]
    fn test_expression_with_extra_whitespace() {
        let s = "   A1    +     A2   ";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::CellName("A1".to_string()),
                Token::Plus,
                Token::CellName("A2".to_string())
            ]
        );
    }

    #[test]
    fn test_expression_with_numbers() {
        let s = "3.14 + 42";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![Token::Number(3.14), Token::Plus, Token::Number(42.0),]
        );
    }

    #[test]
    fn test_expression_with_invalid_cell_name() {
        let s = "A + B2";
        let result = ExpressionTokenizer::new(s.chars().collect()).tokenize_expression();
        assert!(matches!(result, Err(TokenizeError::InvalidCellName(_))));
    }

    #[test]
    fn test_expression_with_invalid_number() {
        let s = "42.3.14 + B2";
        let result = ExpressionTokenizer::new(s.chars().collect()).tokenize_expression();
        assert!(matches!(result, Err(TokenizeError::InvalidNumber(_))));
    }

    #[test]
    fn test_expression_with_unexpected_character() {
        let s = "A1 + $B2";
        let result = ExpressionTokenizer::new(s.chars().collect()).tokenize_expression();
        assert!(matches!(
            result,
            Err(TokenizeError::UnexpectedCharacter('$'))
        ));
    }

    #[test]
    fn test_expression_with_nested_parentheses() {
        let s = "(((A1))) + B2";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::LParen,
                Token::LParen,
                Token::CellName("A1".to_string()),
                Token::RParen,
                Token::RParen,
                Token::RParen,
                Token::Plus,
                Token::CellName("B2".to_string())
            ]
        );
    }

    #[test]
    fn test_expression_with_negative_numbers() {
        let s = "-42.5 * (3 + 4)";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Minus,
                Token::Number(42.5),
                Token::Multiply,
                Token::LParen,
                Token::Number(3.0),
                Token::Plus,
                Token::Number(4.0),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_expression_with_trailing_whitespace() {
        let s = "A1 + B2    ";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::CellName("A1".to_string()),
                Token::Plus,
                Token::CellName("B2".to_string()),
            ]
        );
    }

    #[test]
    fn test_expression_with_multiple_digits_in_cell_name() {
        let s = "A123 + B456";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::CellName("A123".to_string()),
                Token::Plus,
                Token::CellName("B456".to_string()),
            ]
        );
    }

    #[test]
    fn test_expression_with_only_whitespace() {
        let s = "    ";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert!(
            tokens.is_empty(),
            "Expected empty token list for expression with only whitespace"
        );
    }

    #[test]
    fn test_expression_with_complex_numbers() {
        let s = "123.45 * 67.89";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![Token::Number(123.45), Token::Multiply, Token::Number(67.89),]
        );
    }

    #[test]
    fn test_expression_with_function_and_range() {
        let s = "sum(A1:B1)";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::FunctionName("sum".to_string()),
                Token::LParen,
                Token::CellName("A1".to_string()),
                Token::Colon,
                Token::CellName("B1".to_string()),
                Token::RParen
            ]
        );
    }

    #[test]
    fn test_expression_with_function_multiple_args() {
        let s = "sum(A1, C1)";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::FunctionName("sum".to_string()),
                Token::LParen,
                Token::CellName("A1".to_string()),
                Token::Comma,
                Token::CellName("C1".to_string()),
                Token::RParen
            ]
        );
    }

    #[test]
    fn test_simple_comparison() {
        let s = "A1 == B1";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::CellName("A1".to_string()),
                Token::Equals,
                Token::CellName("B1".to_string()),
            ]
        );
    }

    #[test]
    fn test_complex_logical_expression() {
        let s = "A1 > B1 && C1 <= D1";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::CellName("A1".to_string()),
                Token::GreaterThan,
                Token::CellName("B1".to_string()),
                Token::And,
                Token::CellName("C1".to_string()),
                Token::LessEquals,
                Token::CellName("D1".to_string()),
            ]
        );
    }

    #[test]
    fn test_not_equals_and_or() {
        let s = "A1 != B1 || C1 != D1";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::CellName("A1".to_string()),
                Token::NotEquals,
                Token::CellName("B1".to_string()),
                Token::Or,
                Token::CellName("C1".to_string()),
                Token::NotEquals,
                Token::CellName("D1".to_string()),
            ]
        );
    }

    #[test]
    fn test_logical_with_arithmetic() {
        let s = "A1 + B1 > C1 * D1";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::CellName("A1".to_string()),
                Token::Plus,
                Token::CellName("B1".to_string()),
                Token::GreaterThan,
                Token::CellName("C1".to_string()),
                Token::Multiply,
                Token::CellName("D1".to_string()),
            ]
        );
    }

    #[test]
    fn test_logical_with_function() {
        let s = "sum(A1, B1) >= C1";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::FunctionName("sum".to_string()),
                Token::LParen,
                Token::CellName("A1".to_string()),
                Token::Comma,
                Token::CellName("B1".to_string()),
                Token::RParen,
                Token::GreaterEquals,
                Token::CellName("C1".to_string()),
            ]
        );
    }

    #[test]
    fn test_not_operator() {
        let s = "!A1";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(tokens, vec![Token::Not, Token::CellName("A1".to_string()),]);
    }

    #[test]
    fn test_complex_nested_expression() {
        let s = "(A1 > B1 && C1 < D1) || E1 == F1";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::CellName("A1".to_string()),
                Token::GreaterThan,
                Token::CellName("B1".to_string()),
                Token::And,
                Token::CellName("C1".to_string()),
                Token::LessThan,
                Token::CellName("D1".to_string()),
                Token::RParen,
                Token::Or,
                Token::CellName("E1".to_string()),
                Token::Equals,
                Token::CellName("F1".to_string()),
            ]
        );
    }

    #[test]
    fn test_invalid_operators() {
        // Single = is invalid
        let s = "A1 = B1";
        assert!(ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .is_err());

        // Single & is invalid
        let s = "A1 & B1";
        assert!(ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .is_err());

        // Single | is invalid
        let s = "A1 | B1";
        assert!(ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .is_err());
    }

    #[test]
    fn test_bool() {
        let s = "TRUE != FALSE || FALSE != TRUE";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Bool(true),
                Token::NotEquals,
                Token::Bool(false),
                Token::Or,
                Token::Bool(false),
                Token::NotEquals,
                Token::Bool(true),
            ]
        );
    }

    #[test]
    fn test_simple_boolean() {
        let s = "TRUE";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(tokens, vec![Token::Bool(true)]);

        let s = "FALSE";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(tokens, vec![Token::Bool(false)]);
    }

    #[test]
    fn test_boolean_comparison() {
        let s = "A1 == TRUE";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::CellName("A1".to_string()),
                Token::Equals,
                Token::Bool(true),
            ]
        );
    }

    #[test]
    fn test_boolean_logical_operators() {
        let s = "TRUE && FALSE || TRUE";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::Bool(true),
                Token::And,
                Token::Bool(false),
                Token::Or,
                Token::Bool(true),
            ]
        );
    }

    #[test]
    fn test_not_boolean() {
        let s = "!TRUE";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(tokens, vec![Token::Not, Token::Bool(true),]);
    }

    #[test]
    fn test_boolean_in_function() {
        let s = "if(A1 > 10, TRUE, FALSE)";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::FunctionName("if".to_string()),
                Token::LParen,
                Token::CellName("A1".to_string()),
                Token::GreaterThan,
                Token::Number(10.0),
                Token::Comma,
                Token::Bool(true),
                Token::Comma,
                Token::Bool(false),
                Token::RParen,
            ]
        );
    }

    #[test]
    fn test_complex_boolean_expression() {
        let s = "(A1 > B1 && TRUE) || (C1 == FALSE && !D1)";
        let tokens = ExpressionTokenizer::new(s.chars().collect())
            .tokenize_expression()
            .unwrap();
        assert_eq!(
            tokens,
            vec![
                Token::LParen,
                Token::CellName("A1".to_string()),
                Token::GreaterThan,
                Token::CellName("B1".to_string()),
                Token::And,
                Token::Bool(true),
                Token::RParen,
                Token::Or,
                Token::LParen,
                Token::CellName("C1".to_string()),
                Token::Equals,
                Token::Bool(false),
                Token::And,
                Token::Not,
                Token::CellName("D1".to_string()),
                Token::RParen,
            ]
        );
    }

   
}
