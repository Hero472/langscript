use std::{collections::HashMap, rc::Rc};
use crate::generate_ast::LiteralValueAst;

pub struct Enviroment {
    values: HashMap<String, LiteralValueAst>,
    pub enclosing: Option<Rc<Enviroment>>,
}

impl Enviroment {
    
    pub fn new() -> Self {
        Self { 
            values: HashMap::new(),
            enclosing: None
        }
    }

    pub fn define(&mut self, name: String, value: LiteralValueAst) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<&LiteralValueAst> {
        let value: Option<&LiteralValueAst> = self.values.get(name);

        match (value, &self.enclosing) {
            (Some(val), _) => Some(val),
            (None, Some(env)) => env.get(name),
            (None, None) => None
        
        }

    }

    pub fn assign(&mut self, name: &str, value: LiteralValueAst) -> bool {
        
        let old_value: Option<&LiteralValueAst> = self.values.get(name);

        match (old_value, &mut self.enclosing) {
            (Some(_),_) => {
                self.values.insert(name.to_string(), value);
                true
            }
            (None, Some(env)) => Rc::get_mut(&mut env.clone()).expect("Could not get mutable reference to environment").assign(name, value),
            (None, None) => false
        }

    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_init() {
        let enviroment = Enviroment::new();
    }

}