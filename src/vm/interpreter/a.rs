pub struct Interpreter {
    variables: HashMap<String, Value>
}

impl Interpreter {

    pub fn new() -> Self {
        Self {
            variables: HashMap::new()
        }
    }

    pub fn evaluate(&mut self, stmts: Vec<Stmt>) -> Result<Value, RuntimeError> {

        let mut last_value = Value::Bool(false);
        
        for stmt in stmts {
            last_value = self.evaluate_statement(stmt)?;

        }

        Ok(last_value)
    }

    fn evaluate_statement(&mut self, stmt: Stmt) -> Result<Value, RuntimeError> {

        match stmt {
            Stmt::Let { name, value, type_annotation, span } => {
                let evaluated_value = self.evaluate_expr(value)?;
                let actual_type = self.get_value_type(&evaluated_value);
                
                if let Some(expected_type) = type_annotation {

                    if !self.types_are_compatible(&expected_type, &actual_type) {
                        return Err(
                            RuntimeError::new(format!("Type error: variable '{}' declared as {} but assigned {} in line: {} column: {}",
                                    name,
                                    self.type_to_string(&expected_type),
                                    self.type_to_string(&actual_type),
                                    span.start_line,
                                    span.start_column
                                )
                            )
                        )
                    }

                }
                self.variables.insert(name.clone(), evaluated_value.clone());
                Ok(evaluated_value)
            },
            _ => Err(RuntimeError::new("Not yet implemented".to_string()))
        }
    }

    fn evaluate_expr(&mut self, expr: Expr) -> Result<Value, RuntimeError> {

        match expr {
            Expr::IntLiteral(n, _) => Ok(Value::Int(n)),
            Expr::UintLiteral(n, _) => Ok(Value::Uint(n)),
            Expr::StringLiteral(s, _) => Ok(Value::String(s)),
            Expr::BoolLiteral(b, _) => Ok(Value::Bool(b)),
            Expr::CharLiteral(c, _) => Ok(Value::Char(c)),
            Expr::FloatLiteral(f, _) => Ok(Value::Float(f)),
            Expr::Unary { op, expr, span } => {
                let value = self.evaluate_expr(*expr)?;
                self.evaluate_unary_op(op, value, span)
            },
            Expr::Binary { left, op, right, span } => {
                let left_val = self.evaluate_expr(*left)?;
                let right_val = self.evaluate_expr(*right)?;
                self.evaluate_binary_op(left_val, op, right_val, span)
            },
            Expr::Grouped(expr, _) => {
                self.evaluate_expr(*expr)
            },
            _ => Err(RuntimeError::new(format!("Expression type not implemented: {:?}", expr))),
        }
    }

    fn evaluate_negate(&mut self, value: Value) -> Result<Value, String> {

        match value {
            Value::Int(n) => Ok(Value::Int(-n)),
            Value::Float(f) => Ok(Value::Float(-f)),
            _ => {
                let primitive_type = value.get_primitive_type();

                if let Some(t) = primitive_type {
                    return Err(format!("Cannot apply unary operator '-' to type {}", t))
                } else {
                    return Err("Cannot apply unary operator '-' to non primitives types".to_string())
                }
                    
            }
        }

    }

    fn evaluate_not(&mut self, value: Value) -> Result<Value, String> {

        match value {
            Value::Bool(b) => Ok(Value::Bool(!b)),
            _ => {
                let primitive_type = value.get_primitive_type();

                if let Some(t) = primitive_type {
                    return Err(format!("Cannot apply logical NOT to type {}", t))
                } else {
                    return Err("Cannot apply logical NOT non primitives types".to_string())
                }
                    
            }
        }

    }

    fn evaluate_add(&mut self, left: Value, right: Value) -> Result<Value, String> {
        
        match (left, right) {
            (Value::Int(n1), Value::Int(n2)) => {
                n1.checked_add(n2)
                    .map(Value::Int)
                    .ok_or_else(|| "Integer overflow in addition".to_string())
            },
            (Value::Uint(n1), Value::Uint(n2)) => {
                n1.checked_add(n2)
                    .map(Value::Uint)
                    .ok_or_else(|| "Unsigned integer overflow in addition".to_string())
            },
            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Float(f1 + f2)),
            (Value::String(s1), Value::String(s2)) => Ok(Value::String(format!("{}{}", s1, s2))),
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot add {} and {}", left_desc, right_desc))
            }
        }

    }

    fn evaluate_subtract(&mut self, left: Value, right: Value) -> Result<Value, String> {
        
        match (left, right) {
            (Value::Int(n1), Value::Int(n2)) => {
                n1.checked_sub(n2)
                    .map(Value::Int)
                    .ok_or_else(|| "Integer overflow in subtraction".to_string())
            },
            (Value::Uint(n1), Value::Uint(n2)) => {
                n1.checked_sub(n2)
                    .map(Value::Uint)
                    .ok_or_else(|| "Unsigned integer underflow in subtraction".to_string())
            },
            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Float(f1 - f2)),
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot subtract {} from {}", right_desc, left_desc))
            }
        }

    }

    fn evaluate_multiply(&mut self, left: Value, right: Value) -> Result<Value, String> {
        
        match (left, right) {
            (Value::Int(n1), Value::Int(n2)) => {
                n1.checked_mul(n2)
                    .map(Value::Int)
                    .ok_or_else(|| "Integer overflow in multiplication".to_string())
            },
            (Value::Uint(n1), Value::Uint(n2)) => {
                n1.checked_mul(n2)
                    .map(Value::Uint)
                    .ok_or_else(|| "Unsigned integer overflow in multiplication".to_string())
            },
            (Value::Float(f1), Value::Float(f2)) => Ok(Value::Float(f1 * f2)),
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot multiply {} and {}", left_desc, right_desc))
            }
        }

    }

    fn evaluate_divide(&mut self, left: Value, right: Value) -> Result<Value, String> {
        
        match (left, right) {
            (Value::Int(n1), Value::Int(n2)) => {
                if n2 == 0 {
                    Err("Division by zero".to_string())
                } else {
                    // Check for edge case: i64::MIN / -1 which overflows
                    if n1 == i64::MIN && n2 == -1 {
                        Err("Integer overflow in division".to_string())
                    } else {
                        Ok(Value::Int(n1 / n2))
                    }
                }
            },
            (Value::Uint(n1), Value::Uint(n2)) => {
                if n2 == 0 {
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Uint(n1 / n2))
                }
            },
            (Value::Float(f1), Value::Float(f2)) => {
                if f2 == 0.0 {
                    // Float division by zero produces infinity, but you might want to error
                    Err("Division by zero".to_string())
                } else {
                    Ok(Value::Float(f1 / f2))
                }
            },
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot divide {} by {}", left_desc, right_desc))
            }
        }
        
    }

    fn evaluate_modulo(&mut self, left: Value, right: Value) -> Result<Value, String> {
        
        match (left, right) {
            (Value::Int(n1), Value::Int(n2)) => {
                if n2 == 0 {
                    Err("Modulo by zero".to_string())
                } else {
                    Ok(Value::Int(n1 % n2))
                }
            },
            (Value::Uint(n1), Value::Uint(n2)) => {
                if n2 == 0 {
                    Err("Modulo by zero".to_string())
                } else {
                    Ok(Value::Uint(n1 % n2))
                }
            },
            (Value::Float(f1), Value::Float(f2)) => {
                if f2 == 0.0 {
                    Err("Modulo by zero".to_string())
                } else {
                    Ok(Value::Float(f1 % f2))
                }
            },
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot compute modulo of {} and {}", left_desc, right_desc))
            }
        }

    }

    fn evaluate_equals(&mut self, left: Value, right: Value) -> Result<Value, String> {
        
        match (left, right) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a == b)),
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a == b)),
            (Value::Uint(a), Value::Uint(b)) => Ok(Value::Bool(a == b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a == b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a == b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a == b)),
            (Value::Int(a), Value::Uint(b)) => Ok(Value::Bool(a == b as i64)),
            (Value::Uint(a), Value::Int(b)) => Ok(Value::Bool(a == b as u64)),
            (Value::Array(a), Value::Array(b)) => Ok(Value::Bool(a == b)),
            (Value::Tuple(a), Value::Tuple(b)) => Ok(Value::Bool(a == b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 == b as f64)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a as f64 == b as f64)),
            (Value::Uint(a), Value::Float(b)) => Ok(Value::Bool(a as f64 == b as f64)),
            (Value::Float(a), Value::Uint(b)) => Ok(Value::Bool(a as f64 == b as f64)),
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot compare of {} and {} for equality", left_desc, right_desc))
            }
        }

    }

    fn evaluate_not_equals(&mut self, left: Value, right: Value) -> Result<Value, String> {
        
        match (left, right) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a != b)),
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a != b)),
            (Value::Uint(a), Value::Uint(b)) => Ok(Value::Bool(a != b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a != b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a != b)),
            (Value::String(a), Value::String(b)) => Ok(Value::Bool(a != b)),
            (Value::Int(a), Value::Uint(b)) => Ok(Value::Bool(a != b as i64)),
            (Value::Uint(a), Value::Int(b)) => Ok(Value::Bool(a != b as u64)),
            (Value::Array(a), Value::Array(b)) => Ok(Value::Bool(a != b)),
            (Value::Tuple(a), Value::Tuple(b)) => Ok(Value::Bool(a != b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool(a as f64 != b as f64)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a as f64 != b as f64)),
            (Value::Uint(a), Value::Float(b)) => Ok(Value::Bool(a as f64 != b as f64)),
            (Value::Float(a), Value::Uint(b)) => Ok(Value::Bool(a as f64 != b as f64)),
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot compare of {} and {} for equality", left_desc, right_desc))
            }
        }

    }

    fn evaluate_less_than(&mut self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
            (Value::Uint(a), Value::Uint(b)) => Ok(Value::Bool(a < b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a < b)),
            
            // Cross-type comparisons
            (Value::Int(a), Value::Uint(b)) => Ok(Value::Bool(a < b as i64)),
            (Value::Uint(a), Value::Int(b)) => Ok(Value::Bool((a as i64) < b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) < b as f64)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool((a as f64) < b as f64)),
            (Value::Uint(a), Value::Float(b)) => Ok(Value::Bool((a as f64) < b as f64)),
            (Value::Float(a), Value::Uint(b)) => Ok(Value::Bool((a as f64) < b as f64)),
            
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot compare {} and {} with less than", left_desc, right_desc))
            }
        }
    }

    fn evaluate_less_eq(&mut self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
            (Value::Uint(a), Value::Uint(b)) => Ok(Value::Bool(a <= b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a <= b)),
            
            // Cross-type comparisons
            (Value::Int(a), Value::Uint(b)) => Ok(Value::Bool(a <= b as i64)),
            (Value::Uint(a), Value::Int(b)) => Ok(Value::Bool((a as i64) <= b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) <= b as f64)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a as f64 <= b as f64)),
            (Value::Uint(a), Value::Float(b)) => Ok(Value::Bool(a as f64 <= b as f64)),
            (Value::Float(a), Value::Uint(b)) => Ok(Value::Bool(a as f64 <= b as f64)),
            
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot compare {} and {} with less than or equal", left_desc, right_desc))
            }
        }
    }

    fn evaluate_greater_than(&mut self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
            (Value::Uint(a), Value::Uint(b)) => Ok(Value::Bool(a > b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a > b)),
            
            // Cross-type comparisons
            (Value::Int(a), Value::Uint(b)) => Ok(Value::Bool(a > b as i64)),
            (Value::Uint(a), Value::Int(b)) => Ok(Value::Bool((a as i64) > b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) > b as f64)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a as f64 > b as f64)),
            (Value::Uint(a), Value::Float(b)) => Ok(Value::Bool(a as f64 > b as f64)),
            (Value::Float(a), Value::Uint(b)) => Ok(Value::Bool(a as f64 > b as f64)),
            
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot compare {} and {} with greater than", left_desc, right_desc))
            }
        }
    }

    fn evaluate_greater_eq(&mut self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
            (Value::Uint(a), Value::Uint(b)) => Ok(Value::Bool(a >= b)),
            (Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
            (Value::Char(a), Value::Char(b)) => Ok(Value::Bool(a >= b)),
            
            // Cross-type comparisons
            (Value::Int(a), Value::Uint(b)) => Ok(Value::Bool(a >= b as i64)),
            (Value::Uint(a), Value::Int(b)) => Ok(Value::Bool((a as i64) >= b)),
            (Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) >= b as f64)),
            (Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a as f64 >= b as f64)),
            (Value::Uint(a), Value::Float(b)) => Ok(Value::Bool(a as f64 >= b as f64)),
            (Value::Float(a), Value::Uint(b)) => Ok(Value::Bool(a as f64 >= b as f64)),
            
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Cannot compare {} and {} with greater than or equal", left_desc, right_desc))
            }
        }
    }

    fn evaluate_and(&mut self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Logical AND requires boolean operands, got {} and {}", left_desc, right_desc))
            }
        }
    }

    fn evaluate_or(&mut self, left: Value, right: Value) -> Result<Value, String> {
        match (left, right) {
            (Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),
            (left, right) => {
                let left_desc = left.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                let right_desc = right.get_primitive_type()
                    .map(|t| t.to_string())
                    .unwrap_or_else(|| "non-primitive".to_string());
                
                Err(format!("Logical OR requires boolean operands, got {} and {}", left_desc, right_desc))
            }
        }
    }

    fn evaluate_binary_op(
        &mut self,
        left: Value,
        op: BinaryOp,
        right: Value,
        span: Span
    ) -> Result<Value, RuntimeError> {
        let result = match op {
            BinaryOp::Add => self.evaluate_add(left, right),
            BinaryOp::Subtract => self.evaluate_subtract(left, right),
            BinaryOp::Multiply => self.evaluate_multiply(left, right),
            BinaryOp::Divide => self.evaluate_divide(left, right),
            BinaryOp::Modulo => self.evaluate_modulo(left, right),
            BinaryOp::Equals => self.evaluate_equals(left, right),
            BinaryOp::NotEquals => self.evaluate_not_equals(left, right),
            BinaryOp::LessThan => self.evaluate_less_than(left, right),
            BinaryOp::LessEq => self.evaluate_less_eq(left, right),
            BinaryOp::GreaterThan => self.evaluate_greater_than(left, right),
            BinaryOp::GreaterEq => self.evaluate_greater_eq(left, right),
            BinaryOp::And => self.evaluate_and(left, right),
            BinaryOp::Or => self.evaluate_or(left, right),
        };
        
        result.map_err(|e| self.wrap_error(e, span))
    }

    // Helper for unary operations
    fn evaluate_unary_op(
        &mut self,
        op: UnaryOp,
        value: Value,
        span: Span
    ) -> Result<Value, RuntimeError> {
        let result = match op {
            UnaryOp::Negate => self.evaluate_negate(value),
            UnaryOp::Not => self.evaluate_not(value),
        };
        
        result.map_err(|e| self.wrap_error(e, span))
    }

    fn wrap_error<E: Into<String>>(&self, error: E, span: Span) -> RuntimeError {
        RuntimeError::new(error.into()).with_span(span)
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
                    // Empty array - you might want to handle this differently
                    Type::Array(Box::new(Type::Primitive(PrimitiveType::Int))) // Default type
                }
            },
            Value::Tuple(tuple) => {
                let mut values = vec![];

                for t in tuple {
                    let typed = self.get_value_type(t);
                    values.push(typed);
                };
                Type::Tuple(values)
            }
            _ => Type::Primitive(PrimitiveType::Bool) // not yet implemented
            // Add other value types as needed
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
                // Check all parameter types are compatible
                let params_compatible = a_params.iter().zip(b_params.iter())
                    .all(|(a, b)| self.types_are_compatible(a, b));
                
                // Check return types are compatible
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
        }
    }
}