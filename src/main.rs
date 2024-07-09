use std::io::{self, BufRead, Write};

use crate::{lexer::Lexer, token::Token};

// NOTE: `mod` declares the module available to the compiler.
mod lexer;
mod token;

fn main() {
    println!("Welcome to Henkan, an ergonomic unit converter in the terminal.");
    print!("> ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let mut lexer = Lexer::new(l);
        let mut token: Token;
        while !lexer.is_end() {
            token = lexer.next_token();
            if token == Token::EOL {
                break;
            }
            println!("token: {:?}", token);
        }
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
