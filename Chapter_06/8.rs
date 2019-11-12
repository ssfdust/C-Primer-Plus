#[macro_use]
extern crate text_io;
extern crate promptly;

use promptly::prompt;
use std::error::Error;

pub fn main() {
    let mut inputs: String = prompt("Input two numbers");
    let (mut n, mut m): (f64, f64) = (0.0, 0.0);
    loop {
        match read_input(inputs, &mut n, &mut m) {
            Ok((n, m)) => (n, m),
            Err(_) => break,
        };
        println!("{:.2}", ((n - m) / n * m));
        inputs = prompt("Input your next pair of numbers");
    }
    println!("Bye");
}

fn read_input<'a>(
    inputs: String,
    n: &'a mut f64,
    m: &'a mut f64,
) -> Result<(&'a f64, &'a f64), Box<Error>> {
    scan!(inputs.bytes() => "{} {}", (*n), (*m));
    Ok((n, m))
}
