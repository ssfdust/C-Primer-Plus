use std::io::{self, Write};

pub fn main() {
    let mut cup = String::new();

    print!("input the number of cups: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut cup).unwrap();
    let cup: f64 = cup.trim().parse().unwrap();

    let pint = cup / 2.0;
    let ounce = cup * 8.0;
    let soupspoon = ounce * 2.0;
    let teaspoon = soupspoon * 3.0;

    println!("they are equivalent of:");
    println!("{} pint\n{} ounce\n{} soupspoons\n{} teaspoons",
             pint, ounce, soupspoon, teaspoon);
}
