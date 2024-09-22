use std::collections::HashMap;
use crate::generate_ast::LiteralValueAst;

pub struct Enviroment {
    values: HashMap<String, LiteralValueAst>
}

impl Enviroment {
    
    pub fn new() -> Self {
        Self { values: HashMap::new() }
    }

    pub fn define(&mut self, name: String, value: LiteralValueAst) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&LiteralValueAst> {
        self.values.get(name)
    }

}