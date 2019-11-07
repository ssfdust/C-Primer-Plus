pub fn main() {
    println!("Starting now...");
    one_three();
}

pub fn two() -> () {
    println!("two")
}

fn one_three() -> () {
    println!("one");
    two();
    println!("three");
    println!("done!")
}
