use crate::common_types::{ComputeError, Index, Token, Value, AST};

pub trait VarContext {
    fn get_variable(&self, index: Index) -> Option<Result<Value, ComputeError>>;
}

pub struct ASTResolver {}

impl ASTResolver {
    pub fn resolve(ast: &AST, variables: &dyn VarContext) -> Result<Value, ComputeError> {
        match ast {
            AST::Value(value) => Ok(value.clone()),
            AST::CellName(name) => match variables.get_variable(Self::get_cell_idx(name)) {
                Some(value) => Ok(value.unwrap()),
                None => Err(ComputeError::UnfindableReference(format!(
                    "Could not find variable {name} with in context"
                ))),
            },
            AST::BinaryOp { op, left, right } => {
                let left_resolved = Self::resolve(left, variables)?;
                let right_resolved = Self::resolve(right, variables)?;

                match op {
                    Token::Plus => left_resolved
                        .add(right_resolved)
                        .ok_or(ComputeError::TypeError),
                    Token::Minus => left_resolved
                        .sub(right_resolved)
                        .ok_or(ComputeError::TypeError),
                    Token::Division => left_resolved
                        .div(right_resolved)
                        .ok_or(ComputeError::TypeError),
                    Token::Multiply => left_resolved
                        .mult(right_resolved)
                        .ok_or(ComputeError::TypeError),
                    other => panic!("{:?} is not a binary operator", other), // I think this is  unreachable
                }
            }
        }
    }

    pub fn get_cell_idx(cell_name: &str) -> Index {
        let mut x: usize = 0;
        let mut y = 0;

        for (i, c) in cell_name.chars().enumerate() {
            if c.is_ascii_digit() {
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

    struct MockVarContext {
        variables: HashMap<Index, Value>,
    }

    impl VarContext for MockVarContext {
        fn get_variable(&self, index: Index) -> Option<Result<Value, ComputeError>> {
            self.variables.get(&index).cloned().map(Ok)
        }
    }

    impl MockVarContext {
        fn new(variables: HashMap<Index, Value>) -> Self {
            Self { variables }
        }
    }

    #[test]
    fn test_resolve_value_ast() {
        let variables = MockVarContext::new(HashMap::new());
        let ast = AST::Value(Value::Number(42.0));

        let result = ASTResolver::resolve(&ast, &variables).unwrap();
        assert_eq!(result, Value::Number(42.0));
    }

    #[test]
    fn test_resolve_cellname_ast() {
        let mut vars = HashMap::new();
        vars.insert(Index { x: 0, y: 0 }, Value::Number(10.0));

        let variables = MockVarContext::new(vars);
        let ast = AST::CellName("A1".to_string());

        let result = ASTResolver::resolve(&ast, &variables).unwrap();
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

        let result = ASTResolver::resolve(&ast, &variables).unwrap();
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

        let result = ASTResolver::resolve(&ast, &variables).unwrap();
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

        let result = ASTResolver::resolve(&ast, &variables).unwrap();
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

        let result = ASTResolver::resolve(&ast, &variables).unwrap();
        assert_eq!(result, Value::Number(5.0));
    }

    #[test]
    #[should_panic]
    fn test_resolve_missing_cellname() {
        let variables = MockVarContext::new(HashMap::new());
        let ast = AST::CellName("A1".to_string());

        // This should panic because "A1" is not in the context
        ASTResolver::resolve(&ast, &variables).unwrap();
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

        let result = ASTResolver::resolve(&ast, &variables).unwrap();
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

        let result = ASTResolver::resolve(&ast, &variables).unwrap();
        assert_eq!(result, Value::Number(3.0));
    }
}
