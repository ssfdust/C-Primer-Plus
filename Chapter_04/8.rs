use std::io::{self, Write};

const GALLON :f64 = 3.758;
const MILE :f64 = 1.609;

pub fn main() {
    let mut inputs = String::new();

    print!("Input miles and gallons: ");
    io::stdout().flush().ok();

    let (miles, gallon) = {
        io::stdin().read_line(&mut inputs).unwrap();
        let mut iter = inputs.split(" ");
        (iter.next().and_then(|x| x.trim().parse::<f64>().ok()).unwrap(),
         iter.next().and_then(|x| x.trim().parse::<f64>().ok()).unwrap())
    };
    println!("Miles per gallon: {:.1}\n",
             miles / gallon);
    println!("Litre per 100 kilometers: {:.1}",
            gallon * GALLON / (miles * MILE) * 100.0);
}
