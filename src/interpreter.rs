use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{environment::Environment, generate_ast::{Expr, LiteralValueAst}, stmt::Stmt, Token};

#[derive(Debug, Clone)]
pub enum ControlFlow {
    Break,
    None
}

pub struct Interpreter {
    pub specials: Rc<RefCell<Environment>>,
    pub environment: Rc<RefCell<Environment>>,
    pub locals: Rc<RefCell<HashMap<usize, usize>>>
}

impl Interpreter {
    pub fn new() -> Self {

        Self {
            specials: Rc::new(RefCell::new(Environment::new())),
            environment: Rc::new(RefCell::new(Environment::new())),
            locals: Rc::new(RefCell::new(HashMap::new()))
        }
    }
    
    pub fn for_closure(parent: Rc<RefCell<Environment>>) -> Self {
        let environment: Rc<RefCell<Environment>> = Rc::new(RefCell::new(Environment::new()));
        environment.borrow_mut().enclosing = Some(parent);

        Self { 
            specials: Rc::new(RefCell::new(Environment::new())),
            environment,
            locals: Rc::new(RefCell::new(HashMap::new()))
        }
    }

    pub fn for_anon(parent: Rc<RefCell<Environment>>) -> Self {
        let mut env: Environment = Environment::new();
        env.enclosing = Some(parent.clone());
        Self {
            specials: Rc::new(RefCell::new(Environment::new())),
            environment: Rc::new(RefCell::new(env)),
            locals: Rc::new(RefCell::new(HashMap::new()))
        }
    }

    pub fn resolve(&mut self, expr: &Expr, steps: usize) -> Result<(), String> {
        let addr: usize = std::ptr::addr_of!(expr) as usize;
        self.locals.borrow_mut().insert(addr, steps);
        Ok(())
    }

    fn get_distance(&self, expr: &Expr) -> Option<usize> {
        let addr = std::ptr::addr_of!(expr) as usize;
        self.locals.borrow().get(&addr).copied()
    }

    pub fn interpret(&mut self, stmts: Vec<&Stmt>) -> Result<ControlFlow, String> {
        for stmt in stmts {
            let result = match stmt {
                Stmt::Expression { expression } => {
                    let distance = self.get_distance(&expression);
                    expression.evaluate(self.environment.clone())?;
                    Ok(ControlFlow::None)
                }
                Stmt::Print { expression } => {
                    let value: LiteralValueAst = expression.evaluate(self.environment.clone())?;
                    println!("{}", value.to_string());
                    Ok(ControlFlow::None)
                }
                Stmt::Let { name, initializer } => {
                    let value: LiteralValueAst = initializer.evaluate(self.environment.clone())?;

                    self.environment
                        .borrow_mut()
                        .define(name.lexeme.clone(), value);

                    Ok(ControlFlow::None)
                }
                Stmt::Block { statements } => {
                    let mut new_environment: Environment = Environment::new();
                    new_environment.enclosing = Some(self.environment.clone());
                    let old_environment: Rc<RefCell<Environment>> = self.environment.clone();
                    self.environment = Rc::new(RefCell::new(new_environment));
        
                    let block_result: Result<ControlFlow, String> =
                        self.interpret((*statements).iter().map(|b| b.as_ref()).collect());
        
                    self.environment = old_environment;
        
                    block_result
                }
                Stmt::IfStmt { predicate, then, els } => {
                    let truth_value: LiteralValueAst = predicate.evaluate(self.environment.clone())?;
                    if truth_value.is_truthy() == LiteralValueAst::True {
                        self.interpret(vec![then.as_ref()])
                    } else if let Some(els_stmt) = els {
                        self.interpret(vec![els_stmt.as_ref()])
                    } else {
                        Ok(ControlFlow::None)
                    }
                }
                Stmt::WhileStmt { condition, body } => {
                    while condition.evaluate(self.environment.clone())?.is_truthy() == LiteralValueAst::True {
                        let result: ControlFlow = self.interpret(vec![body.as_ref()])?;
                        if let ControlFlow::Break = result {
                            break;
                        }
                    }
                    Ok(ControlFlow::None)
                }
                Stmt::BreakStmt => {
                    Ok(ControlFlow::Break)
                }
                Stmt::Function { name, params, body } => {
                    let arity: usize = params.len();

                    let params: Vec<Token> = params.iter().map(|t| (*t).clone()).collect();
                    let body: Vec<Box<Stmt>> = body.iter().map(|b| (*b).clone()).collect();
                    let name_clone: String = name.lexeme.clone();

                    let parent_env: Rc<RefCell<Environment>> = self.environment.clone();
                    let fun_impl = move |_env: Rc<RefCell<Environment>>, args: &Vec<LiteralValueAst>| {
                        let mut clos_int: Interpreter = Interpreter::for_closure(parent_env.clone());

                        for (i, arg) in args.iter().enumerate() {
                            clos_int
                                .environment
                                .borrow_mut()
                                .define(params[i].lexeme.clone(), (*arg).clone());
                        }

                        for i in 0..(body.len()) {
                            clos_int.interpret(vec![body[i].as_ref()]).expect(&format!(
                                "Evaluating failed inside {}",
                                name_clone
                            ));

                            if let Some(value) = clos_int.specials.borrow().get("return") {
                                return value
                            }
                        }

                        LiteralValueAst::Null
                    };

                    let callable: LiteralValueAst = LiteralValueAst::Callable {
                        name: name.lexeme.clone(),
                        arity,
                        fun: Rc::new(fun_impl),
                    };

                    self.environment
                        .borrow_mut()
                        .define(name.lexeme.clone(), callable);


                    Ok(ControlFlow::None)
                },
                Stmt::ReturnStmt { keyword: _, value } => {

                    let eval_val: LiteralValueAst ;
                    if let Some(value) = value {
                        eval_val = value.evaluate(self.environment.clone())?;
                    } else {
                        eval_val = LiteralValueAst::Null;
                    }
                    self.specials
                        .borrow_mut()
                        .define_top_level("return".to_string(), eval_val);
                    Ok(ControlFlow::None)
                }
            };
    
            if let Ok(ControlFlow::Break) = result {
                return Ok(ControlFlow::Break);
            } else if result.is_err() {
                return result;
            }
        }
    
        Ok(ControlFlow::None)
    }

}

