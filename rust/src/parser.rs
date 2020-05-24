use std::collections::VecDeque;

use crate::evaluator::Proc;


#[derive(Debug,Clone)]
pub enum Val {
    List(VecDeque<Val>),
    Number(f64),
    Symbol(String),
    Callable(Proc),
}


fn tokenize(s: &str) -> VecDeque<String> {
    // Spread each tokens
    s.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|item| item.to_string())
        .collect()
}


pub fn parse(program: &str) -> Val {
    let mut tokens = tokenize(program);
    read_from_tokens(&mut tokens)
}


fn read_from_tokens(tokens: &mut VecDeque<String>) -> Val {
    if tokens.len() == 0 {
        panic!("unexpected EOF while reading");
    }
    let token = tokens.pop_front().unwrap();
    // println!("reading token: '{}'", token);
    if "(".to_string() == token {
        let mut list: VecDeque<Val> = VecDeque::new();
        while tokens[0] != ")".to_string() {
            list.push_back(read_from_tokens(tokens));
        }
        Val::List(list)
    } else if ")".to_string() == token {
        panic!("unexpected ')' while reading");
    } else {
        atom(token)
    }
}


fn atom(token: String) -> Val {
    // TODO: separate int/float types?
    let number = token.parse::<f64>();
    match number {
        Ok(x) => Val::Number(x),
        Err(_) => Val::Symbol(token),
    }
}
