use std::io::{self, Write};

const H_P_M: i32 = 60;

pub fn main() {
    let mut inputs = String::new();

    print!("Enter the number of minutes: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut inputs).unwrap();

    let mut minutes = inputs.trim().parse::<i32>().unwrap();

    while minutes > 0 {
        inputs.clear();

        let hour = minutes / H_P_M;
        let left = minutes % H_P_M;

        println!("{} minutes is {} hours and {} minutes.\n",
                 minutes, hour, left);

        print!("Enter your next value: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut inputs).unwrap();

        minutes = inputs.trim().parse::<i32>().expect("unable to parse input");
    }

    println!("Good bye!")
}
