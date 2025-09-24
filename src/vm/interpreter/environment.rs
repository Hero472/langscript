use std::collections::HashMap;

use crate::{frontend::parser::ast::{EnumDecl, FunctionDecl, StructDecl}, vm::interpreter::Variable};

#[derive(Default)]
pub struct Environment {
    pub variables: HashMap<String, Variable>,
    pub functions: HashMap<String, FunctionDecl>,
    pub structs: HashMap<String, StructDecl>,
    pub enums: HashMap<String, EnumDecl>,
    pub parent: Option<Box<Environment>>, // For nested scopes
}

impl Environment {
    
    pub fn new() -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            structs: HashMap::new(),
            enums: HashMap::new(),
            parent: None,
        }
    }

    pub fn new_child(parent: Environment) -> Self {
        Self {
            variables: HashMap::new(),
            functions: HashMap::new(),
            structs: HashMap::new(),
            enums: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    // Variable operations
    pub fn define_variable(&mut self, name: String, variable: Variable) {
        self.variables.insert(name, variable);
    }

    pub fn get_variable(&self, name: &str) -> Option<&Variable> {
        self.variables.get(name).or_else(|| {
            self.parent.as_ref().and_then(|parent| parent.get_variable(name))
        })
    }

    // Function operations
    pub fn define_function(&mut self, name: String, function: FunctionDecl) {
        self.functions.insert(name, function);
    }

    pub fn get_function(&self, name: &str) -> Option<&FunctionDecl> {
        self.functions.get(name).or_else(|| {
            self.parent.as_ref().and_then(|parent| parent.get_function(name))
        })
    }

}