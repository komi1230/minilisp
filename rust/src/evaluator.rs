fn is_lisp_string(token: String) -> bool {
    let length = token.len();
    let cs = token.chars();
    if cs.nth(0).unwrap() == "\"" && cs.nth(length-1).unwrap() == "\"" {
        for i in 1..(length-1) {
            if cs.nth(i).unwrap() == "\"" {
                return false
            }
        }
    }
    true
}


pub fn eval(tokens: &mut VecDeque<String>) -> String {

    // Get head token
    let token = tokens.pop_front().unwrap();

    // Include list
    if token == "(" {
        let mut list = VecDeque::new();
        while tokens.len() > 0 && tokens[0] != ")" {
            list.push_back(tokens.pop_front());
        }
        return eval(list);
    }

    // String
    if is_lisp_string(token) {
        return token;
    }
    
}

