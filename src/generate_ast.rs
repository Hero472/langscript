use crate::lexer::Token;

pub enum LiteralValue {
    Number(f32),
    StringValue(String),
    True,
    False,
    Null
}

impl LiteralValue {
    pub fn to_string(&self) -> String {
        match self {
            LiteralValue::Number(x) => x.to_string(),
            LiteralValue::StringValue(x) => x.clone(),
            LiteralValue::True => "true".to_string(),
            LiteralValue::False => "false".to_string(),
            LiteralValue::Null => "null".to_string()
        }
    }
}

pub enum Expr {
    Binary { left: Box<Expr>, operator: Token, right: Box<Expr>},
    Grouping { expression: Box<Expr> },
    Literal { value: LiteralValue },
    Unary { operator: Token, value: Box<Expr> }
}

impl Expr {
    pub fn to_string(&self) -> String {
        
        match self {
            Expr::Binary { left, operator, right } => {
                format!("{} {} {}", left.to_string(), operator.to_string(), right.to_string())
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
        let minus_token = Token { token_type: TokenType::Minus, lexeme: "-".to_string(), literal: None, line_number: 1 };
        let one_two_three = Literal { value: LiteralValue::Number(123.0) };
        let group = Grouping { expression: Box::from(Literal {value: LiteralValue::Number(45.67)}) };
        let multi = Token { token_type: TokenType::Star, lexeme: "*".to_string(), literal: None, line_number: 1 };
        let ast = Binary { left: Box::from(Unary { operator: minus_token, value: Box::from(one_two_three) }), operator: multi, right: Box::from(group) };

        let result = ast.to_string();

        assert_eq!(result,"(- 123) * (group 45.67)");
    }
}