#[macro_use]
extern crate scan_fmt;
extern crate promptly;

use promptly::prompt;

pub fn main() {
    let inputs: String = prompt("Input the min and max");
    let (min, max) = scan_fmt!(&inputs, "{} {}", i32, i32).unwrap();
    println!("{:>10} {:>10} {:>10}", "number", "square", "cube");
    for num in min..max {
        println!("{:>10}{:>10}{:>10}", num, num.pow(2), num.pow(3));
    }
}
