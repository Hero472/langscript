use crate::{generate_ast::{Expr, LiteralValueAst}, lexer::{Token, TokenType}};

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

    // checks if there is an equality
    pub fn expression(&mut self) -> Result<Expr,String> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr,String> {
        
        let mut expr: Expr = self.comparison()?;


        while self.match_tokens(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            
            let operator: Token = self.previous().clone();
            let rhs: Expr = self.comparison()?;

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
            self.primary()
        }

    }

    fn primary(&mut self) -> Result<Expr,String> {
        if self.match_token(&TokenType::LeftParen) {
            let expr: Expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expected ')'");
            Ok(Expr::Grouping { expression: Box::from(expr) })
        } else {
            let token: Token = self.peek();
            self.advance();
            Ok(Expr::Literal { value: LiteralValueAst::from_token(token) })
        }
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<(), String>{
        let token: Token = self.peek();
        if token.token_type == token_type {
            self.advance();
            Ok(())
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

}

#[cfg(test)]
mod tests {

    use crate::{Lexer, LiteralValue};

    use super::*;

    #[test]
    fn test_addition_parser() {
        let one: Token = Token { token_type: TokenType::Number, lexeme: "1".to_string(), literal: Some(LiteralValue::IntValue(1)), line_number: 1 };
        let plus: Token = Token { token_type: TokenType::Plus, lexeme: "+".to_string(), literal: None, line_number: 1 };
        let two: Token = Token { token_type: TokenType::Number, lexeme: "2".to_string(), literal: Some(LiteralValue::IntValue(2)), line_number: 1 };
        let semi_colon: Token = Token { token_type: TokenType::Semicolon, lexeme: ";".to_string(), literal: None, line_number: 1 };

        // 1+2;

        let tokens: Vec<Token> = vec![one, plus, two, semi_colon];
        let mut parser: Parser = Parser::new(tokens);

        let parsed_expr: Expr = parser.expression().unwrap();

        let string_expr: String = parsed_expr.to_string();

        assert_eq!(string_expr,"1 + 2");
    }

    #[test]
    fn test_comparison(){
        let source = "3 + 5 == 7 - 5";
        let mut lexer = Lexer::new(source);
        let tokens = lexer.scan_tokens().unwrap();
        println!("{:?}",tokens[3]);
        let mut parser = Parser::new(tokens);
        let parsed_expr: Expr = parser.expression().unwrap();
        let string_expr: String = parsed_expr.to_string();

        assert_eq!(string_expr,"3 + 5 == 7 - 5")
    }

}