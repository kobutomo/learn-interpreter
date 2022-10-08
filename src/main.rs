mod lexer;
mod token;

fn main() {
    println!("Hello, world!");
    let input = "+++".to_string();
    let mut lexer = lexer::new(input);
    lexer.next_token();
}
