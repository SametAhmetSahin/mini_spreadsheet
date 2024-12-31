use std::collections::HashMap;


use crate::common_types::{Index, Token, Value, AST};


pub trait VarContext {
    fn get_variable(&self, index: Index) -> Option<Value>;
}

struct MockVarContext {
    variables : HashMap<Index, Value>
}

impl VarContext for MockVarContext {
    fn get_variable(&self, index: Index) -> Option<Value> {
        self.variables.get(&index).cloned()
    }
}

impl MockVarContext {
    fn new(variables: HashMap<Index, Value>) -> Self {
        Self { variables }
    }
}

pub struct ASTResolver {}

impl ASTResolver {
    pub fn resolve(ast: AST, variables: &dyn VarContext) -> Value {
        match ast {
            AST::Value(value) => value,
            AST::CellName(name) => variables.get_variable(Self::get_cell_idx(&name)).unwrap(),
            AST::BinaryOp { op, left, right } => match op {
                Token => {
                    Self::resolve(*left, variables).add(Self::resolve(*right, variables)).unwrap()
                }
                Token::Minus => {
                    Self::resolve(*left, variables).sub(Self::resolve(*right, variables)).unwrap()
                }
                Token::Division => {
                    Self::resolve(*left, variables).div(Self::resolve(*right, variables)).unwrap()
                }
                Token::Multiply => {
                    Self::resolve(*left, variables).mult(Self::resolve(*right, variables)).unwrap()
                }
                other => panic!("{:?} is not a binary operator", other),
            },
        }
    }

    pub fn get_cell_idx(cell_name: &str) -> Index {
        let mut x: usize = 0;
        let mut y = 0;

        for (i, c) in cell_name.chars().enumerate() {
            if c.is_digit(10) {
                // Parse row number
                y = cell_name[i..].parse::<usize>().expect("Invalid row number");
                break;
            } else {
                // Parse column letters
                x = x * 26 + (c as usize - 'A' as usize + 1);
            }
        }

        // Adjust for 0-based indexing
        Index { x: x - 1, y: y - 1 }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_resolve_value_ast() {
        let variables = MockVarContext::new(HashMap::new());
        let ast = AST::Value(Value::Number(42.0));
        
        let result = ASTResolver::resolve(ast, &variables);
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_resolve_cellname_ast() {
        let mut vars = HashMap::new();
        vars.insert(Index { x: 0, y: 0 }, Value::Number(10.0));

        let variables = MockVarContext::new(vars);
        let ast = AST::CellName("A1".to_string());

        let result = ASTResolver::resolve(ast, &variables);
        assert_eq!(result, Value::Number(10.0));
    }

    #[test]
    fn test_resolve_binary_op_addition() {
        let mut vars = HashMap::new();
        vars.insert(Index { x: 0, y: 0 }, Value::Number(10.0));
        vars.insert(Index { x: 1, y: 0 }, Value::Number(20.0));

        let variables = MockVarContext::new(vars);
        let ast = AST::BinaryOp {
            op: Token::Plus,
            left: Box::new(AST::CellName("A1".to_string())),
            right: Box::new(AST::CellName("B1".to_string())),
        };

        let result = ASTResolver::resolve(ast, &variables);
        assert_eq!(result, Value::Number(30.0));
    }

    #[test]
    fn test_resolve_binary_op_subtraction() {
        let mut vars = HashMap::new();
        vars.insert(Index { x: 0, y: 0 }, Value::Number(30.0));
        vars.insert(Index { x: 1, y: 0 }, Value::Number(20.0));

        let variables = MockVarContext::new(vars);
        let ast = AST::BinaryOp {
            op: Token::Minus,
            left: Box::new(AST::CellName("A1".to_string())),
            right: Box::new(AST::CellName("B1".to_string())),
        };

        let result = ASTResolver::resolve(ast, &variables);
        assert_eq!(result, Value::Number(10.0));
    }

    #[test]
    fn test_resolve_binary_op_multiplication() {
        let mut vars = HashMap::new();
        vars.insert(Index { x: 0, y: 0 }, Value::Number(3.0));
        vars.insert(Index { x: 1, y: 0 }, Value::Number(4.0));

        let variables = MockVarContext::new(vars);
        let ast = AST::BinaryOp {
            op: Token::Multiply,
            left: Box::new(AST::CellName("A1".to_string())),
            right: Box::new(AST::CellName("B1".to_string())),
        };

        let result = ASTResolver::resolve(ast, &variables);
        assert_eq!(result, Value::Number(12.0));
    }

    #[test]
    fn test_resolve_binary_op_division() {
        let mut vars = HashMap::new();
        vars.insert(Index { x: 0, y: 0 }, Value::Number(20.0));
        vars.insert(Index { x: 1, y: 0 }, Value::Number(4.0));

        let variables = MockVarContext::new(vars);
        let ast = AST::BinaryOp {
            op: Token::Division,
            left: Box::new(AST::CellName("A1".to_string())),
            right: Box::new(AST::CellName("B1".to_string())),
        };

        let result = ASTResolver::resolve(ast, &variables);
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    #[should_panic]
    fn test_resolve_missing_cellname() {
        let variables = MockVarContext::new(HashMap::new());
        let ast = AST::CellName("A1".to_string());

        // This should panic because "A1" is not in the context
        ASTResolver::resolve(ast, &variables);
    }

    #[test]
    fn test_resolve_deep_tree_addition_multiplication() {
        let mut vars = HashMap::new();
        vars.insert(Index { x: 0, y: 0 }, Value::Number(2.0));
        vars.insert(Index { x: 1, y: 0 }, Value::Number(3.0));
        vars.insert(Index { x: 2, y: 0 }, Value::Number(4.0));

        let variables = MockVarContext::new(vars);
        let ast = AST::BinaryOp {
            op: Token::Plus,
            left: Box::new(AST::BinaryOp {
                op: Token::Multiply,
                left: Box::new(AST::CellName("A1".to_string())),
                right: Box::new(AST::CellName("B1".to_string())),
            }),
            right: Box::new(AST::CellName("C1".to_string())),
        };

        let result = ASTResolver::resolve(ast, &variables);
        assert_eq!(result, Value::Number(10.0));
    }

    #[test]
    fn test_resolve_deep_tree_subtraction_division() {
        let mut vars = HashMap::new();
        vars.insert(Index { x: 0, y: 0 }, Value::Number(20.0));
        vars.insert(Index { x: 1, y: 0 }, Value::Number(4.0));
        vars.insert(Index { x: 2, y: 0 }, Value::Number(2.0));

        let variables = MockVarContext::new(vars);
        let ast = AST::BinaryOp {
            op: Token::Minus,
            left: Box::new(AST::BinaryOp {
                op: Token::Division,
                left: Box::new(AST::CellName("A1".to_string())),
                right: Box::new(AST::CellName("B1".to_string())),
            }),
            right: Box::new(AST::CellName("C1".to_string())),
        };

        let result = ASTResolver::resolve(ast, &variables);
        assert_eq!(result, Value::Number(3.0));
    }

}
