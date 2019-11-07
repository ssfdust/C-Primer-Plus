use std::io::{self, Write};

const CM_PER_FEET: f64 = 30.48 //1feet=30.48cm
const CM_PER_INCH: f64 = 2.54  //1inch=2.54cm

pub fn main() {
    print!("Enter a height in centimeters: ");
    io::stdout().flush().unwrap();
}
