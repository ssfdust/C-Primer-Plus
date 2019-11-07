use std::io::{self, Write};

pub fn main() {
    let mut ch = String::new();

    print!("please input a number: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut ch).unwrap();
    println!("{}", ch);
}
