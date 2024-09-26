use std::{ops::Deref, rc::Rc};

use crate::{environment::Environment, generate_ast::{LiteralValueAst}, stmt::Stmt};

pub struct Interpreter {
    environment: Rc<Environment>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(Environment::new()),
        }
    }
    

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get mutable reference to environment"),
                    )?;
                }
                Stmt::Print { expression } => {
                    let value: LiteralValueAst = expression.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get mutable to environment"),
                    )?;
                    println!("{:?}",value.to_string());
                }
                Stmt::Let { name, initializer } => {
                    let value: LiteralValueAst = initializer.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get mutable to environment"),
                    )?;

                    Rc::get_mut(&mut self.environment)
                        .expect("Could not get mutable to environment")
                        .define(name.lexeme, value);
                }
                Stmt::Block { statements } => {
                    let mut new_environment: Environment = Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());
                    let old_environment: Rc<Environment> = self.environment.clone();
                    self.environment = Rc::new(new_environment);
                    let block_result: Result<(), String> = self.interpret(statements);
                    self.environment = old_environment;

                    block_result?;
                },
                Stmt::IfStmt { predicate, then, els } => {
                    let truth_value = predicate.evaluate(
                        Rc::get_mut(&mut self.environment)
                            .expect("Could not get mutable ref to env"),
                    )?;
                    if truth_value.is_truthy() == LiteralValueAst::True {
                        self.interpret(vec![*then])?;
                    } else if let Some(els_stmt) = els {
                        self.interpret(vec![*els_stmt])?;
                    }

                }
            };
        }

        Ok(())
    }
}

