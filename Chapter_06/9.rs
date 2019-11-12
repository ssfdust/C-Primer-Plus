#[macro_use]
extern crate text_io;

use std::io::{self, Write};

pub fn main() {
    let mut inputs: String;
    print!("Input two numbers");
    io::stdout().flush().unwrap();
    let (mut n, mut m): (f64, f64);
    loop {
        scan!("{} {}", n, m);
        println!("{:.2}", calculate(n, m));
        print!("Input your next pair of numbers");
        io::stdout().flush().unwrap();
    }
    println!("Bye!")
}

fn calculate(n: f64, m: f64) -> f64 {
    (n - m) / (n * m)
}
