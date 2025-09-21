use std::collections::HashMap;

use crate::{frontend::{lexer::span::Span, parser::ast::{BinaryOp, Expr, UnaryOp}}, vm::{runtime_error::RuntimeError, value::Value}};

pub struct Interpreter {
    variables: HashMap<String, Value>
}

impl Interpreter {

    pub fn new() -> Self {
        Self {
            variables: HashMap::new()
        }
    }

    pub fn evaluate(&mut self, exprs: Vec<Expr>) -> Result<Value, RuntimeError> {

        let mut last_value = Value::Bool(false);
        
        for expr in exprs {
            last_value = self.evaluate_expr(expr)?;

        }

        Ok(last_value)
    }

    fn evaluate_expr(&mut self, expr: Expr) -> Result<Value, RuntimeError> {

        match expr {
            Expr::IntLiteral(n, _) => Ok(Value::Int(n)),
            Expr::UintLiteral(n, _) => Ok(Value::Uint(n)),
            Expr::StringLiteral(s, _) => Ok(Value::String(s)),
            Expr::BoolLiteral(b, _) => Ok(Value::Bool(b)),
            Expr::CharLiteral(c, _) => Ok(Value::Char(c)),
            Expr::FloatLiteral(f, _) => Ok(Value::Float(f)),
            Expr::Unary { op, expr, span } => {
                let value = self.evaluate_expr(*expr)?;
                self.evaluate_unary_op(op, value, span)
            },
            Expr::Binary { left, op, right, span } => {
                let left_val = self.evaluate_expr(*left)?;
                let right_val = self.evaluate_expr(*right)?;
                self.evaluate_binary_op(left_val, op, right_val, span)
            },
            Expr::Grouped(expr, _) => {
                self.evaluate_expr(*expr)
            },
            _ => Err(RuntimeError::new(format!("Expression type not implemented: {:?}", expr))),
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
        
        match (left, right) {
            (Value::Int(n1), Value::Int(n2)) => {
                n1.checked_add(n2)
                    .map(Value::Int)
                    .ok_or_else(|| "Integer overflow in addition".to_string())
            },
            (Value::Uint(n1), Value::Uint(n2)) => {
                n1.checked_add(n2)
                    .map(Value::Uint)
                    .ok_or_else(|| "Unsigned integer overflow in addition".to_string())
            },
            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Float(f1 + f2)),
            (Value::String(s1), Value::String(s2)) => Ok(Value::String(format!("{}{}", s1, s2))),
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot add {} and {}", left_desc, right_desc))
            }
        }

    }

    fn evaluate_subtract(&mut self, left: Value, right: Value) -> Result<Value, String> {
        
        match (left, right) {
            (Value::Int(n1), Value::Int(n2)) => {
                n1.checked_sub(n2)
                    .map(Value::Int)
                    .ok_or_else(|| "Integer overflow in subtraction".to_string())
            },
            (Value::Uint(n1), Value::Uint(n2)) => {
                n1.checked_sub(n2)
                    .map(Value::Uint)
                    .ok_or_else(|| "Unsigned integer underflow in subtraction".to_string())
            },
            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Float(f1 - f2)),
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot subtract {} from {}", right_desc, left_desc))
            }
        }

    }

    fn evaluate_multiply(&mut self, left: Value, right: Value) -> Result<Value, String> {
        
        match (left, right) {
            (Value::Int(n1), Value::Int(n2)) => {
                n1.checked_mul(n2)
                    .map(Value::Int)
                    .ok_or_else(|| "Integer overflow in multiplication".to_string())
            },
            (Value::Uint(n1), Value::Uint(n2)) => {
                n1.checked_mul(n2)
                    .map(Value::Uint)
                    .ok_or_else(|| "Unsigned integer overflow in multiplication".to_string())
            },
            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Float(f1 * f2)),
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot multiply {} and {}", left_desc, right_desc))
            }
        }

    }

    fn evaluate_divide(&mut self, left: Value, right: Value) -> Result<Value, String> {
        
        match (left, right) {
            (Value::Int(n1), Value::Int(n2)) => {
                if n2 == 0 {
                    Err("Division by zero".to_string())
                } else {
                    // Check for edge case: i64::MIN / -1 which overflows
                    if n1 == i64::MIN && n2 == -1 {
                        Err("Integer overflow in division".to_string())
                    } else {
                        Ok(Value::Int(n1 / n2))
                    }
                }
            },
            (Value::Uint(n1), Value::Uint(n2)) => {
                if n2 == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Uint(n1 / n2))
                }
            },
            (Value::Float(f1), Value::Float(f2)) => {
                if f2 == 0.0 {
                    // Float division by zero produces infinity, but you might want to error
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(f1 / f2))
                }
            },
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot divide {} by {}", left_desc, right_desc))
            }
        }
        
    }

    fn evaluate_modulo(&mut self, left: Value, right: Value) -> Result<Value, String> {
        
        match (left, right) {
            (Value::Int(n1), Value::Int(n2)) => {
                if n2 == 0 {
                    Err("Modulo by zero".to_string())
                } else {
                    Ok(Value::Int(n1 % n2))
                }
            },
            (Value::Uint(n1), Value::Uint(n2)) => {
                if n2 == 0 {
                    Err("Modulo by zero".to_string())
                } else {
                    Ok(Value::Uint(n1 % n2))
                }
            },
            (Value::Float(f1), Value::Float(f2)) => {
                if f2 == 0.0 {
                    Err("Modulo by zero".to_string())
                } else {
                    Ok(Value::Float(f1 % f2))
                }
            },
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot compute modulo of {} and {}", left_desc, right_desc))
            }
        }

    }

    fn evaluate_equals(&mut self, left: Value, right: Value) -> Result<Value, String> {
        
        match (left, right) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a == b)),
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a == b)),
            (Value::Uint(a), Value::Uint(b)) => Ok(Value::Bool(a == b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a == b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a == b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a == b)),
            (Value::Int(a), Value::Uint(b)) => Ok(Value::Bool(a == b as i64)),
            (Value::Uint(a), Value::Int(b)) => Ok(Value::Bool(a == b as u64)),
            (Value::Array(a), Value::Array(b)) => Ok(Value::Bool(a == b)),
            (Value::Tuple(a), Value::Tuple(b)) => Ok(Value::Bool(a == b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 == b as f64)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a as f64 == b as f64)),
            (Value::Uint(a), Value::Float(b)) => Ok(Value::Bool(a as f64 == b as f64)),
            (Value::Float(a), Value::Uint(b)) => Ok(Value::Bool(a as f64 == b as f64)),
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot compare of {} and {} for equality", left_desc, right_desc))
            }
        }

    }

    fn evaluate_not_equals(&mut self, left: Value, right: Value) -> Result<Value, String> {
        
        match (left, right) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a != b)),
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a != b)),
            (Value::Uint(a), Value::Uint(b)) => Ok(Value::Bool(a != b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a != b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a != b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a != b)),
            (Value::Int(a), Value::Uint(b)) => Ok(Value::Bool(a != b as i64)),
            (Value::Uint(a), Value::Int(b)) => Ok(Value::Bool(a != b as u64)),
            (Value::Array(a), Value::Array(b)) => Ok(Value::Bool(a != b)),
            (Value::Tuple(a), Value::Tuple(b)) => Ok(Value::Bool(a != b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 != b as f64)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a as f64 != b as f64)),
            (Value::Uint(a), Value::Float(b)) => Ok(Value::Bool(a as f64 != b as f64)),
            (Value::Float(a), Value::Uint(b)) => Ok(Value::Bool(a as f64 != b as f64)),
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot compare of {} and {} for equality", left_desc, right_desc))
            }
        }

    }

    fn evaluate_less_than(&mut self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            (Value::Uint(a), Value::Uint(b)) => Ok(Value::Bool(a < b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a < b)),
            
            // Cross-type comparisons
            (Value::Int(a), Value::Uint(b)) => Ok(Value::Bool(a < b as i64)),
            (Value::Uint(a), Value::Int(b)) => Ok(Value::Bool((a as i64) < b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) < b as f64)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool((a as f64) < b as f64)),
            (Value::Uint(a), Value::Float(b)) => Ok(Value::Bool((a as f64) < b as f64)),
            (Value::Float(a), Value::Uint(b)) => Ok(Value::Bool((a as f64) < b as f64)),
            
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot compare {} and {} with less than", left_desc, right_desc))
            }
        }
    }

    fn evaluate_less_eq(&mut self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            (Value::Uint(a), Value::Uint(b)) => Ok(Value::Bool(a <= b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a <= b)),
            
            // Cross-type comparisons
            (Value::Int(a), Value::Uint(b)) => Ok(Value::Bool(a <= b as i64)),
            (Value::Uint(a), Value::Int(b)) => Ok(Value::Bool((a as i64) <= b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) <= b as f64)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a as f64 <= b as f64)),
            (Value::Uint(a), Value::Float(b)) => Ok(Value::Bool(a as f64 <= b as f64)),
            (Value::Float(a), Value::Uint(b)) => Ok(Value::Bool(a as f64 <= b as f64)),
            
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot compare {} and {} with less than or equal", left_desc, right_desc))
            }
        }
    }

    fn evaluate_greater_than(&mut self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            (Value::Uint(a), Value::Uint(b)) => Ok(Value::Bool(a > b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a > b)),
            
            // Cross-type comparisons
            (Value::Int(a), Value::Uint(b)) => Ok(Value::Bool(a > b as i64)),
            (Value::Uint(a), Value::Int(b)) => Ok(Value::Bool((a as i64) > b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) > b as f64)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a as f64 > b as f64)),
            (Value::Uint(a), Value::Float(b)) => Ok(Value::Bool(a as f64 > b as f64)),
            (Value::Float(a), Value::Uint(b)) => Ok(Value::Bool(a as f64 > b as f64)),
            
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot compare {} and {} with greater than", left_desc, right_desc))
            }
        }
    }

    fn evaluate_greater_eq(&mut self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            (Value::Uint(a), Value::Uint(b)) => Ok(Value::Bool(a >= b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a >= b)),
            
            // Cross-type comparisons
            (Value::Int(a), Value::Uint(b)) => Ok(Value::Bool(a >= b as i64)),
            (Value::Uint(a), Value::Int(b)) => Ok(Value::Bool((a as i64) >= b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) >= b as f64)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a as f64 >= b as f64)),
            (Value::Uint(a), Value::Float(b)) => Ok(Value::Bool(a as f64 >= b as f64)),
            (Value::Float(a), Value::Uint(b)) => Ok(Value::Bool(a as f64 >= b as f64)),
            
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot compare {} and {} with greater than or equal", left_desc, right_desc))
            }
        }
    }

    fn evaluate_and(&mut self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Logical AND requires boolean operands, got {} and {}", left_desc, right_desc))
            }
        }
    }

    fn evaluate_or(&mut self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Logical OR requires boolean operands, got {} and {}", left_desc, right_desc))
            }
        }
    }

    fn evaluate_binary_op(
        &mut self,
        left: Value,
        op: BinaryOp,
        right: Value,
        span: Span
    ) -> Result<Value, RuntimeError> {
        let result = match op {
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
        };
        
        result.map_err(|e| self.wrap_error(e, span))
    }

    // Helper for unary operations
    fn evaluate_unary_op(
        &mut self,
        op: UnaryOp,
        value: Value,
        span: Span
    ) -> Result<Value, RuntimeError> {
        let result = match op {
            UnaryOp::Negate => self.evaluate_negate(value),
            UnaryOp::Not => self.evaluate_not(value),
        };
        
        result.map_err(|e| self.wrap_error(e, span))
    }

    fn wrap_error<E: Into<String>>(&self, error: E, span: Span) -> RuntimeError {
        RuntimeError::new(error.into()).with_span(span)
    }
}