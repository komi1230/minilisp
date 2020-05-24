use std::rc::Rc;
use std::f64::consts;
use std::collections::HashMap;

use crate::parser::Val;


pub type EnvRef = Rc<Option<Env>>;

#[derive(Debug,Clone)]
pub struct Env {
    vars: HashMap<String, Val>,
    parent: EnvRef,
}

impl Env {
    pub fn new(parent: EnvRef) -> Env {
        Env {
            vars: HashMap::new(),
            parent: parent,
        }
    }

    pub fn access(&self, var_name: &String) -> Val {
        match self.vars.get(var_name) {
            Some(x) => x.clone(),
            None => {
                match *self.parent {
                    Some(ref parent) => parent.access(&var_name),
                    None => panic!("can't access undefined variable '{}'", var_name),
                }
            },
        }
    }

    pub fn define(&mut self, var_name: &String, val: Val) {
        match self.vars.insert(var_name.to_owned(), val) {
            Some(_) => panic!("can't define variable '{}', already defined in this scope", var_name),
            None => (),
        }
    }

    pub fn assign(&mut self, var_name: &String, val: Val) {
        match self.vars.get_mut(var_name) {
            Some(x) => { *x = val; },
            None => panic!("can't assign to undefined variable '{}'", var_name),
        }
    }
}


pub fn standard_env() -> EnvRef {
    let null_env_ref: EnvRef = Rc::new(None);
    let mut env = Env::new(null_env_ref);
    env.define(&("pi".to_string()), Val::Number(consts::PI));
    Rc::new(Some(env))
}


pub fn symbol_true() -> Val {
    Val::Symbol("#t".to_string())
}


pub fn symbol_false() -> Val {
    Val::Symbol("#f".to_string())
}

fn bool_to_symbol(x: bool) -> Val {
    if x { symbol_true() } else { symbol_false() }
}

fn extract_number(val: Val) -> f64 {
    match val {
        Val::Number(x) => x,
        _ => panic!("expected a Number"),
    }
}

fn extract_symbol(val: Val) -> String {
    match val {
        Val::Symbol(x) => x,
        _ => panic!("expected a Symbol"),
    }
}


pub fn gt(a: Val, b: Val) -> Val {
    bool_to_symbol(extract_number(a) > extract_number(b))
}

pub fn lt(a: Val, b: Val) -> Val {
    bool_to_symbol(extract_number(a) < extract_number(b))
}

pub fn gte(a: Val, b: Val) -> Val {
    bool_to_symbol(extract_number(a) >= extract_number(b))
}

pub fn lte(a: Val, b: Val) -> Val {
    bool_to_symbol(extract_number(a) <= extract_number(b))
}

pub fn eq(a: Val, b: Val) -> Val {
    match a {
        Val::Symbol(_) => bool_to_symbol(extract_symbol(a) == extract_symbol(b)),
        Val::Number(_) => bool_to_symbol(extract_number(a) == extract_number(b)),
        Val::List(_) => panic!("equality operator not implemented for List :("),
    }
}

pub fn is_false(val: Val) -> bool {
    match val {
        Val::Symbol(x) => x == "#f",
        _ => false,
    }
}

pub fn not(val: Val) -> Val {
    let boolean_value = !is_false(val);
    bool_to_symbol(!boolean_value)
}


pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn sub(a: f64, b: f64) -> f64 {
    a - b
}

pub fn mul(a: f64, b: f64) -> f64 {
    a * b
}

pub fn div(a: f64, b: f64) -> f64 {
    a / b
}
