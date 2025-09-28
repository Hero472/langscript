use crate::{core::types::Type, frontend::lexer::span::Span, vm::value::Value};

#[derive(Debug, Clone)]
pub enum Expr {

    // Literals
    IntLiteral(i64, Span),
    UintLiteral(u64, Span),
    FloatLiteral(f64, Span),
    BoolLiteral(bool, Span),
    StringLiteral(String, Span),
    CharLiteral(char, Span),
    Identifier(String, Span),

    // Binary operations
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
        span: Span,
    },
    
    // Unary operations
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
        span: Span,
    },
    
    // Function call
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
        span: Span,
    },
    
    // Member access (object.field)
    Member {
        object: Box<Expr>,
        member: String,
        span: Span,
    },
    
    // Index access (array[index])
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
        span: Span,
    },
    
    // Parenthesized expression
    Grouped(Box<Expr>, Span,),
    
    // If expression (ternary)
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
        span: Span,
    },
    
    // Cast expression
    Cast {
        expr: Box<Expr>,
        target_type: Type,
        span: Span,
    },
}

impl Expr {

    pub fn get_value(&self) -> Option<Value> {
        match self {
            Expr::IntLiteral(n, _) => Some(Value::Int(*n)),
            Expr::UintLiteral(n, _) => Some(Value::Uint(*n)),
            Expr::FloatLiteral(n, _) => Some(Value::Float(*n)),
            Expr::BoolLiteral(b, _) => Some(Value::Bool(*b)),
            Expr::StringLiteral(s, _) => Some(Value::String(s.clone())),
            Expr::CharLiteral(c, _) => Some(Value::Char(*c)),
            
            // These expressions don't have immediate literal values
            Expr::Identifier(_, _) => None,
            Expr::Binary { .. } => None,
            Expr::Unary { .. } => None,
            Expr::Call { .. } => None,
            Expr::Member { .. } => None,
            Expr::Index { .. } => None,
            Expr::Grouped(expr, _) => expr.get_value(), // Recursively check grouped expression
            Expr::If { .. } => None,
            Expr::Cast { expr, .. } => expr.get_value(), // Check the expression being cast
        }
    }

    pub fn span(&self) -> Span {
        match self {
            Expr::IntLiteral(_, span)
            | Expr::UintLiteral(_, span)
            | Expr::FloatLiteral(_, span)
            | Expr::BoolLiteral(_, span)
            | Expr::StringLiteral(_, span)
            | Expr::CharLiteral(_, span)
            | Expr::Identifier(_, span)
            | Expr::Grouped(_, span) => span.clone(),
            
            Expr::Binary { span, .. }
            | Expr::Unary { span, .. }
            | Expr::Call { span, .. }
            | Expr::Member { span, .. }
            | Expr::Index { span, .. }
            | Expr::If { span, .. }
            | Expr::Cast { span, .. } => span.clone(),
        }
    }
}

// Unary operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Negate,    // -
    Not,       // !
}

// Binary operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    // Arithmetic
    Add,       // +
    Subtract,  // -
    Multiply,  // *
    Divide,    // /
    Modulo,    // %

    // Comparison
    Equals,    // ==
    NotEquals, // !=
    LessThan,  // <
    LessEq,    // <=
    GreaterThan, // >
    GreaterEq, // >=

    // Logical
    And,       // &&
    Or,        // ||
}

// Statement nodes
#[derive(Debug, Clone)]
pub enum Stmt {
    // Variable declaration: let x = 5;
    Let {
        name: String,
        value: Expr,
        type_annotation: Option<Type>,
        span: Span,
        mutable: bool
    },

    // If statement
    If {
        condition: Expr,
        then_branch: Box<Stmt>,
        else_branch: Option<Box<Stmt>>,
        span: Span
    },

    // Expression statement: x + 5;
    Expr(Expr, Span),
    
    // Block: { stmt1; stmt2; }
    Block(Vec<Stmt>, Span),

    // While loop: while condition { ... }
    While {
        condition: Expr,
        body: Box<Stmt>,
        span: Span,
    },
    
    // For loop: for i in 0..10 { ... }
    For {
        variable: String,
        iterable: Expr,
        body: Box<Stmt>,
        span: Span,
    },

    // Function definition: fn name() { ... }
    Function {
        name: String,
        params: Vec<Param>,
        return_type: Option<Type>,
        body: Box<Stmt>,
        span: Span,
    },

    // Return statement: return value;
    Return(Option<Expr>, Span),

    // Match statement: match value { pattern => expr, ... }
    Match {
        expr: Expr,
        arms: Vec<MatchArm>,
        span: Span,
    },
    
    // Break statement
    Break(Span),
    
    // Continue statement
    Continue(Span),
}

impl Stmt {
    pub fn span(&self) -> Span {
        match self {
            Stmt::Let { span, .. }
            | Stmt::If { span, .. } 
            | Stmt::Expr(_, span)
            | Stmt::Block(_, span)
            | Stmt::While { span, .. }
            | Stmt::For { span, .. }
            | Stmt::Function { span, .. }
            | Stmt::Return(_, span)
            | Stmt::Match { span, .. }
            | Stmt::Break(span)
            | Stmt::Continue(span) => span.clone(),
        }
    }
}

// Function parameter
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub type_annotation: Type
}

// Match arm
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Expr
}

// Patterns for match statements
#[derive(Debug, Clone)]
pub enum Pattern {
    Literal(Expr),
    Identifier(String),
    Wildcard, // _
    Tuple(Vec<Pattern>),
    Struct {
        name: String,
        fields: Vec<FieldPattern>,
    },
}

// Field pattern for struct destructuring
#[derive(Debug, Clone)]
pub struct FieldPattern {
    pub name: String,
    pub pattern: Pattern,
}

// Top-level declarations
#[derive(Debug, Clone)]
pub enum Declaration {
    Function(FunctionDecl, Span),    // Function declaration
    Struct(StructDecl, Span),        // Struct declaration  
    Enum(EnumDecl, Span),            // Enum declaration
}

#[derive(Debug, Clone)]
pub struct LetDecl {
    pub name: String,
    pub value: Expr,
    pub type_annotation: Type,
    pub mutable: bool,
    pub is_exported: bool,
}

// Function declaration
#[derive(Debug, Clone)]
pub struct FunctionDecl {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Stmt,
}

// Struct declaration
#[derive(Debug, Clone)]
pub struct StructDecl {
    pub name: String,
    pub fields: Vec<FieldDecl>,
}

// Struct field declaration
#[derive(Debug, Clone)]
pub struct FieldDecl {
    pub name: String,
    pub type_annotation: Type,
}

// Enum declaration
#[derive(Debug, Clone)]
pub struct EnumDecl {
    pub name: String,
    pub variants: Vec<VariantDecl>,
}

// Enum variant declaration
#[derive(Debug, Clone)]
pub struct VariantDecl {
    pub name: String,
    pub data: Option<Type>, // Optional associated data
}

// Complete program
#[derive(Debug, Clone)]
pub struct Program {
    pub declarations: Vec<Declaration>,
}
