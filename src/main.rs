// NOTE: `mod` declares the module available to the compiler.
mod lexer;
mod parser;
mod token;

use std::io::{self, BufRead, Write};

use crate::{
    lexer::Lexer,
    parser::{parse_expression, ASTNode},
    token::Token,
};

fn main() {
    println!("Welcome to Henkan, an ergonomic unit converter in the terminal.");
    print!("> ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let l = line.unwrap();
        let mut lexer = Lexer::new(l);
        let mut tokens: Vec<Token> = vec![];
        while !lexer.is_end() {
            let token = lexer.next_token();
            tokens.push(token);
            if token == Token::EOL {
                break;
            }
            println!("token: {:?}", token);
        }
        let ast = parse_expression(&tokens);
        if ast.is_some() {
            let node = ast.unwrap();
            match node {
                ASTNode::Expr {
                    value,
                    value_unit,
                    operator,
                    unit,
                } => {
                    let n = match value {
                        Token::Number(v) => v,
                        _ => 0.0,
                    };
                    let n_u = match value_unit {
                        Token::Centimeter => "cm",
                        Token::Millimeter => "mm",
                        Token::Meter => "m",
                        Token::Kilometer => "km",
                        Token::Celcius => "Celsius",
                        Token::Fahrenheit => "Fahrenheit",
                        _ => "",
                    };
                    let op = match operator {
                        Token::To => "to",
                        Token::As => "as",
                        Token::In => "in",
                        _ => "",
                    };
                    let u = match unit {
                        Token::Centimeter => "cm",
                        Token::Millimeter => "mm",
                        Token::Meter => "m",
                        Token::Kilometer => "km",
                        Token::Celcius => "Celsius",
                        Token::Fahrenheit => "Fahrenheit",
                        _ => "",
                    };
                    println!("ast: {}{} {} {}", n, n_u, op, u);
                }
            }
        }
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
