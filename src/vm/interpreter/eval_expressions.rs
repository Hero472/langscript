use crate::{error::RuntimeError, frontend::parser::ast::Expr, vm::{interpreter::{eval_binary_ops::evaluate_binary_op, eval_unary_ops::evaluate_unary_op, Interpreter}, value::Value}};

pub fn evaluate_expr(interpreter: &mut Interpreter, expr: Expr) -> Result<Value, RuntimeError> {
    match expr {
        Expr::IntLiteral(n, _) => Ok(Value::Int(n)),
        Expr::UintLiteral(n, _) => Ok(Value::Uint(n)),
        Expr::StringLiteral(s, _) => Ok(Value::String(s)),
        Expr::BoolLiteral(b, _) => Ok(Value::Bool(b)),
        Expr::CharLiteral(c, _) => Ok(Value::Char(c)),
        Expr::FloatLiteral(f, _) => Ok(Value::Float(f)),
        Expr::Unary { op, expr, span } => {
            let value = evaluate_expr(interpreter, *expr)?;
            evaluate_unary_op(interpreter, op, value, span)
        },
        Expr::Binary { left, op, right, span } => {
            let left_val = evaluate_expr(interpreter, *left)?;
            let right_val = evaluate_expr(interpreter, *right)?;
            evaluate_binary_op(interpreter, left_val, op, right_val, span)
        },
        Expr::Grouped(expr, _) => {
            evaluate_expr(interpreter, *expr)
        },
        _ => Err(RuntimeError::new(format!("Expression type not implemented: {:?}", expr))),
    }
}