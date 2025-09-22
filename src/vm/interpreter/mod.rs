use std::collections::HashMap;

use crate::{error::RuntimeError, frontend::parser::ast::{Expr, Stmt}, vm::{interpreter::eval_statements::evaluate_statement, value::Value}};

pub mod eval_expressions;
pub mod eval_statements;
pub mod eval_binary_ops;
pub mod eval_unary_ops;

pub struct Interpreter {
    pub variables: HashMap<String, Value>
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            variables: HashMap::new()
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<Value, RuntimeError> {

        let mut last_value = Value::Bool(false);
        
        for stmt in stmts {
            last_value = evaluate_statement(self, stmt)?;
        }

        Ok(last_value)
    }

    // pub fn execute_statement(&mut self, stmt: Stmt) -> Result<Value, RuntimeError> {
    //     eval_statements::execute_statement(self, stmt)
    // }

    pub fn evaluate_expr(&mut self, expr: Expr) -> Result<Value, RuntimeError> {
        eval_expressions::evaluate_expr(self, expr)
    }

}

pub fn interpret(statements: Vec<Stmt>) -> Result<Value, RuntimeError> {
    let mut interpreter = Interpreter::new();
    interpreter.interpret(statements)
}