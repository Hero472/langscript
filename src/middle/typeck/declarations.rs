use crate::{core::types::Type, error::TypeError, frontend::{lexer::span::Span, parser::ast::FunctionDecl}, middle::typeck::{core::TypeChecker, statements::StatementChecker}};

pub struct DeclarationChecker {
    statement_checker: StatementChecker
}

impl DeclarationChecker {
    pub fn new() -> Self {
        Self {
            statement_checker: StatementChecker::new()
        }
    }

    pub fn check_function(
        type_checker: &mut TypeChecker, 
        function_decl: &FunctionDecl, 
        span: &Span
    ) -> Result<(), TypeError> {
        Self::register_function_signature(type_checker, function_decl, span)?;
        Self::check_function_body(type_checker, function_decl, span)
    }

    fn register_function_signature(
        type_checker: &mut TypeChecker,
        function_decl: &FunctionDecl,
        span: &Span,
    ) -> Result<(), TypeError> {
        let function_name = &function_decl.name;

        if type_checker.type_context().contains_key(function_name) {
            return Err(TypeError::new(
                span.clone(),
                format!("Function '{}' is already declared", function_name),
            ));
        }

        let param_types = function_decl.params.iter()
            .map(|param| param.type_annotation.clone())
            .collect();

        let function_type = Type::Function {
            params: param_types,
            return_type: Box::new(function_decl.return_type.clone()),
        };

        type_checker.type_context_mut().insert(function_name.clone(), function_type);
        Ok(())
    }

    fn check_function_body(
        &self,
        type_checker: &mut TypeChecker,
        function_decl: &FunctionDecl,
        _span: &Span,
    ) -> Result<(), TypeError> {
        // Set up function scope
        type_checker.enter_scope();
        type_checker.set_current_return_type(function_decl.return_type.clone());

        // Register parameters in the new scope
        for param in &function_decl.params {
            type_checker.get_current_scope().insert(
                param.name.clone(), 
                param.type_annotation.clone()
            );
        }

        // Check function body
        let result = self.statement_checker.check_block(
            type_checker, 
            &function_decl.body
        );

        // Clean up
        type_checker.exit_scope();
        type_checker.set_current_return_type(None);

        Ok(())
    }

}