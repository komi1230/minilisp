extern crate minilisp;

use std::io;
use std::io::prelude::*;
use std::collections::VecDeque;

use minilisp::environment::EnvRef;
use minilisp::parser::Val;


fn main() {
    run();
}

fn run() {
    let global_env = minilisp::environment::standard_env();
    read_eval_print_loop(global_env);
}


fn read_eval_print_loop(env: EnvRef) -> ! {
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin().read_line(&mut input)
            .ok()
            .expect("Failed to read line");

        read_eval_print(input.trim(), env.clone());
    }
}

fn read_eval_print(program: &str, env: EnvRef) {
    let program_result = minilisp::evaluator::eval(minilisp::parser::parse(program), env.clone());
    print!("=> ");
    print_val(&program_result);
}

fn print_val(val: &Val) {
    println!("{}", format_val(&val));
}

fn format_val(val: &Val) -> String {
    match *val {
        Val::List(ref x) => format_list(&x),
        Val::Number(ref x) => format!("{}", x),
        Val::Symbol(ref x) => format!("{}", x),
    }
}

fn format_list(list: &VecDeque<Val>) -> String {
    let formatted_items: Vec<String> = list.iter().map(|item| format_val(&item)).collect();

    format!("({})", formatted_items.join(" "))
}