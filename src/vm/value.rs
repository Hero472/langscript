use crate::core::types::{PrimitiveType, Type};

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Unit,
    Bool(bool),
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
    },
    Struct {
        struct_name: String,
        field_values: Vec<Value>
    }
}

impl Value {
    pub fn get_primitive_type(&self) -> Option<PrimitiveType> {
        match self {
            Value::Bool(_) => Some(PrimitiveType::Bool),
            Value::Int(_) => Some(PrimitiveType::Int),
            Value::Uint(_) => Some(PrimitiveType::Uint),
            Value::Float(_) => Some(PrimitiveType::Float),
            Value::Char(_) => Some(PrimitiveType::Char),
            Value::String(_) => Some(PrimitiveType::String),
            _ => None, // Composite types don't have simple primitive types
        }
    }

    pub fn get_type(&self) -> Option<Type> {
        match self {
            Value::Unit => Some(Type::Primitive(PrimitiveType::Unit)),
            Value::Bool(_) => Some(Type::Primitive(PrimitiveType::Bool)),
            Value::Int(_) => Some(Type::Primitive(PrimitiveType::Int)),
            Value::Uint(_) => Some(Type::Primitive(PrimitiveType::Uint)),
            Value::Float(_) => Some(Type::Primitive(PrimitiveType::Float)),
            Value::Char(_) => Some(Type::Primitive(PrimitiveType::Char)),
            Value::String(_) => Some(Type::Primitive(PrimitiveType::String)),
            Value::Array(values) => {
                if let Some(first) = values.first() {
                    // Infer type from first element (homogeneous arrays)
                    first.get_type().map(|element_type| Type::Array(Box::new(element_type)))
                } else {
                    // Empty array - can't infer element type
                    None
                }
            },
            Value::Tuple(values) => {
                let types: Vec<Type> = values.iter()
                    .filter_map(|v| v.get_type())
                    .collect();
                Some(Type::Tuple(types))
            },
            Value::Function { address: _, captured_env: _ } => {
                // Functions need more context - return None or a generic function type
                None
                // Or if you want to be more specific:
                // Some(Type::Function {
                //     params: vec![], // Unknown parameters
                //     return_type: Box::new(Type::Primitive(PrimitiveType::Bool)), // Unknown return
                // })
            },
            Value::Struct { struct_name, field_values: _ } => {
                Some(Type::Struct(struct_name.clone()))
            }
        }
    }
}