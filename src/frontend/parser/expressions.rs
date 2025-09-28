use crate::frontend::{lexer::{span::Span, tokens::Token}, parser::{ast::{BinaryOp, Expr, UnaryOp}, core::ParserCore, error::ParserError}};

pub trait ExpressionParser {
    fn expression(&mut self) -> Result<Expr, ParserError>;
    fn equality(&mut self) -> Result<Expr, ParserError>;
    fn comparison(&mut self) -> Result<Expr, ParserError>;
    fn term(&mut self) -> Result<Expr, ParserError>;
    fn factor(&mut self) -> Result<Expr, ParserError>;
    fn unary(&mut self) -> Result<Expr, ParserError>;
    fn primary(&mut self) -> Result<Expr, ParserError>;
}

impl ExpressionParser for ParserCore {
    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.comparison()?;

        while matches!(self.current.0, Token::DoubleEquals | Token::BangEquals) {

            let op = match &self.current.0 {
                Token::DoubleEquals => BinaryOp::Equals,
                Token::BangEquals => BinaryOp::NotEquals,
                _ => unreachable!()
            };

            self.advance();

            let right = Box::from(self.comparison()?);

            let span = expr.span().merge(&right.span());

            expr = Expr::Binary {
                left: Box::from(expr),
                op,
                right,
                span
            }
        }
        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.term()?;
        
        while matches!(self.current.0, Token::Greater | Token::GreaterEqual | Token::Less | Token::LessEqual) {
            let op = match &self.current.0 {
                Token::Greater => BinaryOp::GreaterThan,
                Token::GreaterEqual => BinaryOp::GreaterEq,
                Token::Less => BinaryOp::LessThan,
                Token::LessEqual => BinaryOp::LessEq,
                _ => unreachable!()
            };

            self.advance();
            let right = Box::from(self.term()?);

            let span = expr.span().merge(&right.span());

            expr = Expr::Binary {
                left: Box::from(expr),
                op,
                right,
                span
            }

        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.factor()?;

        while matches!(self.current.0, Token::Minus | Token:: Plus) {

            let op = match &self.current.0 {
                Token::Plus => BinaryOp::Add,
                Token::Minus => BinaryOp::Subtract,
                _ => unreachable!(),
            };

            self.advance();

            let right = Box::from(self.factor()?);

            let span = expr.span().merge(&right.span());

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right,
                span
            };

        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = self.unary()?;

        while matches!(self.current.0, Token::Star | Token::Slash) {

            let op = match &self.current.0 {
                Token::Star => BinaryOp::Multiply,
                Token::Slash => BinaryOp::Divide,
                _ => unreachable!(),
            };

            self.advance();

            let right = Box::from(self.unary()?);

            let span = expr.span().merge(&right.span());

            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right,
                span
            };
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {

        if matches!(self.current.0, Token::Bang | Token::Minus | Token::Plus) {

            let operator_span = self.current.1;

            let op = match &self.current.0 {
                Token::Bang => UnaryOp::Not,
                Token::Minus => UnaryOp::Negate,
                _ => unreachable!()
            };

            self.advance();

            let expr = self.unary()?;

            let combined_span = operator_span.merge(&expr.span());

            let expr = Expr::Unary {
                op,
                expr: Box::new(expr),
                span: combined_span
            };

            Ok(expr)
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {

        match self.current.0.clone() {
            Token::IntLiteral(n) => {
                self.advance();
                return Ok(Expr::IntLiteral(n, *self.current_span()))
            },
            Token::UintLiteral(n) => {
                self.advance();
                return Ok(Expr::UintLiteral(n, *self.current_span()))
            }
            Token::FloatLiteral(f) => {
                self.advance();
                return Ok(Expr::FloatLiteral(f, *self.current_span()))
            },
            Token::CharLiteral(c) => {
                self.advance();
                return Ok(Expr::CharLiteral(c, *self.current_span()))
            }
            Token::StringLiteral(s) => {
                self.advance();
                return Ok(Expr::StringLiteral(s, *self.current_span()))
            },
            Token::BoolLiteral(b) => {
                self.advance();
                return Ok(Expr::BoolLiteral(b, *self.current_span()))
            },
            Token::Identifier(name) => {
                self.advance();
                return Ok(Expr::Identifier(name, *self.current_span()))
            },
            Token::LParen => {
                self.advance();
                let expr = self.expression()?;
                self.consume(Token::RParen, "Expect ')' after expression.")?;
                return Ok(Expr::Grouped(Box::from(expr), *self.current_span()))
            },
            t => return Err(self.error(&format!("Expect expression, got {:?} at {:?}", t, self.current_span())))
        }
    }
}