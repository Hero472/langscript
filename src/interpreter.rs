use std::collections::HashMap;

use crate::parser::ASTNode;

pub struct Interpreter {
    enviroment: HashMap<String, f64>,
}

impl Interpreter {

    pub fn new() -> Self {
        Interpreter {
            enviroment: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, node: ASTNode) -> f64 {
        match node {
            ASTNode::Assignment(name,value_expr) => {
                let value = self.interpret(*value_expr);
                self.set_variable(name, value);
                value
            }
            ASTNode::Number(value) => value,
            ASTNode::Identifier(name) => {
                *self.enviroment.get(&name).unwrap_or_else(|| {
                    panic!("undefined variable: {}", name)
                })
            }
            ASTNode::BinaryOperation { left, operator, right } => {
                let left_val: f64 = self.interpret(*left);
                let right_val: f64 = self.interpret(*right);
                
                match operator.as_str() {
                    "+" => left_val + right_val,
                    "-" => left_val - right_val,
                    "*" => left_val * right_val,
                    "/" => left_val / right_val,
                    _ => panic!("Unknown operator: {}", operator),
                }
            }
            ASTNode::Grouping(expr) => self.interpret(*expr),
        }
    }

    pub fn set_variable(&mut self, name: String, value: f64) {
        self.enviroment.insert(name, value);
    }

}