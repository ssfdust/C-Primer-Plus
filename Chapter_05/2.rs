use std::io::{self, Write};

pub fn main() {
    let mut num = String::new();
    let mut cnt = 0;

    print!("Input a integer: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut num).unwrap();

    let mut num: i32 = num.trim().parse().unwrap();

    while cnt < 10 {
        cnt += 1;
        num += 1;
        print!("{} ", num);
    }
    println!();
}
