use crate::{generate_ast::Expr, Token};

#[derive(Clone)]
pub enum Stmt {
    Expression { expression: Expr },
    Print {expression: Expr},
    Let {name: Token, initializer: Expr },
    Block { statements: Vec<Stmt>}
}



impl Stmt {

    pub fn to_string(&self) -> String {
        match self {
            Stmt::Expression { expression } => expression.to_string(),
            Stmt::Print { expression } => format!("print {}", expression.to_string()),
            Stmt::Let { name, initializer } => format!("let {} = {}", name.to_string(), initializer.to_string()),
            Stmt::Block { statements } => format!("[{:?}]", 
                statements.into_iter().map(|stmt| stmt.to_string()).collect::<String>()
            )
        }
    }
}