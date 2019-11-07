use std::io::{self, Write};

pub fn main() {
    let mut days = String::new();

    print!("Input the number of days: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut days).unwrap();

    let mut days: i32 = days.trim().parse().unwrap();

    while days > 0 {
        let weeks = days / 7;
        let left = days % 7;
        println!("{} days are {} weak, {} days", days, weeks, left);

        let mut input = String::new();

        print!("Next input: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();

        days = input.trim().parse().unwrap();
    }
}
