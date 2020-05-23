extern crate minilisp;

fn main() {
    println!("Hello, world!");

    let s = "   (+ 1 (- \"2.2\" 3))";
    println!("Before : {}", s);

    let mut ss = minilisp::parser::tokenize(s);
    println!("Tokens: {:?}", ss);

    let mut hoge = minilisp::environment::cdr(&mut ss);
    println!("{:?}", hoge);

    println!("{}", minilisp::parser::print(hoge));
}
