use crate::{error::RuntimeError, frontend::{lexer::span::Span, parser::ast::UnaryOp}, vm::{interpreter::Interpreter, runtime_error::wrap_error, value::Value}};

pub fn evaluate_unary_op(
    interpreter: &mut Interpreter,
    op: UnaryOp,
    value: Value,
    span: Span
) -> Result<Value, RuntimeError> {
    let result = match op {
        UnaryOp::Negate => evaluate_negate(interpreter, value),
        UnaryOp::Not => evaluate_not(interpreter, value),
    };
    
    result.map_err(|e| wrap_error(e, span))
}

fn evaluate_negate(interpreter: &mut Interpreter, value: Value) -> Result<Value, String> {

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

fn evaluate_not(interpreter: &mut Interpreter, value: Value) -> Result<Value, String> {

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