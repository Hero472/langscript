use std::{cell::RefCell, rc::Rc};

use crate::{environment::Environment, generate_ast::LiteralValueAst, stmt::Stmt};

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }
    

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), String> {
        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate(&mut self.environment.clone())?;
                }
                Stmt::Print { expression } => {
                    let value: LiteralValueAst = expression.evaluate(&mut self.environment.clone())?;
                    println!("{}", value.to_string());
                }
                Stmt::Let { name, initializer } => {
                    let value: LiteralValueAst = initializer.evaluate(&mut self.environment.clone())?;

                    self.environment.borrow_mut().define(name.lexeme.clone(), value);
                }
                Stmt::Block { statements } => {
                    let mut new_environment: Environment = Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());
                    let old_environment: Rc<RefCell<Environment>> = self.environment.clone();
                    self.environment = Rc::new(RefCell::new(new_environment));
                    let block_result: Result<(), String> = self.interpret(
                        statements.into_iter().map(|b| *b).collect::<Vec<Stmt>>()
                    );

                    self.environment = old_environment;

                    block_result?;
                },
                Stmt::IfStmt { predicate, then, els } => {
                    let truth_value: LiteralValueAst = predicate.evaluate(&mut self.environment.clone())?;
                    if truth_value.is_truthy() == LiteralValueAst::True {
                        self.interpret(vec![*then])?;
                    } else if let Some(els_stmt) = els {
                        self.interpret(vec![*els_stmt])?;
                    }

                },
                Stmt::WhileStmt { condition, body } => {
                    let mut flag: LiteralValueAst = condition.evaluate(&mut self.environment.clone())?;

                    while flag.is_truthy() == LiteralValueAst::True {
                        self.interpret(vec![*body.clone()])?;
                        flag = condition.evaluate(&mut self.environment.clone())?;
                    }
                }
            };
        }

        Ok(())
    }
}

