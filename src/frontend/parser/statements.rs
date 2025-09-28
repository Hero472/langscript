use crate::{core::types::{PrimitiveType, Type}, frontend::{lexer::{span::Span, tokens::Token}, parser::{ast::{Param, Stmt}, core::ParserCore, error::ParserError, expressions::ExpressionParser}}};

pub trait StatementParser {
    fn statement(&mut self) -> Result<Stmt, ParserError>;
    fn let_statement(&mut self) -> Result<Stmt, ParserError>;
    fn function_statement(&mut self) -> Result<Stmt, ParserError>;
    fn block_statement(&mut self) -> Result<Stmt, ParserError>;
    fn if_statement(&mut self) -> Result<Stmt, ParserError>;
    fn consume_identifier(&mut self, message: &str) -> Result<String, ParserError>;
    fn consume_token(&mut self, expected: Token, message: &str) -> Result<(), ParserError>;
    fn consume_semicolon(&mut self) -> Result<(), ParserError> ;
    fn parse_type(&mut self) -> Result<Type, ParserError>;
}

impl StatementParser for ParserCore {

    fn statement(&mut self) -> Result<Stmt, ParserError> {
        if self.matches(Token::Let) {
            self.let_statement()
        } else if self.matches(Token::Fn) {
            self.function_statement()
        } else if self.matches(Token::If) {
            self.if_statement()
        } else {
            let expr = self.expression()?;
            let span = expr.span();
            Ok(Stmt::Expr(expr, span))
        }
    }

    fn let_statement(&mut self) -> Result<Stmt, ParserError> {
        use super::ExpressionParser;
        
        let mut mutable = false;

        // let
        let let_span = self.previous_token().unwrap().1;

        // let mut
        if self.matches(Token::Mut) {
            mutable = true;
        }

        // let name
        let name = self.consume_identifier("Expect variable name after 'let'")?;

        // let name:
        let type_annotation = if self.matches(Token::Colon) {
            Some(self.parse_type()?)
        } else {
            None
        };

        self.consume(Token::Equals, "Expect '=' after variable name")?;

        let value = self.expression()?;

        let span = let_span.merge(&value.span());

        self.consume_semicolon()?;

        Ok(Stmt::Let {
            name,
            value,
            type_annotation,
            span,
            mutable
        })

    }

    fn function_statement(&mut self) -> Result<Stmt, ParserError> {

        // fn
        let fn_span = self.previous_token().unwrap().1;

        // fn name
        let name = self.consume_identifier("Expect function name after 'fn'")?;

        // fn name(
        self.consume_token(Token::LParen, "Expect left parenthesis after function name")?;

        let mut params = vec![];

        if !self.check(&Token::RParen) {
            loop {
                let param_name = self.consume_identifier("Expect parameter name")?;

                self.consume(Token::Colon, "Expect ':' after parameter name")?;

                let param_type = self.parse_type()?;

                params.push(Param {
                    name: param_name,
                    type_annotation: param_type,
                });

                if !self.matches(Token::Comma) {
                    break;
                }

            }
        }

        self.consume(Token::RParen, "Expect ')' after parameters")?;

        let return_type = if self.matches(Token::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };

        let body = self.block_statement()?;

        let span = fn_span.merge(&body.span());

        Ok(Stmt::Function {
            name,
            params,
            return_type,
            body: Box::new(body),
            span,
        })

    }

    fn block_statement(&mut self) -> Result<Stmt, ParserError> {

        let left_brace_span = self.actual_token().1;

        let mut statements = vec![];
        
        // consume '{'
        self.advance();

        while !self.check(&Token::RBrace) && !self.is_at_end() {
            statements.push(self.statement()?);
        }

        self.consume(Token::RBrace, "Expect '}' after block")?;
        let right_brace_span = self.previous_token().unwrap().1;
        
        let span = left_brace_span.merge(&right_brace_span);

        Ok(Stmt::Block(statements, span))

    }

    fn if_statement(&mut self) -> Result<Stmt, ParserError> {
        
        let start_span = self.previous_token().unwrap().1;

        let condition = self.expression()?;

        let then_branch = Box::new(self.block_statement()?);

        let else_branch;

        if self.matches(Token::Else) {

            else_branch = Some(Box::new(self.block_statement()?));

        } else {
            else_branch = None
        };

        Ok(
            Stmt::If { 
                condition,
                then_branch,
                else_branch,
                span: start_span 
            
            }
        )
    }

    // Helper Functions

    fn consume_token(&mut self, expected: Token, message: &str) -> Result<(), ParserError> {
        match self.peek_token() {
            Some((ref token, _)) if token == &expected => {
                self.advance();
                Ok(())
            }
            Some((found, _)) => {
                Err(self.error(&format!("{} (expected {:?}, found {:?})", message, expected, found)))
            }
            None => {
                Err(self.error(&format!("{} (reached end of input)", message)))
            }
        }
    }

    fn consume_identifier(&mut self, message: &str) -> Result<String, ParserError> {
    // Get the current token and span
        let (token, span) = self.actual_token().clone();

        match token {
            Token::Identifier(name) => {
                self.advance(); // Consume the identifier token
                Ok(name)
            }
            other_token => {
                Err(self.error(&format!("{} (found {:?}) at line {} column {}",
                            message,
                            other_token,
                            span.start_line,
                            span.start_column
                        )
                    )
                )
            }
        }
    }

    fn consume_semicolon(&mut self) -> Result<(), ParserError> {
        let span = self.current.1;
        self.consume(Token::Semicolon, &format!("Expected semicolon ';' at the end of statement at line {} column {}", span.start_line, span.start_column))
    }

    fn parse_type(&mut self) -> Result<Type, ParserError> {

        let (token, span)= self.actual_token().clone();

        match &token {
            Token::Identifier(name) => {
                match name.as_str() {
                    "bool" => {
                        self.consume(Token::Identifier("bool".to_string()), "Expected 'bool'")?;
                        Ok(Type::Primitive(PrimitiveType::Bool))
                    }
                    "int" => {
                        self.consume(Token::Identifier("int".to_string()), "Expected 'int'")?;
                        Ok(Type::Primitive(PrimitiveType::Int))
                    }
                    "uint" => {
                        self.consume(Token::Identifier("uint".to_string()), "Expected 'uint'")?;
                        Ok(Type::Primitive(PrimitiveType::Uint))
                    }
                    "float" => {
                        self.consume(Token::Identifier("float".to_string()), "Expected 'float'")?;
                        Ok(Type::Primitive(PrimitiveType::Float))
                    }
                    "char" => {
                        self.consume(Token::Identifier("char".to_string()), "Expected 'char'")?;
                        Ok(Type::Primitive(PrimitiveType::Char))
                    }
                    "string" => {
                        self.consume(Token::Identifier("string".to_string()), "Expected 'string'")?;
                        Ok(Type::Primitive(PrimitiveType::String))
                    }
                    _ => Err(self.error(&format!("Unknown type: {}", name))),
                    // more to come
                }
            },
            _ => Err(self.error(&format!("Unexpected token in type position: {:?}", token))),
        }
    }

}