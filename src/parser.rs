use core::panic;

use crate::lexer::{Lexer, TokenType};

#[derive(Debug, PartialEq)]
pub enum ASTNode {
    Assignment(String, Box<ASTNode>),
    Number(f64),
    Identifier(String),
    BinaryOperation {
        left: Box<ASTNode>,
        operator: String,
        right: Box<ASTNode>,
    },
    Grouping(Box<ASTNode>),
}

pub struct Parser {
    lexer: Lexer,
    current_token: TokenType,
}

impl Parser {
    pub fn new(lexer: Lexer) -> Self {
        let mut parser: Parser = Parser {
            lexer,
            current_token: TokenType::EOF,
        };
        parser.advance();
        parser
    }

    fn advance(&mut self) {
        self.current_token = self.lexer.next_token();
    }

    fn parse_primary(&mut self) -> ASTNode {
        match &self.current_token {
            TokenType::Number(value) => {
                let node: ASTNode = ASTNode::Number(*value);
                self.advance();
                node
            }
            TokenType::Identifier(name) => {
                let node: ASTNode = ASTNode::Identifier(name.clone());
                self.advance();
                node
            }
            TokenType::ParenLeft => {
                self.advance();
                let expr: ASTNode = self.parse_expression();
                if self.current_token != TokenType::ParenRight {
                    panic!("Expected ')'");
                }
                self.advance();
                ASTNode::Grouping(Box::new(expr))
            }
            _ => panic!("Unexpected token: {:?}", self.current_token),
        }
    }

    pub fn parse_statement(&mut self) -> ASTNode {
        if let TokenType::Keyword(keyword) = &self.current_token {
            if keyword == "let" {
                return self.parse_assignment()
            }
        }
        self.parse_expression()
    }

    fn parse_assignment(&mut self) -> ASTNode {
        if let TokenType::Keyword(keyword) = &self.current_token {
            if keyword == "let" {
                self.advance();
                if let TokenType::Identifier(name) = &self.current_token {
                    let var_name = name.clone();
                    self.advance();
                    if self.current_token == TokenType::Operator("=".to_string()) {
                        self.advance();
                        let value = self.parse_expression();
                        return ASTNode::Assignment(var_name, Box::new(value));
                    } else {
                        panic!("Expected '=' after variable name");
                    }
                } else {
                    panic!("Expected identifier after 'let'");
                }
            }
        }
        panic!("Invalid assignment syntax");
    }

    fn parse_binary_op(&mut self, left: ASTNode, precedence: u8) -> ASTNode {
        let mut left_node: ASTNode = left;

        while let TokenType::Operator(op) = &self.current_token {
            let current_precedence: u8 = self.get_operator_precedence(op);
            if current_precedence < precedence {
                break;
            }

            let operator: String = op.clone();
            self.advance();
            let mut right_node: ASTNode = self.parse_primary();

            if let TokenType::Operator(next_op) = &self.current_token {
                let next_precedence: u8 = self.get_operator_precedence(next_op);
                if current_precedence < next_precedence {
                    right_node = self.parse_binary_op(right_node, current_precedence + 1);
                }
            }

            left_node = ASTNode::BinaryOperation {
                left: Box::new(left_node),
                operator,
                right: Box::new(right_node),
            };
        }

        left_node
    }

    pub fn parse_expression(&mut self) -> ASTNode {
        let left: ASTNode = self.parse_primary();
        self.parse_binary_op(left, 0)
    }

    fn get_operator_precedence(&self, op: &str) -> u8 {
        match op {
            "+" | "-" => 1,
            "*" | "/" => 2,
            _ => 0,
        }
    }
}