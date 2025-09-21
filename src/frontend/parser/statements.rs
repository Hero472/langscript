use crate::{core::types::{PrimitiveType, Type}, frontend::{lexer::{span::Span, tokens::Token}, parser::{ast::Stmt, core::ParserCore, error::ParserError, expressions::ExpressionParser}}};

pub trait StatementParser {
    fn statement(&mut self) -> Result<Stmt, ParserError>;
    fn let_statement(&mut self) -> Result<Stmt, ParserError>;
    fn print_statement(&mut self) -> Result<Stmt, ParserError>;
    fn consume_identifier(&mut self, message: &str) -> Result<String, ParserError>;
    fn consume_token(&mut self, expected: Token, message: &str) -> Result<(), ParserError>;
    fn parse_type(&mut self) -> Result<Type, ParserError>;
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
        use super::ExpressionParser;
        
        // let
        let let_span = self.previous_token().unwrap().1;

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

        let span = Span {
            start_line: let_span.start_line,
            start_column: let_span.start_column,
            end_line: value.span().end_line,
            end_column: value.span().end_column,
            file_id: let_span.file_id,
        };

        Ok(Stmt::Let {
            name,
            value,
            type_annotation,
            span,
        })

    }

    fn print_statement(&mut self) -> Result<Stmt, ParserError> {
        todo!()
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

    fn parse_type(&mut self) -> Result<Type, ParserError> {

        let (token, span)= self.actual_token()
            .clone();

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