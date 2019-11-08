use std::io::{self, Write};

pub fn main() {
    let mut input = String::new();

    println!("This program computes moduli.\n");
    print!("Enter an integer to serve as the second operand: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let num1 = input.trim().parse::<i32>().unwrap();
    input.clear();

    print!("Now enter the first operand: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();
    let mut num2 = input.trim().parse::<i32>().unwrap();
    input.clear();

    while num2 > 0 {
        println!("{} % {} is {}", num2, num1, num2 % num1);

        print!("Enter next number for first operand (<= 0 to quit): ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
        num2 = input.trim().parse::<i32>().unwrap();
        input.clear();
    }
}
