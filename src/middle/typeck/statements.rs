use crate::{error::TypeError, frontend::parser::ast::Stmt, middle::typeck::utils::{type_to_string, types_are_compatible}};

pub struct StatementChecker;

impl StatementChecker {
    pub fn new() -> Self {
        Self
    }

    fn check_statement(&mut self, stmt: &Stmt) -> Result<(), TypeError> {
        match stmt {
            Stmt::Let { name, value, type_annotation, span, .. } => {

                let value_type = self.check_expression(value)?;

                if let Some(expected_type) = type_annotation {
                    if !types_are_compatible(expected_type, &value_type) {
                        return Err(TypeError::new(
                                span.clone(),
                            format!("Variable '{}' type mismatch: expected {}, found {}",
                                    name,
                                    type_to_string(expected_type),
                                    type_to_string(&value_type)
                                )
                            )
                        );
                    }
                }

                self.type_context.insert(name.clone(), value_type);
                Ok(())

            },
            _ => Ok(())
        }
    }
}