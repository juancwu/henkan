use std::io::{self, BufRead, Write};

mod temperature;

fn main() {
    println!("Welcome to Henkan, an ergonomic unit converter in the terminal.");
    println!("Try entering 28c in f");
    print!("> ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("entered: {}", line.unwrap());
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
