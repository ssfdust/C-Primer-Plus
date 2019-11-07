use std::io::{self, Write};

pub fn main() {
    let mut input = String::new();

    print!("Enter a floating-point value: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    let input: f64 = input.trim().parse().unwrap();

    println!("fixed-point notation: {}\n", input);
    println!("exponential notation: {:+e}\n", input);
}
