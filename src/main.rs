use std::io::{self, BufRead, Write};

use crate::lexer::Lexer;

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
        let token = lexer.next_token();
        println!("token: {:?}", token);
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
