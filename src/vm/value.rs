use crate::core::types::{PrimitiveType, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Bool(bool),
    Byte(u8),
    Int(i64),
    Uint(u64),
    Float(f64),
    Char(char),
    String(String),
    Array(Vec<Value>),
    Tuple(Vec<Value>),
    Function {
        address: usize,
        captured_env: Vec<Value>, // for closures
    }
}

impl Value {
    pub fn get_primitive_type(&self) -> Option<PrimitiveType> {
        match self {
            Value::Bool(_) => Some(PrimitiveType::Bool),
            Value::Byte(_) => Some(PrimitiveType::Byte),
            Value::Int(_) => Some(PrimitiveType::Int),
            Value::Uint(_) => Some(PrimitiveType::Uint),
            Value::Float(_) => Some(PrimitiveType::Float),
            Value::Char(_) => Some(PrimitiveType::Char),
            Value::String(_) => Some(PrimitiveType::String),
            _ => None, // Composite types don't have simple primitive types
        }
    }
}