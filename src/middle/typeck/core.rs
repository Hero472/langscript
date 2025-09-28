use std::collections::HashMap;

use crate::{core::types::Type, error::TypeError, frontend::parser::ast::{Declaration, Program}, middle::typeck::{declarations::DeclarationChecker, statements::StatementChecker}};

pub struct TypeChecker {
    pub type_context: HashMap<String, Type>,
    pub scopes: Vec<HashMap<String, Type>>,
    pub current_return_type: Option<Type>,
    pub declaration_checker: DeclarationChecker
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            type_context: HashMap::new().into(),
            scopes: vec![HashMap::new()].into(),
            current_return_type: None,
            declaration_checker: DeclarationChecker::new()
        }
    }

    pub fn check(&mut self, program: &Program) -> Result<(), Vec<TypeError>> {
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
            Declaration::Function(function_decl, span) => {
                DeclarationChecker::check_function(self, &function_decl, &span)
            },
            Declaration::Struct(struct_decl, span) => {
                // TODO: Implement struct checking
                Ok(())
            },
            Declaration::Enum(enum_decl, span) => {
                // TODO: Implement enum checking
                Ok(())
            },
        }
    }

    // Scope management
    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }
    
    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }
    
    pub fn get_current_scope(&mut self) -> &mut HashMap<String, Type> {
        self.scopes.last_mut().unwrap()
    }

    // Context accessors
    pub fn is_function_declared(&self, name: &str) -> bool {
        self.type_context.contains_key(name)
    }

    pub fn type_context(&self) -> &HashMap<String, Type> {
        &self.type_context
    }
    
    pub fn type_context_mut(&mut self) -> &mut HashMap<String, Type> {
        &mut self.type_context
    }
    
    pub fn current_return_type(&self) -> Option<&Type> {
        self.current_return_type.as_ref()
    }
    
    pub fn set_current_return_type(&mut self, return_type: Option<Type>) {
        self.current_return_type = return_type;
    }
}