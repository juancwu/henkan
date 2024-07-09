use std::io::{self, BufRead, Write};

fn main() {
    print!("> ");
    io::stdout().flush().unwrap();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        println!("entered: {}", line.unwrap());
        print!("> ");
        io::stdout().flush().unwrap();
    }
}
