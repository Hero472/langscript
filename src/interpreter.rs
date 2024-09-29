use std::{cell::RefCell, rc::Rc};

use crate::{environment::Environment, generate_ast::LiteralValueAst, stmt::Stmt};

#[derive(Debug, Clone)]
pub enum ControlFlow {
    Break,
    None,
}

pub struct Interpreter {
    environment: Rc<RefCell<Environment>>,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Rc::new(RefCell::new(Environment::new())),
        }
    }
    

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<ControlFlow, String> {
        for stmt in stmts {
            let result = match stmt {
                Stmt::Expression { expression } => {
                    expression.evaluate(&mut self.environment.clone())?;
                    Ok(ControlFlow::None)
                }
                Stmt::Print { expression } => {
                    let value: LiteralValueAst = expression.evaluate(&mut self.environment.clone())?;
                    println!("{}", value.to_string());
                    Ok(ControlFlow::None)
                }
                Stmt::Let { name, initializer } => {
                    let value: LiteralValueAst = initializer.evaluate(&mut self.environment.clone())?;
                    self.environment.borrow_mut().define(name.lexeme.clone(), value);
                    Ok(ControlFlow::None)
                }
                Stmt::Block { statements } => {
                    let mut new_environment: Environment = Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());
                    let old_environment: Rc<RefCell<Environment>> = self.environment.clone();
                    self.environment = Rc::new(RefCell::new(new_environment));
        
                    let block_result: Result<ControlFlow, String> =
                        self.interpret(statements.into_iter().map(|b| *b).collect::<Vec<Stmt>>());
        
                    self.environment = old_environment;
        
                    block_result
                }
                Stmt::IfStmt { predicate, then, els } => {
                    let truth_value: LiteralValueAst = predicate.evaluate(&mut self.environment.clone())?;
                    if truth_value.is_truthy() == LiteralValueAst::True {
                        self.interpret(vec![*then])
                    } else if let Some(els_stmt) = els {
                        self.interpret(vec![*els_stmt])
                    } else {
                        Ok(ControlFlow::None)
                    }
                }
                Stmt::WhileStmt { condition, body } => {
                    while condition.evaluate(&mut self.environment.clone())?.is_truthy() == LiteralValueAst::True {
                        let result: ControlFlow = self.interpret(vec![*body.clone()])?;
                        if let ControlFlow::Break = result {
                            break;
                        }
                    }
                    Ok(ControlFlow::None)
                }
                Stmt::BreakStmt => {
                    Ok(ControlFlow::Break)
                }
            };
    
            // If we encounter ControlFlow::Break, propagate it immediately
            if let Ok(ControlFlow::Break) = result {
                return Ok(ControlFlow::Break);
            } else if result.is_err() {
                return result;
            }
        }
    
        Ok(ControlFlow::None)
    }
}

