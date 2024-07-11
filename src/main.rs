// NOTE: `mod` declares the module available to the compiler.
mod eval;
mod lexer;
mod parser;
mod token;
mod util;

use std::io::{self, BufRead, Write};

use crate::{eval::eval_expression_ast, lexer::Lexer, parser::parse_expression, token::Token};

fn main() {
    println!("Welcome to Henkan, an ergonomic unit converter in the terminal.");
    print!("> ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        if l.len() > 0 {
            if l.chars().nth(0).eq(&Some('\\')) {
                println!("Command parsing not implemented.");
            } else {
                let mut lexer = Lexer::new(l);
                let mut tokens: Vec<Token> = vec![];
                let mut token: Token;
                while !lexer.is_end() {
                    token = lexer.next_token();
                    tokens.push(token.clone());
                    if token == Token::EOL {
                        break;
                    }
                }
                let ast = parse_expression(&tokens);
                if ast.is_some() {
                    println!("{:.10}", eval_expression_ast(&ast.unwrap()));
                } else {
                    println!("Invalid input.");
                }
            }
        }
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
