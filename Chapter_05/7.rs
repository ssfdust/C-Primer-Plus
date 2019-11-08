use std::io::{self, Write};

pub fn main() {
    let mut num = String::new();

    print!("Input a number: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut num).unwrap();

    let num = num.trim().parse::<f64>().unwrap();

    cube(num);
}

fn cube(num: f64) -> () {
    println!("The cube of {} is {}", num, num.powf(3.0));
}
