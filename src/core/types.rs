use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimitiveType {
    Bool,
    Byte,
    Int,
    Uint,
    Float,
    Double,
    Char,
    String,
}

impl fmt::Display for PrimitiveType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PrimitiveType::Bool => write!(f, "bool"),
            PrimitiveType::Byte => write!(f, "byte"),
            PrimitiveType::Int => write!(f, "int"),
            PrimitiveType::Uint => write!(f, "uint"),
            PrimitiveType::Float => write!(f, "float"),
            PrimitiveType::Double => write!(f, "double"),
            PrimitiveType::Char => write!(f, "char"),
            PrimitiveType::String => write!(f, "string"),
        }
    }
}

impl PrimitiveType {
    pub fn is_numeric(&self) -> bool {
        matches!(
            self,
            PrimitiveType::Byte
                | PrimitiveType::Int
                | PrimitiveType::Uint
                | PrimitiveType::Float
                | PrimitiveType::Double
        )
    }
    
    pub fn is_integer(&self) -> bool {
        matches!(
            self,
            PrimitiveType::Byte | PrimitiveType::Int | PrimitiveType::Uint
        )
    }
    
    pub fn is_float(&self) -> bool {
        matches!(self, PrimitiveType::Float | PrimitiveType::Double)
    }
    
    pub fn is_signed(&self) -> bool {
        matches!(self, PrimitiveType::Int | PrimitiveType::Float | PrimitiveType::Double)
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
    // Add type variables for generics later if needed
    // TypeVar(String),
}