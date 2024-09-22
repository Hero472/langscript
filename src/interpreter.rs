use crate::{environment::Enviroment, generate_ast::{Expr, LiteralValueAst}, stmt::Stmt};

pub struct Interpreter {
    enviroment: Enviroment,
}

impl Interpreter {

    pub fn new() -> Self {
        Self {
            enviroment: Enviroment::new()
        }
    }

    pub fn interpreter(&mut self, stmts: Vec<Stmt>) -> Result<(), String> {

        for stmt in stmts {
            match stmt {
                Stmt::Expression { expression } => expression.evaluate(&self.enviroment)?,
                Stmt::Print { expression } => {
                    let value: LiteralValueAst = expression.evaluate(&self.enviroment)?;
                    print!("{:?}", value);
                    return Ok(())
                },
                Stmt::Let { name, initializer } => {
                    let value: LiteralValueAst = initializer.evaluate(&self.enviroment)?;
                    
                   self.enviroment.define(name.lexeme, value);
                   return Ok(())
                }
            };
        }
        Ok(())
    }

}