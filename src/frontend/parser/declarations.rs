use crate::frontend::{lexer::tokens::Token, parser::{ast::{Declaration, FunctionDecl, LetDecl, Param}, core::ParserCore, error::ParserError, expressions::ExpressionParser, statements::StatementParser}};

pub trait DeclarationParser {
    fn declaration(&mut self) -> Result<Declaration, ParserError>;
    fn function_declaration(&mut self) -> Result<Declaration, ParserError>;
    fn struct_declaration(&mut self) -> Result<Declaration, ParserError>;
    fn enum_declaration(&mut self) -> Result<Declaration, ParserError>;
}

impl DeclarationParser for ParserCore {

    fn declaration(&mut self) -> Result<Declaration, ParserError> {
        if self.matches(Token::Fn) {
            self.function_declaration()
        } else if self.matches(Token::Struct) {
            self.struct_declaration()
        } else if self.matches(Token::Enum) {
            self.enum_declaration()
        } else {
            Err(ParserError::new(
                "Expected declaration: fn, struct, or enum".to_string(), 
                self.current_span().clone() // Use clone() instead of dereference
            ))
        }
    }

    fn function_declaration(&mut self) -> Result<Declaration, ParserError> {
        let start_span = self.previous_token().unwrap().1;

        // fn name
        let name = self.consume_identifier("Expect function name after 'fn'")?;

        // fn name(
        self.consume(Token::LParen, "Expect '(' after function name")?;
        
        let mut params = Vec::new();
        if !self.check(Token::RParen) {
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

        // fn name()
        self.consume(Token::RParen, "Expect ')' after parameters")?;
        
        // fn name() ->
        let return_type = if self.matches(Token::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };

        // fn name () -> {}
        let body = self.block_statement()?;
        let span = start_span.merge(&body.span());
        Ok(Declaration::Function(FunctionDecl {
            name,
            params,
            return_type,
            body,
        }, span))
    }

    fn struct_declaration(&mut self) -> Result<Declaration, ParserError> {
        todo!()
    }

    fn enum_declaration(&mut self) -> Result<Declaration, ParserError> {
        todo!()
    }

}