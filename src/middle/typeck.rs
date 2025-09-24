use std::collections::HashMap;

use crate::{core::types::{PrimitiveType, Type}, error::TypeError, frontend::{lexer::span::Span, parser::ast::{BinaryOp, Declaration, Expr, Program, Stmt, UnaryOp}}, vm::value::Value};

pub struct TypeChecker {
    type_context: HashMap<String, Type>, // Variables and their types
    scopes: Vec<HashMap<String, Type>>,  // For nested scopes
    current_return_type: Option<Type>,   // For function return checking
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            type_context: HashMap::new(),
            scopes: vec![HashMap::new()],
            current_return_type: None,
        }
    }

    pub fn check(&mut self, program: &Program)-> Result<(), Vec<TypeError>> {
        let mut type_errors = vec![];

        for decl in &program.declarations {
            if let Err(error) = self.check_declaration(&decl) {
                type_errors.push(error);
            }
        }

        if type_errors.is_empty() {
            Ok(())
        } else {
            Err(type_errors)
        }
    }

    fn check_declaration(&mut self, decl: &Declaration) -> Result<(), TypeError> {

        match decl {
            Declaration::Function(function_decl, span) => Ok(()),
            Declaration::Struct(struct_decl, span) => Ok(()),
            Declaration::Enum(enum_decl, span) => Ok(()),
        }

    }

    fn check_statement(&mut self, stmt: &Stmt) -> Result<(), TypeError> {
        match stmt {
            Stmt::Let { name, value, type_annotation, span, .. } => {

                let value_type = self.check_expression(value)?;

                if let Some(expected_type) = type_annotation {
                    if !self.types_are_compatible(expected_type, &value_type) {
                        return Err(TypeError::new(
                                span.clone(),
                            format!("Variable '{}' type mismatch: expected {}, found {}",
                                    name,
                                    self.type_to_string(expected_type),
                                    self.type_to_string(&value_type)
                                )
                            )
                        );
                    }
                }

                self.type_context.insert(name.clone(), value_type);
                Ok(())

            },
            _ => Ok(())
        }
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
                
                if !self.types_are_compatible(&left_type, &right_type) {
                    return Err(TypeError::new(
                            span.clone(),
                            format!("Operation '{:?}' between incompatible types: {} and {}",
                                op,
                                self.type_to_string(&left_type),
                                self.type_to_string(&right_type)
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
                                self.type_to_string(operand_type)
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
                            op, self.type_to_string(operand_type))
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
                            op, self.type_to_string(operand_type))
                    ))
                }
            }
            _ => Err(TypeError::new(
                span.clone(),
                format!("Operator '{:?}' not supported for type {}", 
                    op, self.type_to_string(operand_type))
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
                            self.type_to_string(operand_type))
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
                            self.type_to_string(operand_type))
                    ))
                }
            }
        }
    }

    fn get_value_type(&self, value: &Value) -> Type {
        match value {
            Value::Int(_) => Type::Primitive(PrimitiveType::Int),
            Value::Uint(_) => Type::Primitive(PrimitiveType::Uint),
            Value::Float(_) => Type::Primitive(PrimitiveType::Float),
            Value::Bool(_) => Type::Primitive(PrimitiveType::Bool),
            Value::Char(_) => Type::Primitive(PrimitiveType::Char),
            Value::String(_) => Type::Primitive(PrimitiveType::String),
            Value::Array(elements) => {
                if let Some(first) = elements.first() {
                    Type::Array(Box::new(self.get_value_type(first)))
                } else {
                    // For empty arrays, you might want a different approach
                    // Could use Type::Unknown or specific empty array type
                    Type::Array(Box::new(Type::Primitive(PrimitiveType::Int))) // Default fallback
                }
            },
            Value::Tuple(elements) => {
                let types = elements.iter()
                    .map(|elem| self.get_value_type(elem))
                    .collect();
                Type::Tuple(types)
            },
            Value::Function { address, captured_env } => {
                // Type::Function { params: (), return_type: () }
                todo!("Function type checking not implemented yet")
            },
            Value::Struct { struct_name, field_values } => {
                todo!("Struct type checking not implemented yet")
            }
            // Handle other value types
        }
    }

    fn types_are_compatible(&self, expected: &Type, actual: &Type) -> bool {
        match (expected, actual) {
            (Type::Primitive(a), Type::Primitive(b)) => a == b,
            (Type::Array(a), Type::Array(b)) => self.types_are_compatible(a, b),
            (Type::Tuple(a), Type::Tuple(b)) if a.len() == b.len() => {
                a.iter().zip(b.iter()).all(|(a_type, b_type)| {
                    self.types_are_compatible(a_type, b_type)
                })
            },
            (Type::Function { params: a_params, return_type: a_return }, 
             Type::Function { params: b_params, return_type: b_return }) 
                if a_params.len() == b_params.len() => 
            {
                let params_compatible = a_params.iter().zip(b_params.iter())
                    .all(|(a, b)| self.types_are_compatible(a, b));
                let return_compatible = self.types_are_compatible(a_return, b_return);
                params_compatible && return_compatible
            },
            _ => false
        }
    }

    fn type_to_string(&self, type_: &Type) -> String {
        match type_ {
            Type::Primitive(prim) => prim.to_string(),
            Type::Array(inner) => format!("[{}]", self.type_to_string(inner)),
            Type::Tuple(types) => {
                let types_str = types.iter()
                    .map(|t| self.type_to_string(t))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("({})", types_str)
            }
            Type::Function { params, return_type } => {
                let params_str = params.iter()
                    .map(|t| self.type_to_string(t))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("fn({}) -> {}", params_str, self.type_to_string(return_type))
            }
            Type::Struct(name) => name.clone()
        }
    }

    fn check_assignment(&mut self, expected_type: &Type, value_expr: &Expr, span: Span) -> Result<(), TypeError> {
        let actual_type = self.infer_expression_type(value_expr)?;
        if !self.types_are_compatible(expected_type, &actual_type) {
            return Err(TypeError::new(
                span,
                format!("Type mismatch: expected {}, found {}", 
                        self.type_to_string(expected_type),
                        self.type_to_string(&actual_type))
            ));
        }
        Ok(())
    }

    fn infer_expression_type(&mut self, expr: &Expr) -> Result<Type, TypeError> {
        // Walk the AST and infer types
        match expr {
            Expr::IntLiteral(_, _) => Ok(Type::Primitive(PrimitiveType::Int)),
            Expr::BoolLiteral(_, _) => Ok(Type::Primitive(PrimitiveType::Bool)),
            // ... other expression types
            _ => Err(TypeError::new(expr.span(), "Cannot infer type for expression"))
        }
    }

    // Helper functions

    fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    
    fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
    
    fn get_current_scope(&mut self) -> &mut HashMap<String, Type> {
        self.scopes.last_mut().unwrap()
    }
}