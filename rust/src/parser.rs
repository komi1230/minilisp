use std::collections::VecDeque;

pub fn tokenize(s: &str) -> VecDeque<String> {
    // Spread each tokens
    let spreaded = s.replace("(", " ( ").replace(")", " ) ");

    // Split string to each tokens
    let tokens: VecDeque<String> = spreaded
        .trim()
        .split_whitespace()
        .map(|item| item.to_string())
        .collect();

    tokens
}

pub fn print(tokens: &mut VecDeque<String>) -> String {
    let mut s = "".to_string();
    for i in 0..(tokens.len()-1) {
        if tokens[i] == "(" || tokens[i+1] == ")" {
            s = s + &tokens[i];
        } else {
            s = s + &tokens[i] + " ";
        }
    }
    s = s + &tokens[tokens.len()-1];
    s
}
