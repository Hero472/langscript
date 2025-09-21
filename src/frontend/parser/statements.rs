use crate::frontend::{lexer::tokens::Token, parser::{ast::Stmt, core::ParserCore, error::ParserError, expressions::ExpressionParser}};

pub trait StatementParser {
    fn statement(&mut self) -> Result<Stmt, ParserError>;
    fn let_statement(&mut self) -> Result<Stmt, ParserError>;
}

impl StatementParser for ParserCore {
    fn statement(&mut self) -> Result<Stmt, ParserError> {
        if self.matches(Token::Let) {
            self.let_statement()
        } else {
            let expr = self.expression()?;
            let span = expr.span();
            Ok(Stmt::Expr(expr, span))
        }
    }

    fn let_statement(&mut self) -> Result<Stmt, ParserError> {
        // Implementation for let statement
        todo!()
    }
}