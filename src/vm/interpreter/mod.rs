use std::collections::HashMap;

use crate::{core::types::Type, error::RuntimeError, frontend::parser::ast::{Expr, FunctionDecl, Program, Stmt}, vm::{interpreter::{environment::Environment, eval_declaration::evaluate_declaration, eval_statements::evaluate_statement}, value::Value}};

pub mod eval_expressions;
pub mod eval_statements;
pub mod eval_binary_ops;
pub mod eval_unary_ops;
pub mod eval_declaration;
pub mod environment;

#[derive(Debug, Clone)]
pub struct Variable {
    pub value: Value,
    pub typed: Type,
    pub mutable: bool
}

pub struct Interpreter {
    pub environment: Environment
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new()
        }
    }

    pub fn interpret(&mut self, program: Program) -> Result<Value, RuntimeError> {
        
        for decl in program.declarations {
            evaluate_declaration(self, decl)?;
        }

        if let Some(main_func) = self.environment.get_function("main").cloned() {
            println!("{:#?}", main_func);
            self.execute_function(&main_func, vec![])
        } else {
            Ok(Value::Bool(false))
        }
    }

    fn execute_function(&mut self, function: &FunctionDecl, args: Vec<Value>) -> Result<Value, RuntimeError> {
        // Create new environment for function scope

        let mut function_env = Environment::new_child(std::mem::take(&mut self.environment));

        // Bind parameters
        for (param, arg) in function.params.iter().zip(args) {
            function_env.define_variable(param.name.clone(), Variable {
                value: arg,
                typed: param.type_annotation.clone(),
                mutable: false,
            });
        }
        
        // Swap environments
        self.environment = function_env;

        // Execute function body
        let result = evaluate_statement(self, function.body.clone())?;
        
        // Restore parent environment (pop the scope)
        if let Some(parent_env) = self.environment.parent.take() {
            self.environment = *parent_env;
        }
        
        Ok(result)
    }

    pub fn enter_scope(&mut self) {
        // Save the current environment and create a new child scope
        let new_env = Environment::new_child(std::mem::take(&mut self.environment));
        self.environment = new_env;
    }
    
    pub fn exit_scope(&mut self) {
        // Restore the parent environment
        if let Some(parent_env) = self.environment.parent.take() {
            self.environment = *parent_env;
        }
        // If there's no parent, we're at the global scope - do nothing
    }

}