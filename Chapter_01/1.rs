use std::io::{self, Write};

pub fn main() {
    let mut inch = String::new();

    print!("Please input the inches: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut inch).unwrap();

    let inch: f64 = inch.trim().parse().unwrap();
    
    let cm = inch * 2.54;

    println!("{:.2} cm\n", cm)
}
