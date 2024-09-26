use crate::{environment, lexer::{self, Token}, TokenType};
use crate::environment::Environment;

#[derive(Debug, Clone, PartialEq)]
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

    pub fn to_type(&self) -> &str {
        match self {
            LiteralValueAst::Number(_) => "Number",
            LiteralValueAst::StringValue(_) => "String",
            LiteralValueAst::True => "Boolean",
            LiteralValueAst::False => "Boolean",
            LiteralValueAst::Null => "null"
        }
    }

    pub fn from_token(token : Token) -> Self {
        match token.token_type {
            TokenType::Number => Self::Number(unwrap_as_f32(token.literal)),
            TokenType::String => Self::StringValue(unwrap_as_string(token.literal)),

            TokenType::True => Self::True,
            TokenType::False => Self::False,
            TokenType::Null => Self::Null,

            _ => panic!("Could not create LiteralValue from {:?}",token)
        }
    }

    pub fn from_bool(boolean : bool) -> Self {
        if boolean {
            LiteralValueAst::True
        } else {
            LiteralValueAst::False
        }
    }

    pub fn is_falsy(&self) -> LiteralValueAst {

        match self {
            Self::Number(x) => {
                if *x == 0.0 {
                    Self::True
                } else {
                    Self::False
                }
            },
            Self::StringValue(s) => {
                if s.len() == 0 {
                    Self::True
                } else {
                    Self::False
                }
            },
            Self::True => Self::False,
            Self::False => Self::True,
            Self::Null => Self::True,
        }

    }

    pub fn is_truthy(&self) -> LiteralValueAst {

        match self {
            Self::Number(x) => {
                if *x == 0.0 {
                    Self::False
                } else {
                    Self::True
                }
            },
            Self::StringValue(s) => {
                if s.len() == 0 {
                    Self::False
                } else {
                    Self::True
                }
            },
            Self::True => Self::True,
            Self::False => Self::False,
            Self::Null => Self::False,
        }

    }

    fn is_false(&self) -> bool {
        match self {
            Self::Number(x) => *x == 0.0,
            Self::StringValue(s) => s.is_empty(),
            Self::True => false,
            Self::False => true,
            Self::Null => true,
        }
    }

}

#[derive(Debug, Clone)]
pub enum Expr {
    Assign {
        name: Token,
        value: Box<Expr>
    },
    Logical {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Binary { left: Box<Expr>,
        operator: Token,
        right: Box<Expr>
    },
    Grouping { expression: Box<Expr> },
    Literal { value: LiteralValueAst },
    Unary { 
        operator: Token,
        value: Box<Expr>
    },
    Ternary { condition: Box<Expr>, expr_true: Box<Expr>, expr_false: Box<Expr> },

    Variable { name: Token }
}

impl Expr {
    pub fn to_string(&self) -> String {
        
        match self {
            Expr::Assign { name, value } => format!("({name:?} = {}", value.to_string()),
            Expr::Logical { left, operator, right } => format!("{} {} {}",left.to_string(), operator.to_string(), right.to_string()),
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

            Expr::Ternary { condition, expr_true, expr_false } => {
                format!("{} ? {} : {}", condition.to_string(), expr_true.to_string(), expr_false.to_string())
            }
            Expr::Variable { name } => {format!("(let {} )", name.lexeme)}
        }

    }

    pub fn evaluate(&self, environment: &mut Environment) -> Result<LiteralValueAst, String> {
        match self {
            Expr::Assign { name, value } => {
                let new_value: LiteralValueAst = (*value).evaluate(environment)?;
                let assign_success: bool = environment.assign(&name.lexeme, new_value.clone());

                if assign_success {
                    Ok(new_value)
                } else {
                    Err(format!("Variable {:?} has not been declared", name.lexeme))
                }

            },
            Expr::Logical { left, operator, right } => todo!(),
            Expr::Literal { value } => Ok((*value).clone()),
            Expr::Grouping { expression } => expression.evaluate(environment),
            Expr::Unary { operator, value } => {

                let value: LiteralValueAst = value.evaluate(environment)?;

                match (operator.token_type, &value) {
                    (TokenType::Minus, LiteralValueAst::Number(x)) => Ok(LiteralValueAst::Number(-x)),
                    (TokenType::Minus, _) => return Err(format!("Minus not implemented for {:?}",value.to_type())),
                    (TokenType::Bang, any) => Ok(any.is_falsy()),
                    (ttype, _) => Err(format!("{} is not a valid unary operator", ttype)),
                }
            }
            Expr::Binary { left, operator, right } => {
                let left: LiteralValueAst = left.evaluate(environment)?;
                let right: LiteralValueAst = right.evaluate(environment)?;

                match (&left, operator.token_type, &right) {
                    (LiteralValueAst::Number(x), TokenType::Plus, LiteralValueAst::Number(y)) => 
                        Ok(LiteralValueAst::Number(x + y)),
                    (LiteralValueAst::Number(x), TokenType::Minus, LiteralValueAst::Number(y)) => 
                        Ok(LiteralValueAst::Number(x - y)),
                    (LiteralValueAst::Number(x), TokenType::Star, LiteralValueAst::Number(y)) => 
                        Ok(LiteralValueAst::Number(x * y)),
                    (LiteralValueAst::Number(x), TokenType::Slash, LiteralValueAst::Number(y)) => 
                        Ok(LiteralValueAst::Number(x / y)),
            
                    (LiteralValueAst::Number(x), TokenType::Greater, LiteralValueAst::Number(y)) => 
                        Ok(LiteralValueAst::from_bool(x > y)),
                    (LiteralValueAst::Number(x), TokenType::GreaterEqual, LiteralValueAst::Number(y)) => 
                        Ok(LiteralValueAst::from_bool(x >= y)),
                    (LiteralValueAst::Number(x), TokenType::Less, LiteralValueAst::Number(y)) => 
                        Ok(LiteralValueAst::from_bool(x < y)),
                    (LiteralValueAst::Number(x), TokenType::LessEqual, LiteralValueAst::Number(y)) => 
                        Ok(LiteralValueAst::from_bool(x <= y)),
            
                    (LiteralValueAst::StringValue(_), op, LiteralValueAst::Number(_)) => 
                        Err(format!("{} is not defined for string and number", op)),
                    (LiteralValueAst::Number(_), op, LiteralValueAst::StringValue(_)) => 
                    Err(format!("{} is not defined for string and number", op)),
                    
                    (LiteralValueAst::StringValue(s1), TokenType::Plus, LiteralValueAst::StringValue(s2)) =>
                        Ok(LiteralValueAst::StringValue(format!("{}{}", s1, s2))),
                    (LiteralValueAst::StringValue(s1), TokenType::EqualEqual, LiteralValueAst::StringValue(s2)) =>
                        Ok(LiteralValueAst::from_bool(s1 == s2)),
                    (LiteralValueAst::StringValue(s1), TokenType::Greater, LiteralValueAst::StringValue(s2)) =>
                        Ok(LiteralValueAst::from_bool(s1 > s2)),
                    (LiteralValueAst::StringValue(s1), TokenType::GreaterEqual, LiteralValueAst::StringValue(s2)) =>
                        Ok(LiteralValueAst::from_bool(s1 >= s2)),
                    (LiteralValueAst::StringValue(s1), TokenType::Less, LiteralValueAst::StringValue(s2)) =>
                        Ok(LiteralValueAst::from_bool(s1 < s2)),
                    (LiteralValueAst::StringValue(s1), TokenType::LessEqual, LiteralValueAst::StringValue(s2)) =>
                        Ok(LiteralValueAst::from_bool(s1 <= s2)),
                    
                    (x, TokenType::EqualEqual, y) =>
                        Ok(LiteralValueAst::from_bool(x == y)),
                    (x, TokenType::BangEqual, y) =>
                        Ok(LiteralValueAst::from_bool(x != y)),
                    
                    _ => Err("Case not supported".to_string()),
                }

            }
            Expr::Ternary { condition, expr_true, expr_false } => {

                let condition_value: LiteralValueAst = condition.evaluate(environment)?;

                if !condition_value.is_false() {
                    expr_true.evaluate(environment)
                } else {
                    expr_false.evaluate(environment)
                }
            },
            Expr::Variable { name } => {
                match environment.get(&name.lexeme) {
                    Some(value) => Ok(value.clone()),
                    None => Err(format!("Variable '{}' has not been declared", name.lexeme))
                }
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

    #[test]
    fn pretty_ternary_expr() {
        let condition: Box<Expr> = Box::new(Expr::Literal { value: LiteralValueAst::Number(1.0) });
        let expr_true: Box<Expr> = Box::new(Expr::Literal { value: LiteralValueAst::Number(2.0) });
        let expr_false: Box<Expr> = Box::new(Expr::Literal { value: LiteralValueAst::Number(3.0) });
        
        let ternary_expr: Expr = Expr::Ternary {
            condition,
            expr_true,
            expr_false,
        };

        assert_eq!(format!("{}", ternary_expr.to_string()), "1 ? 2 : 3");
    }

}