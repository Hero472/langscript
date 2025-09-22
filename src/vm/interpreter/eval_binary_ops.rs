use crate::{error::RuntimeError, frontend::{lexer::span::Span, parser::ast::BinaryOp}, vm::{interpreter::Interpreter, runtime_error::wrap_error, value::Value}};

pub fn evaluate_binary_op(
    interpreter: &mut Interpreter,
    left: Value,
    op: BinaryOp,
    right: Value,
    span: Span
) -> Result<Value, RuntimeError> {
    let result = match op {
        BinaryOp::Add => evaluate_add(interpreter, left, right),
        BinaryOp::Subtract => evaluate_subtract(interpreter, left, right),
        BinaryOp::Multiply => evaluate_multiply(interpreter, left, right),
        BinaryOp::Divide => evaluate_divide(interpreter, left, right),
        BinaryOp::Modulo => evaluate_modulo(interpreter, left, right),
        BinaryOp::Equals => evaluate_equals(interpreter, left, right),
        BinaryOp::NotEquals => evaluate_not_equals(interpreter, left, right),
        BinaryOp::LessThan => evaluate_less_than(interpreter, left, right),
        BinaryOp::LessEq => evaluate_less_eq(interpreter, left, right),
        BinaryOp::GreaterThan => evaluate_greater_than(interpreter, left, right),
        BinaryOp::GreaterEq => evaluate_greater_eq(interpreter, left, right),
        BinaryOp::And => evaluate_and(interpreter, left, right),
        BinaryOp::Or => evaluate_or(interpreter, left, right),
    };
    
    result.map_err(|e| wrap_error(e, span))
}

fn evaluate_add(interpreter: &mut Interpreter, left: Value, right: Value) -> Result<Value, String> {
        
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

fn evaluate_subtract(interpreter: &mut Interpreter, left: Value, right: Value) -> Result<Value, String> {
    
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

fn evaluate_multiply(interpreter: &mut Interpreter, left: Value, right: Value) -> Result<Value, String> {
    
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

fn evaluate_divide(interpreter: &mut Interpreter, left: Value, right: Value) -> Result<Value, String> {
    
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

fn evaluate_modulo(interpreter: &mut Interpreter, left: Value, right: Value) -> Result<Value, String> {
    
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

fn evaluate_equals(interpreter: &mut Interpreter, left: Value, right: Value) -> Result<Value, String> {
    
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

fn evaluate_not_equals(interpreter: &mut Interpreter, left: Value, right: Value) -> Result<Value, String> {
    
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

fn evaluate_less_than(interpreter: &mut Interpreter, left: Value, right: Value) -> Result<Value, String> {
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

fn evaluate_less_eq(interpreter: &mut Interpreter, left: Value, right: Value) -> Result<Value, String> {
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

fn evaluate_greater_than(interpreter: &mut Interpreter, left: Value, right: Value) -> Result<Value, String> {
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

fn evaluate_greater_eq(interpreter: &mut Interpreter, left: Value, right: Value) -> Result<Value, String> {
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

fn evaluate_and(interpreter: &mut Interpreter, left: Value, right: Value) -> Result<Value, String> {
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

fn evaluate_or(interpreter: &mut Interpreter, left: Value, right: Value) -> Result<Value, String> {
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