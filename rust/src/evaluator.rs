use std::collections::VecDeque;

#[derive(Debug,Clone)]
struct Proc {
    params: VecDeque<minilisp::parser::Val>,
    body: minilisp::parser::Val,
    env: EnvRef,
}

impl Proc {
    fn new(params: Vec<Val>, body: Val, env: EnvRef) -> Proc {
        Proc {
            params: params,
            body: body,
            env: env,
        }
    }

    fn call(&self, args: Vec<Val>) -> Val {
        if args.len() != self.params.len() {
            panic!("incorrect number of args for func, expected {}, got {}", self.params.len(), args.len());
        } else {
            let mut local_env = Env::new(self.env.clone());
            for i in 0..self.params.len() {
                let param_name = match self.params[i] {
                    Val::Symbol(ref x) => x.clone(),
                    _ => panic!("param names must be symbols"),
                };
                local_env.define(&param_name, args[0].clone()); // TODO: optimise
            }
            let local_env_ref: EnvRef = Rc::new(Some(local_env));
            eval(self.body.clone(), local_env_ref)
        }
    }
}
