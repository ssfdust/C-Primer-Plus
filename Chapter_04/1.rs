use std::io::{self, Write};

pub fn main() {
    let (mut first_name, mut last_name) = (String::new(), String::new());

    print!("Input your first name: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut first_name).unwrap();

    print!("Input your last name: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut last_name).unwrap();

    println!("Your name is {} {}",
             first_name.trim(), last_name.trim())
}
