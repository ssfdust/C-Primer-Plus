use std::io::{self, Write};

pub fn main() {
    let mut input = String::new();
    let mut fah = Some(0.0);

    print!("Input the Fahrenheit temperature: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    while let Some(_i) = fah{
        let tmp = match input.trim().parse::<f64>(){
            Ok(fah) => fah,
            Err(_) => panic!("failed to parse input {}.", input.trim())
        };
        input.clear();

        fah = Some(tmp);
        temperature(tmp);
        print!("Next input: ");
        io::stdout().flush().unwrap();

        io::stdin().read_line(&mut input).unwrap();
    }
}

fn temperature(fah: f64) -> () {
    let (a, b, c, d): (f64, f64, f64, f64) = (5.0, 9.0, 32.0, 276.13);
    println!("{} ℉ is {} ℃, {} K.\n",
            fah, a / b * (fah - c),
            a / b * (fah - c) + d);
}
