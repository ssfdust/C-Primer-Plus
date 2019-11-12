#[macro_use]
extern crate text_io;
extern crate promptly;

use promptly::prompt;
use std::error::Error;

pub fn main() {
    let mut inputs: String = prompt("Input two numbers");
    let (mut n, mut m): (f64, f64) = (0.0, 0.0);
    loop {
        read_input(inputs, &mut n, &mut m).expect("Bye!");
        println!("{:.2}", ((n - m) / n * m));
        inputs = prompt("Input your next pair of numbers");
    }
}

fn read_input<'a>(inputs: String, mut n: &'a f64, mut m: &'a f64) -> Result<(&'a f64, &'a f64), Box<Error>> {
    try_scan!(inputs.bytes() => "{} {}", *n, *m);
    println!("{} {}", n, m);
    Ok((n, m))
}
