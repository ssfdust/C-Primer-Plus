#[macro_use]
extern crate text_io;
extern crate promptly;

use promptly::prompt;
use std::error::Error;

pub fn main() {
    let mut inputs: String = prompt("Enter lower and upper integer limits");
    let (mut lower, mut upper): (i32, i32);
    let get_input = |inputs: String, lower: &mut i32, upper: &mut i32| -> Result<(& i32, & i32), Box<Error>> {
        scan!(inputs.bytes() => "{} {}", (*lower), (*upper));
        Ok((lower, upper))
    };

/*     while true {
 *         let mut sum = 0;
 *
 *         for num in lower..upper + 1 {
 *             sum += num.pow(2);
 *         }
 *
 *         println!("The sums of the squares from {} to {} is {}",
 *                  lower.pow(2), upper.pow(2), sum);
 *         inputs = prompt("Input your next pair of numbers");
 *     } */
}
