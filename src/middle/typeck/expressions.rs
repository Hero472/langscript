use crate::{core::types::{PrimitiveType, Type}, error::TypeError, frontend::{lexer::span::Span, parser::ast::{BinaryOp, Expr, UnaryOp}}, middle::typeck::utils::{type_to_string, types_are_compatible}, vm::value::Value};

pub struct ExpressionChecker;

impl ExpressionChecker {
    pub fn new() -> Self {
        Self
    }

    fn check_expression(&mut self, expr: &Expr) -> Result<Type, TypeError> {
        match expr {
            Expr::IntLiteral(_, _) => Ok(Type::Primitive(PrimitiveType::Int)),
            Expr::UintLiteral(_, _) => Ok(Type::Primitive(PrimitiveType::Uint)),
            Expr::FloatLiteral(_, _) => Ok(Type::Primitive(PrimitiveType::Float)),
            Expr::BoolLiteral(_, _) => Ok(Type::Primitive(PrimitiveType::Bool)),
            Expr::StringLiteral(_, _) => Ok(Type::Primitive(PrimitiveType::String)),
            Expr::CharLiteral(_, _) => Ok(Type::Primitive(PrimitiveType::Char)),
            
            Expr::Binary { left, op, right, span } => {
                let left_type = self.check_expression(left)?;
                let right_type = self.check_expression(right)?;
                
                if !types_are_compatible(&left_type, &right_type) {
                    return Err(TypeError::new(
                            span.clone(),
                            format!("Operation '{:?}' between incompatible types: {} and {}",
                                op,
                                type_to_string(&left_type),
                                type_to_string(&right_type)
                            )
                        )   
                    );
                }
                
                self.get_binary_op_result_type(op, &left_type, span)
            }
            
            Expr::Unary { op, expr, span } => {
                let expr_type = self.check_expression(expr)?;
                self.get_unary_op_result_type(op, &expr_type, span)
            }
            
            Expr::Grouped(expr, _) => self.check_expression(expr),
            
            // Add other expression types...
            _ => Err(TypeError::new(
                expr.span().clone(),
                "Expression type not yet supported in type checker"
            ))
        }
    }

    fn get_binary_op_result_type(
        &self, 
        op: &BinaryOp, 
        operand_type: &Type, 
        span: &Span
    ) -> Result<Type, TypeError> {
        match op {
            // Arithmetic operations (return same type as operands)
            BinaryOp::Add | BinaryOp::Subtract | BinaryOp::Multiply | BinaryOp::Divide => {
                if operand_type.is_numeric() {
                    Ok(operand_type.clone())
                } else {
                    Err(TypeError::new(
                            span.clone(),
                            format!("Arithmetic operation '{:?}' cannot be applied to non-numeric type {}",
                                op,
                                type_to_string(operand_type)
                            )
                        )
                    )
                }
            }
            
            // Comparison operations (always return bool)
            BinaryOp::Equals | BinaryOp::NotEquals |
            BinaryOp::LessThan | BinaryOp::LessEq |
            BinaryOp::GreaterThan | BinaryOp::GreaterEq => {
                if operand_type.is_comparable() {
                    Ok(Type::Primitive(PrimitiveType::Bool))
                } else {
                    Err(TypeError::new(
                        span.clone(),
                        format!("Comparison operation '{:?}' cannot be applied to type {}",
                            op, type_to_string(operand_type))
                    ))
                }
            }
            
            // Logical operations (require bool, return bool)
            BinaryOp::And | BinaryOp::Or => {
                if *operand_type == Type::Primitive(PrimitiveType::Bool) {
                    Ok(Type::Primitive(PrimitiveType::Bool))
                } else {
                    Err(TypeError::new(
                        span.clone(),
                        format!("Logical operation '{:?}' requires boolean operands, found {}",
                            op, type_to_string(operand_type))
                    ))
                }
            }
            _ => Err(TypeError::new(
                span.clone(),
                format!("Operator '{:?}' not supported for type {}", 
                    op, type_to_string(operand_type))
            ))
        }
    }

    fn get_unary_op_result_type(
        &self, 
        op: &UnaryOp , 
        operand_type: &Type, 
        span: &Span
    ) -> Result<Type, TypeError> {
        match op {
            // Numeric negation
            UnaryOp::Negate => {
                if operand_type.is_numeric() {
                    Ok(operand_type.clone())
                } else {
                    Err(TypeError::new(
                        span.clone(),
                        format!("Unary minus cannot be applied to non-numeric type {}",
                            type_to_string(operand_type))
                    ))
                }
            },
            UnaryOp::Not => {
                if *operand_type == Type::Primitive(PrimitiveType::Bool) {
                    Ok(Type::Primitive(PrimitiveType::Bool))
                } else {
                    Err(TypeError::new(
                        span.clone(),
                        format!("Logical not requires boolean operand, found {}",
                            type_to_string(operand_type))
                    ))
                }
            }
        }
    }

}