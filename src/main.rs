use std::io::{self, BufRead, Write};

fn main() {
    println!("Welcome to Henkan, an ergonomic unit converter in the terminal.");
    print!("> ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("entered: {}", line.unwrap());
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
