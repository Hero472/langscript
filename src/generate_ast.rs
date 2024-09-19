use crate::lexer::{self, Token};

pub enum LiteralValueAst {
    Number(f32),
    StringValue(String),
    True,
    False,
    Null
}

fn unwrap_as_f32(literal: Option<lexer::LiteralValue>) -> f32 {
    match literal {
        Some(lexer::LiteralValue::IntValue(x)) => x as f32,
        Some(lexer::LiteralValue::FloatValue(x)) => x as f32,
        _ => panic!("Could not unwrap as f32")
    }
}

fn unwrap_as_string(literal: Option<lexer::LiteralValue>) -> String {
    match literal {
        Some(lexer::LiteralValue::StringValue(s)) => s.clone(),
        Some(lexer::LiteralValue::Identifier(s)) => s.clone(),
        _ => panic!("Could not unwrap as f32")
    }
}

impl LiteralValueAst {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValueAst::Number(x) => x.to_string(),
            LiteralValueAst::StringValue(x) => x.clone(),
            LiteralValueAst::True => "true".to_string(),
            LiteralValueAst::False => "false".to_string(),
            LiteralValueAst::Null => "null".to_string()
        }
    }

    pub fn from_token(token : Token) -> Self {
        match token.token_type {
            number => Self::Number(unwrap_as_f32(token.literal)),
            string_value => Self::StringValue(unwrap_as_string(token.literal)),

            r#true => Self::True,
            r#false => Self::False,
            null => Self::Null,

            _ => panic!("Could not create LiteralValue from {:?}",token)
        }
    }

}

pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr>},
    Grouping { expression: Box<Expr> },
    Literal { value: LiteralValueAst },
    Unary { operator: Token, value: Box<Expr> }
}

impl Expr {
    pub fn to_string(&self) -> String {
        
        match self {
            
            Expr::Binary { left, operator, right } => {
                format!("{} {} {}", left.to_string(), operator.lexeme, right.to_string())
            },

            Expr::Grouping { expression } => format!("(group {})",expression.to_string()),

            Expr::Literal { value } => format!("{}",value.to_string()),

            Expr::Unary { operator, value } => {
                let operator_str = &operator.lexeme;
                let right_str = value.to_string();
                format!("({} {})", operator_str, right_str)
            }
        }

    }

    pub fn print(&self) {
        println!("{}",self.to_string());
    }
}

#[cfg(test)]
mod tests {
    use crate::TokenType;

    use super::Expr::*;
    use super::*;

    #[test]
    fn pretty_print_ast() {
        let minus_token: Token = Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: None, line_number: 1 };
        let one_two_three: Expr = Literal { value: LiteralValueAst::Number(123.0) };
        let group: Expr = Grouping { expression: Box::from(Literal {value: LiteralValueAst::Number(45.67)}) };
        let multi: Token = Token { token_type: TokenType::Star, lexeme: "*".to_string(), literal: None, line_number: 1 };
        let ast: Expr = Binary { left: Box::from(Unary { operator: minus_token, value: Box::from(one_two_three) }), operator: multi, right: Box::from(group) };

        let result: String = ast.to_string();

        assert_eq!(result,"(- 123) * (group 45.67)");
    }
}