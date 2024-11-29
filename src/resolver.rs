use std::collections::HashMap;

use crate::{generate_ast::Expr, interpreter::Interpreter, stmt::Stmt, Token};

pub struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>
}

impl Resolver {
    
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
            scopes: vec![]
        }
    }

    #[allow(dead_code)]
    pub fn resolve(&mut self, stmt: &Stmt) -> Result<(), String> {
        
        match stmt {
            Stmt::Block { statements: _ } => self.resolve_block(stmt)?,
            Stmt::Let { name: _, initializer: _ } => self.resolve_let(stmt)?,
            Stmt::Function { name: _, params: _, body: _ } => self.resolve_function(stmt)?,
            Stmt::Expression { expression } => self.resolve_expr(expression)?,
            Stmt::IfStmt { predicate: _, then: _, els: _ } => self.resolve_if_stmt(&stmt)?,
            Stmt::Print { expression } => self.resolve_expr(expression)?,
            Stmt::ReturnStmt { keyword: _, value: None } => (),
            Stmt::ReturnStmt { keyword: _, value: Some(value) } => self.resolve_expr(value)?,
            Stmt::WhileStmt { condition, body } => {
                self.resolve_expr(condition)?;
                self.resolve(body)?;
            }
            _ => todo!()
        }
        todo!()
    }

    #[allow(dead_code)]
    fn resolve_block(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Block { statements } => {
                self.begin_scope();
                self.resolve_many(statements);
                self.end_scope();
            }
            _ => panic!("Wrong type")
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn resolve_if_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        if let Stmt::IfStmt { predicate, then, els } = stmt {
            self.resolve_expr(predicate)?;
            self.resolve(then.as_ref())?;
            if let Some(els) = els {
                self.resolve(els.as_ref())?;
            }
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn resolve_function(&mut self, stmt: &Stmt) -> Result<(), String> {
        if let Stmt::Function { name, params , body  } = stmt {
            self.declare(name);
            self.define(name);
            
            self.resolve_function_helper(params, body)?;
        } else {
            panic!("Wrong type in resolve function")
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn resolve_function_helper(&mut self, params: &Vec<Token>, body: &Vec<Box<Stmt>>) -> Result<(), String> {

        self.begin_scope();

        for param in params {
            self.declare(param);
            self.define(param);
        }

        self.resolve_many(body);
        self.end_scope();

        Ok(())
    }

    #[allow(dead_code)]
    fn resolve_let(&mut self, stmt: &Stmt) -> Result<(), String>  {
        if let Stmt::Let { name, initializer } = stmt {
            self.declare(name);
            self.resolve_expr(&initializer.clone())?;
            self.define(name);
            todo!()
        } else {
            panic!("Wrong type in resolve let")
        }
    }

    #[allow(dead_code)]
    fn resolve_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Variable { name: _ } => self.resolve_expr_let(expr),
            Expr::Assign { name: _, value: _ } => self.resolve_expr_assign(expr),
            Expr::Binary { left, operator: _, right } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)
            },
            Expr::Call { callee, paren: _, arguments } => {
                self.resolve_expr(callee.as_ref())?;
                for arg in arguments {
                    self.resolve_expr(arg)?;
                }
                Ok(())
            },
            Expr::Literal { value: _ } => Ok(()),
            Expr::Logical { left, operator: _, right } => {
                self.resolve_expr(left)?;
                self.resolve_expr(right)
            },
            Expr::Unary { operator: _, value } => {
                self.resolve_expr(value)
            },
            Expr::Ternary { condition, expr_true, expr_false } => {
                self.resolve_expr(&condition)?;
                self.resolve_expr(&expr_true)?;
                self.resolve_expr(&expr_false)?;
                Ok(())
            },
            Expr::Grouping { expression } => self.resolve_expr(&expression),
            Expr::AnonFunction { paren: _, arguments, body } => self.resolve_function_helper(arguments, body)
        }
    }

    #[allow(dead_code)]
    fn resolve_expr_let(&mut self, expr: &Expr) -> Result<(), String> {
        if let Expr::Variable { name } = expr {
            if !self.scopes.is_empty() && *self.scopes[self.scopes.len() - 1].get(&name.lexeme.clone()).unwrap() == false {
                return Err("Can't read local variable in its own initializer".to_string())
            }

            self.resolve_local(expr)
        } else {
            panic!("Wrong type in resolve_expr_let")
        }
    }

    #[allow(dead_code)]
    fn resolve_expr_assign(&mut self, expr: &Expr) -> Result<(), String> {
        if let Expr::Assign { name: _, value } = expr {
            self.resolve_expr(value.as_ref())?;
            self.resolve_local(expr)?;
        } else {
            panic!("Wrong type in resolve assign")
        }
        Ok(())
    }

    #[allow(dead_code)]
    fn resolve_local(&mut self, expr: &Expr) -> Result<(), String> {
        if let Expr::Variable { name }  = expr {
            let size: usize = self.scopes.len();
            for i in (size - 1)..0 {
                if self.scopes[i].contains_key(&name.lexeme) {
                    self.interpreter.resolve(expr, size - 1 - i)?;
                }
            }
            Ok(())
        } else {
            panic!("Wrong type in resolve local")
        }
    }   

    #[allow(dead_code)]
    fn declare(&mut self, name: &Token) {
        if self.scopes.is_empty() {return;}
        if let Some(last_scope) = self.scopes.last_mut() {
            last_scope.insert(name.lexeme.clone(), false);
        }
    }

    #[allow(dead_code)]
    fn define(&mut self, name: &Token) {
        if self.scopes.is_empty() {return;}
        if let Some(last_scope) = self.scopes.last_mut() {
            last_scope.insert(name.lexeme.clone(), true);
        }
    }

    #[allow(dead_code)]
    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    #[allow(dead_code)]
    fn resolve_many(&mut self, stmts: &Vec<Box<Stmt>>) {
        for stmt in stmts {
            let _ = self.resolve(stmt);
        }
    }

    #[allow(dead_code)]
    fn end_scope(&mut self) {
        self.scopes.pop().expect("Stack underflow");
    }

}