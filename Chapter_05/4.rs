use std::io::{self, Write};

const CM_PER_FEET: f64 = 30.48; //1feet=30.48cm
const CM_PER_INCH: f64 = 2.54;  //1inch=2.54cm

pub fn main() {
    let mut cm = String::new();

    print!("Enter a height in centimeters: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut cm).unwrap();
    let mut cm: f64 = cm.trim().parse().unwrap();

    while cm > 0.0 {
        let feet = (cm / CM_PER_FEET) as i32;
        let inch = (cm - feet as f64 * CM_PER_FEET) / CM_PER_INCH;
        println!("{:.1} cm = {} feet, {:.1} inches",cm,feet,inch);
        print!("Enter a height in centimeters (<=0 to quit): ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        cm = input.trim().parse().unwrap();
    }
}
