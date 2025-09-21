use std::collections::HashMap;

use crate::{frontend::ast::{BinaryOp, Expr, UnaryOp}, vm::value::Value};

pub struct Interpreter {
    variables: HashMap<String, Value>
}

impl Interpreter {

    pub fn new() -> Self {
        Self {
            variables: HashMap::new()
        }
    }


    pub fn evaluate(&mut self, exprs: Vec<Expr>) -> Result<Value, String> {

        let mut last_value = Value::Bool(false);
        
        for expr in exprs {
            last_value = self.evaluate_expr(expr)?;
        }

        Ok(last_value)
    }

    fn evaluate_expr(&mut self, expr: Expr) -> Result<Value, String> {

        match expr {
            Expr::IntLiteral(n) => Ok(Value::Int(n)),
            Expr::UintLiteral(n) => Ok(Value::Uint(n)),
            Expr::StringLiteral(s) => Ok(Value::String(s)),
            Expr::BoolLiteral(b) => Ok(Value::Bool(b)),
            Expr::CharLiteral(c) => Ok(Value::Char(c)),
            Expr::FloatLiteral(f) => Ok(Value::Float(f)),
            Expr::Unary { op, expr } => {
                let value = self.evaluate_expr(*expr)?;

                match op {
                    UnaryOp::Negate => self.evaluate_negate(value),
                    UnaryOp::Not => self.evaluate_not(value),
                }
            },
            Expr::Binary { left, op, right } => {
                let left = self.evaluate_expr(*left)?;
                let right = self.evaluate_expr(*right)?;

                match op {
                    BinaryOp::Add => self.evaluate_add(left, right),
                    BinaryOp::Subtract => self.evaluate_subtract(left, right),
                    BinaryOp::Multiply => self.evaluate_multiply(left, right),
                    BinaryOp::Divide => self.evaluate_divide(left, right),
                    BinaryOp::Modulo => self.evaluate_modulo(left, right),
                    BinaryOp::Equals => self.evaluate_equals(left, right),
                    BinaryOp::NotEquals => self.evaluate_not_equals(left, right),
                    BinaryOp::LessThan => self.evaluate_less_than(left, right),
                    BinaryOp::LessEq => self.evaluate_less_eq(left, right),
                    BinaryOp::GreaterThan => self.evaluate_greater_than(left, right),
                    BinaryOp::GreaterEq => self.evaluate_greater_eq(left, right),
                    BinaryOp::And => self.evaluate_and(left, right),
                    BinaryOp::Or => self.evaluate_or(left, right),
                }
            }
            _ => Err(format!("Expression type not implemented: {:?}", expr)),
        }
    }

    fn evaluate_negate(&mut self, value: Value) -> Result<Value, String> {

        match value {
            Value::Int(n) => Ok(Value::Int(-n)),
            Value::Float(f) => Ok(Value::Float(-f)),
            _ => {
                let primitive_type = value.get_primitive_type();

                if let Some(t) = primitive_type {
                    return Err(format!("Cannot apply unary operator '-' to type {}", t))
                } else {
                    return Err("Cannot apply unary operator '-' to non primitives types".to_string())
                }
                    
            }
        }

    }

    fn evaluate_not(&mut self, value: Value) -> Result<Value, String> {

        match value {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            _ => {
                let primitive_type = value.get_primitive_type();

                if let Some(t) = primitive_type {
                    return Err(format!("Cannot apply logical NOT to type {}", t))
                } else {
                    return Err("Cannot apply logical NOT non primitives types".to_string())
                }
                    
            }
        }

    }

    fn evaluate_add(&mut self, left: Value, right: Value) -> Result<Value, String> {
        todo!()
    }

    fn evaluate_subtract(&mut self, left: Value, right: Value) -> Result<Value, String> {
        todo!()
    }

    fn evaluate_multiply(&mut self, left: Value, right: Value) -> Result<Value, String> {
        todo!()
    }

    fn evaluate_divide(&mut self, left: Value, right: Value) -> Result<Value, String> {
        todo!()
    }

    fn evaluate_modulo(&mut self, left: Value, right: Value) -> Result<Value, String> {
        todo!()
    }

    fn evaluate_equals(&mut self, left: Value, right: Value) -> Result<Value, String> {
        todo!()
    }

    fn evaluate_not_equals(&mut self, left: Value, right: Value) -> Result<Value, String> {
        todo!()
    }

    fn evaluate_less_than(&mut self, left: Value, right: Value) -> Result<Value, String> {
        todo!()
    }

    fn evaluate_less_eq(&mut self, left: Value, right: Value) -> Result<Value, String> {
        todo!()
    }

    fn evaluate_greater_than(&mut self, left: Value, right: Value) -> Result<Value, String> {
        todo!()
    }

    fn evaluate_greater_eq(&mut self, left: Value, right: Value) -> Result<Value, String> {
        todo!()
    }

    fn evaluate_and(&mut self, left: Value, right: Value) -> Result<Value, String> {
        todo!()
    }

    fn evaluate_or(&mut self, left: Value, right: Value) -> Result<Value, String> {
        todo!()
    }
}