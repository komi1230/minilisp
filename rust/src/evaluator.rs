use std::collections::VecDeque;
use std::rc::Rc;

use crate::environment::{self, Env, EnvRef};
use crate::parser::{self, Val};


#[derive(Debug,Clone)]
pub struct Proc {
    params: VecDeque<Val>,
    body: Val,
    env: EnvRef,
}

impl Proc {
    pub fn new(params: VecDeque<Val>, body: Val, env: EnvRef) -> Proc {
        Proc {
            params: params,
            body: body,
            env: env,
        }
    }

    pub fn call(&self, args: VecDeque<Val>) -> Val {
        if args.len() != self.params.len() {
            panic!("incorrect number of args for func, expected {}, got {}", self.params.len(), args.len());
        } else {
            let mut local_env = environment::Env::new(self.env.clone());
            for i in 0..self.params.len() {
                let param_name = match self.params[i] {
                    parser::Val::Symbol(ref x) => x.clone(),
                    _ => panic!("param names must be symbols"),
                };
                local_env.define(&param_name, args[0].clone()); // TODO: optimise
            }
            let mut local_env_ref: EnvRef = Rc::new(Some(local_env));
            eval(self.body.clone(), &mut local_env_ref)
        }
    }
}


pub fn eval(val: Val, env: &mut EnvRef) -> Val {
    // println!("eval {:?}", &val);
    let result = match val {
        Val::Symbol(x) => match Rc::try_unwrap(env.clone()) {
            Ok(v) => v.unwrap().access(&x),
            Err(_v) => Val::Symbol("Invalid input".to_string())
        },
        Val::Number(_) => val,
        Val::List(list) => {
            let mut args = list;
            let proc_name = args.remove(0);

            match proc_name.unwrap() {
                Val::Symbol(symbol) => {
                    match symbol.trim() {
                        "quote" => args.pop_front().unwrap(),
                        "if" => {
                            let test = args.pop_front().unwrap();
                            let conseq = args.pop_front().unwrap();
                            let alt = args.pop_front().unwrap();
                            let test_result = eval(test, env);
                            let exp = if !environment::is_false(test_result) { conseq } else { alt };
                            eval(exp, env)
                        },
                        // "lambda" => {
                        //     let params = match args.pop_front().unwrap() {
                        //         Val::List(x) => x,
                        //         _ => panic!("expected params to be a list"),
                        //     };
                        //     let body = args.pop_front().unwrap();
                        //     Val::Callable(Proc::new(params, body, env.clone()))
                        // },
                        "define" => {
                            let var = args.pop_front().unwrap();
                            let exp = args.pop_front().unwrap();
                            let var_name = match var {
                                Val::Symbol(ref x) => x,
                                _ => panic!("first arg to define must be a symbol"),
                            };
                            let exp_result = eval(exp, env);
                            match Rc::try_unwrap(env.clone()) {
                                Ok(v) => v.unwrap().define(&var_name, exp_result),
                                Err(v) => panic!("You cannot define function")
                            };
                            //env.define(&var_name, exp_result);
                            environment::symbol_true()
                        },
                        // otherwise, call procedure
                        _ => {
                            let evaluated_args: VecDeque<Val> = args.iter()
                                .map(|arg| eval(arg.clone(), env))
                                .collect();
                            call_proc(&symbol, evaluated_args)
                        },
                    }
                },
                _ => panic!("unknown list form"),
            }
        },
    };
    result
}


pub fn call_proc(proc_name: &String, mut args: VecDeque<Val>) -> Val {
    match proc_name.trim() {
        "+" => apply_arithmetic(args, environment::add),
        "-" => apply_arithmetic(args, environment::sub),
        "*" => apply_arithmetic(args, environment::mul),
        "/" => apply_arithmetic(args, environment::div),
        ">" => apply2(args, environment::gt),
        "<" => apply2(args, environment::lt),
        ">=" => apply2(args, environment::gte),
        "<=" => apply2(args, environment::lte),
        "=" => apply2(args, environment::eq),
        "not" => apply1(args, environment::not),
        "list" => Val::List(args),
        "begin" => {
            match args.pop_front() {
                Some(x) => x,
                None => environment::symbol_false(),
            }
        },
        _ => Val::Symbol("Invalid input".to_string()),
    }
}


fn apply_arithmetic<F: Fn(f64, f64) -> f64>(args: VecDeque<Val>, operator: F) -> Val {
    let mut accumulated: f64 = 0f64;
    for i in 0..args.len() {
        accumulated = match args[i] {
            Val::Number(operand) => {
                if i == 0 { operand } else { operator(accumulated, operand) }
            },
            _ => panic!("args to arithmetic functions must be Numbers"),
        };
    };
    Val::Number(accumulated)
}

fn apply1<F: Fn(Val) -> Val>(args: VecDeque<Val>, func: F) -> Val {
    if args.len() != 1 {
        panic!("incorrect number of args for func, expected 1, got {}", args.len());
    } else {
        func(args[0].clone())
    }
}

fn apply2<F: Fn(Val, Val) -> Val>(args: VecDeque<Val>, func: F) -> Val {
    if args.len() != 2 {
        panic!("incorrect number of args for func, expected 2, got {}", args.len());
    } else {
        func(args[0].clone(), args[1].clone())
    }
}
