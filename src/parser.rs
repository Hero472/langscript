use std::fmt::{format, Arguments};

use crate::{generate_ast::{Expr, LiteralValueAst}, lexer::{Token, TokenType}};
use crate::stmt::Stmt;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {

    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut stmts: Vec<Stmt> = vec![];
        let mut errors: Vec<String> = vec![];

        while !self.is_at_end() {
            let stmt: Result<Stmt, String> = self.declaration();
            match stmt {
                Ok(s) => stmts.push(s),
                Err(msg) => {
                    errors.push(msg);
                    self.synchronize();
                },
            }
        }

        if errors.len() == 0 {
            Ok(stmts)
        } else {
            Err(errors.join("\n"))
        }
    }

    fn declaration(&mut self) -> Result<Stmt, String> {
        if self.match_token(&TokenType::Let) {
            
            match self.var_declaration() {
                Ok(stmt) => Ok(stmt),
                Err(msg) => Err(msg)
            }
        } else {
            self.statement()
        }
    }

    fn var_declaration(&mut self) -> Result<Stmt, String> {
        let token: Token = self.consume(TokenType::Identifier, "Expected variable name")?;

        let initializer: Expr; 
        if self.match_token(&TokenType::Equal) {
            initializer = self.expression()?;
        } else {
            initializer = Expr::Literal { value: LiteralValueAst::Null }
        }
        let _ = self.consume(TokenType::Semicolon, "Expected ';' after variable declaration");
        Ok(Stmt::Let { name: token, initializer: initializer })
    } 

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.match_token(&TokenType::Print) {
            self.print_statement()
        } else if self.match_token(&TokenType::LeftBrace) {
            self.block_statement()
        } else if self.match_token(&TokenType::If) {
            self.if_statement()
        } else if self.match_token(&TokenType::While){
            self.while_statement()
        } else if self.match_token(&TokenType::For) {
            self.for_statement()
        } else if self.match_token(&TokenType::Break){
            self.break_statement()
        } else {
            self.expression_statement()
        }

    }

    fn break_statement(&mut self) -> Result<Stmt, String> {
        let _ = self.consume(TokenType::Semicolon, "expected ';' after 'break'");
        Ok(Stmt::BreakStmt)
    }

    fn for_statement(&mut self) -> Result<Stmt, String> {

        self.consume(TokenType::LeftParen, "Expected '(' after 'for'.")?;

        let initializer: Option<Stmt>;
        if self.match_token(&TokenType::Semicolon) {
            initializer = None;
        } else if self.match_token(&TokenType::Let) {
            let var_decl: Stmt = self.var_declaration()?;
            initializer = Some(var_decl);
        } else {
            let expr: Stmt = self.expression_statement()?;
            initializer = Some(expr);
        }

        let condition: Option<Expr>;
        if !self.check(TokenType::Semicolon) {
            let expr = self.expression()?;
            condition = Some(expr);
        } else {
            condition = None;
        }
        self.consume(TokenType::Semicolon, "Expected ';' after loop condition.")?;

        let increment: Option<Expr>;
        if !self.check(TokenType::RightParen) {
            let expr: Expr = self.expression()?;
            increment = Some(expr);
        } else {
            increment = None;
        }
        self.consume(TokenType::RightParen, "Expected ')' after for clauses.")?;

        let mut body: Stmt = self.statement()?;

        if let Some(incr) = increment {
            body = Stmt::Block {
                statements: vec![
                    Box::new(body),
                    Box::new(Stmt::Expression { expression: incr }),
                ],
            };
        }

        let cond: Expr;
        match condition {
            None => {
                cond = Expr::Literal {
                    value: LiteralValueAst::True,
                }
            }
            Some(c) => cond = c,
        }
        body = Stmt::WhileStmt {
            condition: cond,
            body: Box::new(body),
        };

        if let Some(init) = initializer {
            body = Stmt::Block {
                statements: vec![Box::new(init), Box::new(body)],
            };
        }

        Ok(body)
    }


    fn while_statement(&mut self) -> Result<Stmt, String> {
        let _ = self.consume(TokenType::LeftParen, "expected '(' after 'while'");
        let condition: Expr = self.expression()?;
        let _ = self.consume(TokenType::RightParen, "expected ')' after 'while'");
        let body: Stmt = self.statement()?;

        Ok(Stmt::WhileStmt { condition: condition, body: Box::new(body) })

    }

    fn if_statement(&mut self) -> Result<Stmt, String> {
        self.consume(TokenType::LeftParen, "Expected '(' after 'if'")?;
        let predicate: Expr = self.expression()?;
        self.consume(TokenType::RightParen, "Expected ')' after if-predicate")?;

        let then: Box<Stmt> = Box::new(self.statement()?);
        let els: Option<Box<Stmt>> = if self.match_token(&TokenType::Else) {
            let stm: Stmt = self.statement()?;
            Some(Box::new(stm))
        } else {
            None
        };

        Ok(Stmt::IfStmt { predicate, then, els })
    }


    fn block_statement(&mut self) -> Result<Stmt, String> {
        let mut statements: Vec<Box<Stmt>> = vec![];

        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            let decl: Stmt = self.declaration()?;
            statements.push(Box::new(decl));
        }

        self.consume(TokenType::RightBrace, "Expected '}' after a block")?;
        Ok(Stmt::Block { statements })
    }

    fn print_statement(&mut self) -> Result<Stmt, String> {
        let value: Expr = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value.")?;
        Ok(Stmt::Print { expression: value })
    }

    fn expression_statement(&mut self) -> Result<Stmt, String> {
        let expr: Expr = self.expression()?;
        let _ = self.consume(TokenType::Semicolon, "Expected ';' after statement");
        return Ok(Stmt::Expression { expression: expr })
    }

    // checks if there is an equality
    fn expression(&mut self) -> Result<Expr,String> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        let expr: Expr = self.or()?;

        if self.match_token(&TokenType::Equal) {
            let value: Expr = self.assignment()?;

            match expr {
                Expr::Variable { name } => {
                    Ok(Expr::Assign { name: name, value: Box::from(value) })
                },
                _ => Err("Invalid assignment target".to_string()),
            }

        } else {
            Ok(expr)
        }
    }

    fn or(&mut self) -> Result<Expr, String> {
        let mut expr: Expr = self.and()?;

        while self.match_token(&TokenType::Or) {
            let operator: Token = self.previous();
            let right: Expr = self.and()?;
            expr = Expr::Logical { left: Box::new(expr), operator: operator, right: Box::new(right) }
        }
        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, String> {
        let mut expr: Expr = self.ternary()?;

        while self.match_token(&TokenType::And) {
            let operator: Token = self.previous();
            let right: Expr = self.ternary()?;
            expr = Expr::Logical { left: Box::new(expr), operator: operator, right: Box::new(right) }
        }
        Ok(expr)
    }

    // you can do nest ternary expression...
    fn ternary(&mut self) -> Result<Expr,String> {
        let condition: Expr = self.equality()?;

        if self.match_token(&TokenType::QuestionMark) {
            let expr_true: Expr = self.expression()?;
            let _ = self.consume(TokenType::Colon, "Expected ':' after true expression");
            let expr_false: Expr = self.expression()?;

            return Ok(Expr::Ternary {
                condition: Box::new(condition),
                expr_true: Box::new(expr_true),
                expr_false: Box::new(expr_false),
            });
        }

        Ok(condition)

    }

    fn equality(&mut self) -> Result<Expr,String> {
        
        let mut expr: Expr = self.comparison()?;

        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            
            let operator: Token = self.previous().clone();
            let rhs: Expr = self.term()?;

            expr = Expr::Binary { left: Box::from(expr), operator: operator, right: Box::from(rhs) };
        }
        Ok(expr)
    }

    // compares greater, greater equal, less, less equal and put it in a Expr:Binary
    fn comparison(&mut self) -> Result<Expr,String> {
        let mut expr: Expr = self.term()?;

        while self.match_tokens(&[TokenType::Greater, TokenType::GreaterEqual, TokenType::Less, TokenType::LessEqual]) {

            let op: Token = self.previous();
            let rhs: Expr = self.term()?;

            expr = Expr::Binary { left: Box::from(expr), operator: op, right: Box::from(rhs) };

        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr,String> {
        let mut expr: Expr = self.factor()?;

        while self.match_tokens(&[TokenType::Minus, TokenType::Plus]) {
            let op: Token = self.previous();
            let rhs: Expr = self.factor()?;

            expr = Expr::Binary { left: Box::from(expr), operator: op, right: Box::from(rhs) };

        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr,String> {
        let mut expr: Expr = self.unary()?;

        while self.match_tokens(&[TokenType::Slash, TokenType::Star]) {
            let op: Token = self.previous();
            let rhs: Expr = self.unary()?;

            expr = Expr::Binary { left: Box::from(expr), operator: op, right: Box::from(rhs) };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr,String> {
        
        if self.match_tokens(&[TokenType::Bang, TokenType::Minus]) {
            let op: Token = self.previous();
            let rhs: Expr = self.unary()?;

            Ok(Expr::Unary { operator: op, value: Box::from(rhs) })

        } else {
            self.call()
        }

    }

    fn call(&mut self) -> Result<Expr, String> {
        let mut expr: Expr = self.primary()?;

        loop {
            if self.match_token(&TokenType::LeftParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn finish_call(&mut self, callee: Expr) -> Result<Expr, String> {
        let mut args: Vec<Expr> = vec![];

        if !self.check(TokenType::RightParen) {
            loop {
                let arg: Expr = self.expression()?;
                args.push(arg);
                
                if args.len() >= 255 {
                    let location: usize = self.peek().line_number;
                    return Err(format!("Function cant have more than 255 arguments, in line {}", location))
                }
                
                if !self.match_token(&TokenType::Comma){
                    break;
                }
            }
        }
        let paren = self.consume(TokenType::RightParen, "Expected ')' after arguments")?;

        Ok(Expr::Call { callee: Box::new(callee), paren: paren, arguments: args })
    }

    fn primary(&mut self) -> Result<Expr, String> {
        let token: Token = self.peek();
        let result: Expr;
        match token.token_type {
            TokenType::LeftParen => {
                self.advance();
                let expr: Expr = self.expression()?;
                self.consume(TokenType::RightParen, "Expected ')'")?;
                result = Expr::Grouping {
                    expression: Box::from(expr),
                };
            }
            TokenType::False | TokenType::True | TokenType::Null | TokenType::Number | TokenType::String => {
                self.advance();
                result = Expr::Literal {
                    value: LiteralValueAst::from_token(token),
                }
            }
            TokenType::Identifier => {
                self.advance();
                result = Expr::Variable {
                    name: self.previous(),
                };
            }
            ttype => return Err(format!("Expected expression, last token read was {}", ttype)),
        }

        Ok(result)
    }


    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, String>{
        let token: Token = self.peek();
        if token.token_type == token_type {
            self.advance();
            let token: Token = self.previous();
            Ok(token)
        } else {
            Err(msg.to_string())
        }
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].clone()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current].clone()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EOF
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn match_tokens(&mut self, typs: &[TokenType]) -> bool {
        for typ in typs {
            if self.match_token(typ) {
                return true
            }
        }
        false
    }

    fn match_token(&mut self, typ: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            if self.peek().token_type == *typ {
                self.advance();
                true
            } else {
                false
            }
        }
    }

    fn check(&mut self, typ: TokenType) -> bool {
        self.peek().token_type == typ
    }

    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::Semicolon { return; }

            match self.peek().token_type {
                TokenType::Class | TokenType::Fun | TokenType::Let | TokenType::If |
                TokenType::While | TokenType::Print | TokenType::Return => return,
                _ => (),
            }
            self.advance();
        }



    }

}

#[cfg(test)]
mod tests {

    use crate::Lexer;

    use super::*;

    fn stmt_vec_to_string(stmts: &Vec<Stmt>) -> String {
        stmts
            .iter()
            .map(|stmt| stmt.to_string())
            .collect::<Vec<String>>()
            .join("\n")
    }

    #[test]
    fn test_addition_parser() {
        let source: &str = "1 + 2;";
        let mut lexer: Lexer = Lexer::new(source);
        let tokens: Vec<Token> = lexer.scan_tokens().unwrap();
        let mut parser: Parser = Parser::new(tokens);

        let parsed_expr: Vec<Stmt> = parser.parse().unwrap();

        let string_expr: String = stmt_vec_to_string(&parsed_expr);

        assert_eq!(string_expr,"1 + 2");
    }

    #[test]
    fn test_comparison(){
        let source: &str = "3 + 5 == 7 - 5";
        let mut lexer: Lexer = Lexer::new(source);
        let tokens: Vec<Token> = lexer.scan_tokens().unwrap();
        let mut parser: Parser = Parser::new(tokens);
        let parsed_expr: Vec<Stmt> = parser.parse().unwrap();
        let string_expr: String = stmt_vec_to_string(&parsed_expr);
        assert_eq!(string_expr,"3 + 5 == 7 - 5")
    }

    #[test]
    fn test_quality_paren(){
        let source: &str = "4 == (2 + 2)";
        let mut lexer: Lexer = Lexer::new(source);
        let tokens: Vec<Token> = lexer.scan_tokens().unwrap();
        let mut parser: Parser = Parser::new(tokens);
        let parsed_expr: Vec<Stmt> = parser.parse().unwrap();
        let string_expr: String = stmt_vec_to_string(&parsed_expr);

        assert_eq!(string_expr,"4 == (group 2 + 2)")
    }

    #[test]
    fn test_ternary_expr() {
        let source: &str = "1 ? 2 : 3";
        let mut lexer: Lexer = Lexer::new(source);
        let tokens: Vec<Token> = lexer.scan_tokens().unwrap();
        let mut parser: Parser = Parser::new(tokens);
        let parsed_expr: Vec<Stmt> = parser.parse().unwrap();
        let string_expr: String = stmt_vec_to_string(&parsed_expr);

        assert_eq!(string_expr,"1 ? 2 : 3")
    }

}