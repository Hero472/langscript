use std::{ops::Deref, rc::Rc};

use crate::{environment::Enviroment, generate_ast::{LiteralValueAst}, stmt::Stmt};

pub struct Interpreter {
    enviroment: Rc<Enviroment>,
}

impl Interpreter {

    pub fn new() -> Self {
        Self {
            enviroment: Rc::new(Enviroment::new())
        }
    }

    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), String> {

        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => 
                    expression.evaluate(Rc::get_mut(&mut self.enviroment).expect("Could not get mutable reference to environment"))?,
                Stmt::Print { expression } => {
                    let value: LiteralValueAst = expression.evaluate(Rc::get_mut(&mut self.enviroment).expect("Could not get mutable reference to environment"))?;
                    print!("{:?}", value.to_string());
                    return Ok(())
                },
                Stmt::Let { name, initializer } => {
                    let value: LiteralValueAst = initializer.evaluate(Rc::get_mut(&mut self.enviroment).expect("Could not get mutable reference to environment"))?;
                    
                  Rc::get_mut(&mut self.enviroment).expect("Could not get mutable ref to env").define(name.lexeme, value);
                  return Ok(())
                },
                Stmt::Block { statements } => {

                    let mut new_environment: Enviroment = Enviroment::new();
                    new_environment.enclosing = Some(self.enviroment.clone());

                    let old_environment: Rc<Enviroment> = self.enviroment.clone();
                    self.enviroment = Rc::new(new_environment);


                    let block_result: Result<(), String> = self.interpret(statements);

                    self.enviroment = old_environment;

                    block_result?;
                    return Ok(());
                }
            };
        }
        Ok(())
    }

}