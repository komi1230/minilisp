use std::collections::VecDeque;

pub type EnvRef = Rc<Option<Env>>;

#[derive(Debug,Clone)]
pub struct Env {
    vars: HashMap<String, Val>,
    parent: EnvRef,
}

impl Env {
    fn new(parent: EnvRef) -> Env {
        Env {
            vars: HashMap::new(),
            parent: parent,
        }
    }

    fn access(&self, var_name: &String) -> Val {
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

    fn define(&mut self, var_name: &String, val: Val) {
        match self.vars.insert(var_name.to_owned(), val) {
            Some(_) => panic!("can't define variable '{}', already defined in this scope", var_name),
            None => (),
        }
    }

    fn assign(&mut self, var_name: &String, val: Val) {
        match self.vars.get_mut(var_name) {
            Some(x) => { *x = val; },
            None => panic!("can't assign to undefined variable '{}'", var_name),
        }
    }
}
