use std::io::{self, Write};

pub fn main() {
    let mut age = String::new();

    print!("Please input your age: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut age).unwrap();
    let age: f64 = age.trim().parse().unwrap();
    let seconds = age * 3.156e7;

    println!("the corresponding seconds are: {:+e}", seconds)
}
