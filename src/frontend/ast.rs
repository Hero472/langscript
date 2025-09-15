use crate::core::types::Type;

#[derive(Debug, Clone)]
pub enum Expr {

    // Literals
    IntLiteral(i64),
    FloatLiteral(f64),
    BoolLiteral(bool),
    StringLiteral(String),
    CharLiteral(char),
    Identifier(String),

    // Binary operations
    Binary {
        left: Box<Expr>,
        op: BinaryOp,
        right: Box<Expr>,
    },
    
    // Unary operations
    Unary {
        op: UnaryOp,
        expr: Box<Expr>,
    },
    
    // Function call
    Call {
        callee: Box<Expr>,
        args: Vec<Expr>,
    },
    
    // Member access (object.field)
    Member {
        object: Box<Expr>,
        member: String,
    },
    
    // Index access (array[index])
    Index {
        array: Box<Expr>,
        index: Box<Expr>,
    },
    
    // Parenthesized expression
    Grouped(Box<Expr>),
    
    // If expression (ternary)
    If {
        condition: Box<Expr>,
        then_branch: Box<Expr>,
        else_branch: Option<Box<Expr>>,
    },
    
    // Cast expression
    Cast {
        expr: Box<Expr>,
        target_type: Type,
    },
}

// Unary operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum UnaryOp {
    Negate,    // -
    Not,       // !
    BitNot,    // ~
}

// Binary operators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BinaryOp {
    Add,       // +
    Subtract,  // -
    Multiply,  // *
    Divide,    // /
    Modulo,    // %
    Equals,    // ==
    NotEquals, // !=
    LessThan,  // <
    LessEq,    // <=
    GreaterThan, // >
    GreaterEq, // >=
    And,       // &&
    Or,        // ||
    BitAnd,    // &
    BitOr,     // |
    BitXor,    // ^
    ShiftLeft, // <<
    ShiftRight, // >>
}

// Statement nodes
#[derive(Debug, Clone)]
pub enum Stmt {
    // Variable declaration: let x = 5;
    Let {
        name: String,
        value: Expr,
        type_annotation: Option<Type>,
    },

    // Expression statement: x + 5;
    Expr(Expr),
    
    // Block: { stmt1; stmt2; }
    Block(Vec<Stmt>),

    // While loop: while condition { ... }
    While {
        condition: Expr,
        body: Box<Stmt>,
    },
    
    // For loop: for i in 0..10 { ... }
    For {
        variable: String,
        iterable: Expr,
        body: Box<Stmt>,
    },

    // Function definition: fn name() { ... }
    Function {
        name: String,
        params: Vec<Param>,
        return_type: Option<Type>,
        body: Box<Stmt>,
    },

    // Return statement: return value;
    Return(Option<Expr>),

    // Match statement: match value { pattern => expr, ... }
    Match {
        expr: Expr,
        arms: Vec<MatchArm>,
    },
    
    // Break statement
    Break,
    
    // Continue statement
    Continue,
}

// Function parameter
#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub type_annotation: Type,
}

// Match arm
#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub body: Expr,
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
    Function(FunctionDecl),
    Struct(StructDecl),
    Enum(EnumDecl),
    Let(Stmt), // Global variable
    TypeAlias {
        name: String,
        underlying_type: Type,
    },
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
