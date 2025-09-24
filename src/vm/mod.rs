use crate::{error::RuntimeError, frontend::parser::ast::Program, vm::{interpreter::Interpreter, value::Value}};

pub mod interpreter;
pub mod value;
pub mod runtime_error;

pub struct VirtualMachine {
    pub interpreter: Interpreter,
}

impl VirtualMachine {
    pub fn new() -> Self {
        Self {
            interpreter: Interpreter::new(),
        }
    }

    pub fn interpret(&mut self, program: Program) -> Result<Value, RuntimeError> {
        self.interpreter.interpret(program)
    }
}