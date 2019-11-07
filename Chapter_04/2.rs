use std::io::{self, Write};

pub fn main() {
    let mut name = String::new();

    print!("Input your name: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut name).unwrap();

    let width = name.len() + 3;

    println!("{name:=>width$}!",
             name=name.trim(),
             width=width);
}
