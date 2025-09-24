use crate::{core::types::{PrimitiveType, Type}, error::RuntimeError, frontend::parser::ast::Stmt, vm::{interpreter::{eval_expressions::evaluate_expr, Interpreter, Variable}, value::Value}};

pub fn evaluate_statement(interpreter: &mut Interpreter, stmt: Stmt) -> Result<Value, RuntimeError> {

    match stmt {
        Stmt::Let { name, value, type_annotation, span, mutable } => {

            let evaluated_value = evaluate_expr(interpreter, value)?;
            
            let stored_type = type_annotation.unwrap_or_else(|| {

                evaluated_value.get_type().unwrap_or_else(|| {
                    // Default to a generic type if we really can't infer
                    Type::Primitive(PrimitiveType::Bool) // or whatever makes sense
                })
            });

            let variable = Variable { value: evaluated_value.clone(), typed: stored_type, mutable };
            
            interpreter.environment.define_variable(name, variable);

            Ok(evaluated_value)
        },
        Stmt::Block(statements, span) => {
            interpreter.enter_scope();

            let mut last_value = Value::Bool(false);

            for stmt in statements {
                last_value = evaluate_statement(interpreter, stmt)?;
            }
            
            interpreter.exit_scope();

            Ok(last_value)
        },
        Stmt::Expr(expr, span) => evaluate_expr(interpreter, expr),
        _ => Err(RuntimeError::new("Statement no yet implemented".to_string()))
    }
}