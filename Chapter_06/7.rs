extern crate promptly;

use promptly::prompt;

pub fn main() {
    let word: String = prompt("Input a word");
    println!("{}", word.chars().rev().collect::<String>());
}
