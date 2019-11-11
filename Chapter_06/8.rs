#[macro_use]
extern crate scan_fmt;
extern crate promptly;

use promptly::prompt;

pub fn main() {
    let mut inputs: String = prompt("Input two numbers");
    loop {
        let (n, m): (f64, f64) = match scan_fmt!(&inputs, "{} {}", f64, f64){
            Ok((n, m)) => (n, m),
            Err(_) => break
        };
        println!("{:.2}", ((n - m) / n * m));
        inputs = prompt("Input your next pair of numbers");
    }
    println!("Bye!")
}
