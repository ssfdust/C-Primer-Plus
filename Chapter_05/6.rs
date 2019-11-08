use std::io::{self, Write};

pub fn main() {
    let mut inputs = String::new();
    let (mut count, mut sum): (i32, i32) = (0, 0);
    let days: i32;

    print!("Input the number of days: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut inputs).unwrap();

    days = inputs.trim().parse().unwrap();

    while count < days {
        count += 1;
        sum += count.pow(2);
    }

    println!("The money you earned: {}", sum);
}
