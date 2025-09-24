use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Bool,
    Int,
    Uint,
    Float,
    Char,
    String,
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrimitiveType::Bool => write!(f, "bool"),
            PrimitiveType::Int => write!(f, "int"),
            PrimitiveType::Uint => write!(f, "uint"),
            PrimitiveType::Float => write!(f, "float"),
            PrimitiveType::Char => write!(f, "char"),
            PrimitiveType::String => write!(f, "string"),
        }
    }
}

impl PrimitiveType {
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
                PrimitiveType::Int
                | PrimitiveType::Uint
                | PrimitiveType::Float
        )
    }
    
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            PrimitiveType::Int | PrimitiveType::Uint
        )
    }
    
    pub fn is_float(&self) -> bool {
        matches!(self, PrimitiveType::Float)
    }
    
    pub fn is_signed(&self) -> bool {
        matches!(self, PrimitiveType::Int | PrimitiveType::Float)
    }
}

// Type annotations used throughout the compiler
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Primitive(PrimitiveType),
    Array(Box<Type>),
    Tuple(Vec<Type>),
    Function {
        params: Vec<Type>,
        return_type: Box<Type>,
    },
    Struct(String)
}

#[derive(Debug, Clone)]
pub struct StructDefinition {
    pub name: String,
    pub fields: HashMap<String, Type>, // No Hash needed here
}

#[derive(Debug, Clone)]
pub struct TypeContext {
    pub structs: HashMap<String, StructDefinition>,
}

impl Type {
    pub fn is_numeric(&self) -> bool {
        match self {
            Type::Primitive(primitive) => matches!(primitive, PrimitiveType::Int | PrimitiveType::Float),
            Type::Array(element_type) => element_type.is_numeric(),
            Type::Tuple(types) => types.iter().all(|t| t.is_numeric()),
            Type::Function { .. } => false,
            Type::Struct(_) => false 
        }
    }

    pub fn is_comparable(&self) -> bool {
        match self {
            Type::Primitive(primitive) => matches!(
                primitive, 
                PrimitiveType::Int | PrimitiveType::Float | PrimitiveType::Bool | 
                PrimitiveType::Char | PrimitiveType::String
            ),
            Type::Array(element_type) => element_type.is_comparable(),
            Type::Tuple(types) => types.iter().all(|t| t.is_comparable()),
            Type::Function { .. } => false,
            Type::Struct(_) => false
        }
    }
}