use crate::{core::types::{PrimitiveType, Type}, vm::value::Value};

pub fn get_value_type(value: &Value) -> Type {
    match value {
        Value::Unit => Type::Primitive(PrimitiveType::Unit),
        Value::Int(_) => Type::Primitive(PrimitiveType::Int),
        Value::Uint(_) => Type::Primitive(PrimitiveType::Uint),
        Value::Float(_) => Type::Primitive(PrimitiveType::Float),
        Value::Bool(_) => Type::Primitive(PrimitiveType::Bool),
        Value::Char(_) => Type::Primitive(PrimitiveType::Char),
        Value::String(_) => Type::Primitive(PrimitiveType::String),
        Value::Array(elements) => {
            if let Some(first) = elements.first() {
                Type::Array(Box::new(get_value_type(first)))
            } else {
                // For empty arrays, you might want a different approach
                // Could use Type::Unknown or specific empty array type
                Type::Array(Box::new(Type::Primitive(PrimitiveType::Int))) // Default fallback
            }
        },
        Value::Tuple(elements) => {
            let types = elements.iter()
                .map(|elem| get_value_type(elem))
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

pub fn optional_types_are_compatible(a: Option<Type>, b: Option<Type>) -> bool {
    match (a, b) {
        (Some(a_type), Some(b_type)) => types_are_compatible(&a_type, &b_type),
        (None, None) => true,
        _ => false,
    }
}

pub fn types_are_compatible(expected: &Type, actual: &Type) -> bool {
    match (expected, actual) {
        (Type::Primitive(a), Type::Primitive(b)) => a == b,
        (Type::Array(a), Type::Array(b)) => types_are_compatible(a, b),
        (Type::Tuple(a), Type::Tuple(b)) if a.len() == b.len() => {
            a.iter().zip(b.iter()).all(|(a_type, b_type)| {
                types_are_compatible(a_type, b_type)
            })
        },
        (Type::Function { params: a_params, return_type: a_return }, 
            Type::Function { params: b_params, return_type: b_return }) 
            
                if a_params.len() == b_params.len() => {
                    let params_compatible = a_params.iter().zip(b_params.iter())
                        .all(|(a, b)| types_are_compatible(a, b));

                    let return_compatible = optional_types_are_compatible(*a_return.clone(), *b_return.clone());

                    params_compatible && return_compatible
                },
        _ => false
    }
}

pub fn type_to_string(type_: &Type) -> String {
    match type_ {
        Type::Primitive(prim) => prim.to_string(),
        Type::Array(inner) => format!("[{}]", type_to_string(inner)),
        Type::Tuple(types) => {
            let types_str = types.iter()
                .map(|t| type_to_string(t))
                .collect::<Vec<_>>()
                .join(", ");
            format!("({})", types_str)
        }
        Type::Function { params, return_type } => {

            let returned_type = if let Some(return_type) = *return_type.clone() {
                type_to_string(&return_type)
            } else {
                "None".to_string()
            };
            
            let params_str = params.iter()
                .map(|t| type_to_string(t))
                .collect::<Vec<_>>()
                .join(", ");
            format!("fn({}) -> {}", params_str, returned_type)
        }
        Type::Struct(name) => name.clone()
    }
}