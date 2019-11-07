use std::io::{self, Write};

pub fn main() {
    let mut inches = String::new();

    print!("input your height(inch): ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut inches).unwrap();
    let inches: f64 = inches.trim().parse().unwrap();
    let cms = inches * 2.54;

    println!("your height(cm): {}", cms);
}
