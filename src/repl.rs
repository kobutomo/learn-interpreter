use std::io::{self, Write};

use crate::{lexer, token};

pub fn start() -> ! {
    loop {
        let mut line = String::with_capacity(1000);
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line.");
        let mut l = lexer::new(line);

        loop {
            let tok = l.next_token();
            if tok.token_type == token::TokenType::Special(token::Special::EOF) {
                break;
            } else {
                println!("{:?}", tok)
            }
        }
    }
}
