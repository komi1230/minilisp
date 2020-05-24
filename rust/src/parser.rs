use std::collections::VecDeque;

#[derive(Debug,Clone)]
pub enum Val {
    List(VecDeque<Val>),
    Number(f64),
    Symbol(String),
}



pub fn tokenize(s: &str) -> VecDeque<String> {
    // Spread each tokens
    s.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|item| item.to_string())
        .collect();
}


pub fn parse(program: &str) -> Val {
    let mut tokens = tokenize(program);
    read_from_tokens(&mut tokens)
}


fn read_from_tokens(tokens: &mut Vec<String>) -> Val {
    if tokens.len() == 0 {
        panic!("unexpected EOF while reading");
    }
    let token = tokens.remove(0);
    // println!("reading token: '{}'", token);
    if "(".to_string() == token {
        let mut list: Vec<Val> = Vec::new();
        while tokens[0] != ")".to_string() {
            list.push(read_from_tokens(tokens));
        }
        tokens.remove(0); // pop off ")"
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


fn symbol_true() -> Val {
    Val::Symbol("#t".to_string())
}


fn symbol_false() -> Val {
    Val::Symbol("#f".to_string())
}

