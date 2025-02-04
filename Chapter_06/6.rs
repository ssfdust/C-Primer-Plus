#[macro_use]
extern crate text_io;
extern crate promptly;

use promptly::prompt;

pub fn main() {
    let inputs: String = prompt("Input the min and max");
    let (min, max): (i32, i32);
    scan!(inputs.bytes() => "{} {}", min, max);
    println!("{:>10} {:>10} {:>10}", "number", "square", "cube");
    for num in min..max {
        println!("{:>10}{:>10}{:>10}", num, num.pow(2), num.pow(3));
    }
}
