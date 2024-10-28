use crate::{generate_ast::Expr, Token};

#[derive(Debug, Clone)]
pub enum Stmt {
    Expression { expression: Expr },
    Print {expression: Expr},
    Let {name: Token, initializer: Expr },
    Block { statements: Vec<Box<Stmt>>},
    IfStmt { predicate: Expr, then: Box<Stmt>, els: Option<Box<Stmt>>},
    WhileStmt {condition: Expr, body: Box<Stmt>},
    BreakStmt,
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Box<Stmt>>,
    },
    ReturnStmt {
        keyword: Token,
        value: Option<Expr>
    }
}



impl Stmt {
    #[allow(dead_code)]
    pub fn to_string(&self) -> String {
        match self {
            Stmt::Expression { expression } => expression.to_string(),
            Stmt::Print { expression } => format!("(print {})", expression.to_string()),
            Stmt::Let { name, initializer } => format!("(let {} = {})", name.to_string(), initializer.to_string()),
            Stmt::Block { statements } => format!("[{:?}]", 
                statements.into_iter().map(|stmt| stmt.to_string()).collect::<String>()
            ),
            Stmt::IfStmt { predicate, then, els } => format!("if {:?} {:?} else {:?}", predicate, then, els),
            Stmt::WhileStmt { condition, body } => format!("while ({:?}) ({:?})", condition, body),
            Stmt::BreakStmt => format!("break"),
            Stmt::Function { name, params, body } => format!("{:?} | {:?} | {:?}", name, params, body),
            Stmt::ReturnStmt { value , keyword: _ } => format!("returning: {:?}", value)
        }
    }
}