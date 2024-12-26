use super::ExprToken;


enum Value {
    Text(String),
    Number(f64),
}
struct Context;

impl Context {
    fn resolve(&self, cell_name : &str) -> Value{
        todo!()
    }
}

trait Node {
    // Cells require to see other cells to evaluate expression such as A1
    fn evaluate(&self, context: &Context) -> Value;
}

struct CellNode {
    cell_name: String,
}

impl Node for CellNode {
    fn evaluate(&self, context: &Context) -> Value {
        context.resolve(&self.cell_name)
    }
}

enum BinaryOperator {
    Plus,
    Minus,
    Div,
    Mult,
}

struct BinaryOperatorNode {
    operation: BinaryOperator,
    left: Box<dyn Node>,
    right: Box<dyn Node>,
}

impl Node for BinaryOperatorNode {
    fn evaluate(&self, context: &Context) -> Value {
        let left_val = self.left.evaluate(context);
        let right_val = self.right.evaluate(context);
        
        let (left_val, right_val) = match (left_val, right_val) {
            (Value::Number(l), Value::Number(r)) => (l, r),
            _ => panic!("Expected left and right to be numbers"),
        };
        match self.operation {
            BinaryOperator::Plus =>Value::Number( left_val + right_val),
            BinaryOperator::Minus =>Value::Number( left_val - right_val),
            BinaryOperator::Div =>Value::Number( left_val / right_val),
            BinaryOperator::Mult =>Value::Number( left_val * right_val),
        }
    }
}

pub struct AST {
    head : Box<dyn Node>
}

impl AST {
    pub fn new(tokens : Vec<ExprToken>) -> Self {
        todo!()
    }
}