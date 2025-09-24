use crate::{error::RuntimeError, frontend::parser::ast::Declaration, vm::{interpreter::{eval_expressions::evaluate_expr, Interpreter, Variable}, value::Value}};



pub fn evaluate_declaration(interpreter: &mut Interpreter, decl: Declaration) -> Result<(), RuntimeError> {

    match decl {
        Declaration::Function(function_decl, span) => {
            // Store the function in the environment for later calling
            interpreter.environment.functions.insert(
                function_decl.name.clone(), 
                function_decl
            );
            // Function declarations don't produce a value
            Ok(())
        },
        Declaration::Struct(struct_decl, span) => {
            // Store the struct definition for type checking and creation
            interpreter.environment.structs.insert(
                struct_decl.name.clone(), 
                struct_decl
            );
            // Struct declarations don't produce a value
            Ok(())
        },
        Declaration::Enum(enum_decl, span) => {
            // Store the enum definition for type checking and matching
            interpreter.environment.enums.insert(
                enum_decl.name.clone(), 
                enum_decl
            );
            // Enum declarations don't produce a value
            Ok(())
        },
    }

}