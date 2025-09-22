use crate::{error::RuntimeError, frontend::parser::ast::Stmt, vm::{interpreter::{eval_expressions::evaluate_expr, Interpreter}, value::Value}};

pub fn evaluate_statement(interpreter: &mut Interpreter, stmt: Stmt) -> Result<Value, RuntimeError> {

    match stmt {
        Stmt::Let { name, value, type_annotation, span } => {

            let evaluated_value = evaluate_expr(interpreter, value)?;
            interpreter.variables.insert(name.clone(), evaluated_value.clone());
            Ok(evaluated_value)
        },
        _ => Err(RuntimeError::new("Not yet implemented".to_string()))
    }
}