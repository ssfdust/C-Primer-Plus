#[macro_use]
extern crate scan_fmt;
extern crate promptly;

use promptly::prompt;

pub fn main() {
    let mut inputs: String = prompt("Enter lower and upper integer limits");
    let mut results = scan_fmt!(&inputs, "{} {}", i32, i32).unwrap();

    while let (lower, upper) = results {
        let mut sum = 0;

        for num in lower..upper + 1 {
            sum += num.pow(2);
        }

        println!("The sums of the squares from {} to {} is {}",
                 lower.pow(2), upper.pow(2), sum);
        inputs = prompt("Input your next pair of numbers");
    }
}
