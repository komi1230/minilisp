extern crate minilisp;

fn main() {
    println!("Hello, world!");

    let s = "   (+ 1 (- \"2.2\" 3))";
    println!("Before : {}", s);

    let mut ss = minilisp::parser::parse(s);
    println!("Tokens: {:?}", ss);
}
