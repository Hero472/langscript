use crate::generate_ast::LiteralValueAst;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct Environment {
    globals: Rc<HashMap<String, LiteralValueAst>>,
    values: HashMap<String, LiteralValueAst>,
    pub enclosing: Option<Rc<RefCell<Environment>>>,
}

fn clock_impl(_env: Rc<RefCell<Environment>>, _args: &Vec<LiteralValueAst>) -> LiteralValueAst {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::SystemTime::UNIX_EPOCH)
        .expect("Could not get system time")
        .as_millis();

    LiteralValueAst::Number(now as f64 / 1000.0)
}

fn get_globals() -> HashMap<String, LiteralValueAst> {
    let mut env: HashMap<String, LiteralValueAst> = HashMap::new();
    env.insert(
        "clock".to_string(),
        LiteralValueAst::Callable {
            name: "clock".to_string(),
            arity: 0,
            fun: Rc::new(clock_impl)
        },
    );
    env
}


impl Environment {
    pub fn new() -> Self {
        Self {
            globals: Rc::new(get_globals()),
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn define_top_level(&mut self, name: String, value: LiteralValueAst) {

        match &self.enclosing {
            None => self.define(name, value),
            Some(env) => env.borrow_mut().define_top_level(name, value),
        }

    }

    pub fn define(&mut self, name: String, value: LiteralValueAst) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &str) -> Option<LiteralValueAst> {
        let value: Option<&LiteralValueAst> = self.values.get(name);

        match (value, &self.enclosing) {
            (Some(val), _) => Some(val.clone()),
            (None, Some(env)) => env.borrow().get(name),
            (None, None) => self.globals.get(name).cloned(),
        }
    }

    pub fn assign(&mut self, name: &str, value: LiteralValueAst) -> bool {
        let old_value: Option<&LiteralValueAst> = self.values.get(name);

        match (old_value, &self.enclosing) {
            (Some(_), _) => {
                self.values.insert(name.to_string(), value);
                true
            }
            (None, Some(env)) => (env.borrow_mut()).assign(name, value),
            (None, None) => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn try_init() {
        let _environment: Environment = Environment::new();
    }
}
